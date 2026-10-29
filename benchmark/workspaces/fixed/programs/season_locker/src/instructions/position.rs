use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use super::deposit_to_vault;
use crate::errors::LockerError;
use crate::events::PositionUpdated;
use crate::math::lock_weight;
use crate::state::*;

#[derive(Accounts)]
#[instruction(position_id: u64)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub season: Account<'info, Season>,
    #[account(init, payer = owner, space = 8 + Position::INIT_SPACE,
        seeds = [POSITION_SEED, season.key().as_ref(), owner.key().as_ref(), &position_id.to_le_bytes()], bump)]
    pub position: Account<'info, Position>,
    #[account(mut, seeds = [STAKE_VAULT_SEED, season.key().as_ref()], bump = season.stake_vault_bump)]
    pub stake_vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = season.stake_mint, token::authority = owner)]
    pub owner_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn open_position(ctx: Context<OpenPosition>, position_id: u64, amount: u64, lock_secs: i64) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(amount > 0, LockerError::ZeroAmount);
    require!((MIN_LOCK_SECS..=MAX_LOCK_SECS).contains(&lock_secs), LockerError::InvalidLockDuration);

    let season_key = ctx.accounts.season.key();
    let season = &mut ctx.accounts.season;
    season.require_enrollment(now)?;

    let lock_end = now.checked_add(lock_secs).ok_or(LockerError::MathOverflow)?;
    let weight = lock_weight(amount, lock_end, now)?;
    season.add_weight(weight)?;

    let position = &mut ctx.accounts.position;
    position.season = season_key;
    position.owner = ctx.accounts.owner.key();
    position.id = position_id;
    position.amount = amount;
    position.lock_end = lock_end;
    position.weight = weight;
    position.reward_claimed = false;
    position.bump = ctx.bumps.position;
    let event = updated(position);

    let a = &ctx.accounts;
    deposit_to_vault(&a.token_program, &a.owner_token, &a.stake_vault, &a.owner, amount)?;
    emit!(event);
    Ok(())
}

#[derive(Accounts)]
pub struct AddStake<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub season: Account<'info, Season>,
    #[account(mut, has_one = owner, has_one = season)]
    pub position: Account<'info, Position>,
    #[account(mut, seeds = [STAKE_VAULT_SEED, season.key().as_ref()], bump = season.stake_vault_bump)]
    pub stake_vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = season.stake_mint, token::authority = owner)]
    pub owner_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Adds tokens to a position under its current lock end.
pub fn add_stake(ctx: Context<AddStake>, amount: u64) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(amount > 0, LockerError::ZeroAmount);

    let season = &mut ctx.accounts.season;
    season.require_enrollment(now)?;

    let position = &mut ctx.accounts.position;
    let added = lock_weight(amount, position.lock_end, now)?;
    position.amount = position.amount.checked_add(amount).ok_or(LockerError::MathOverflow)?;
    position.weight = position.weight.checked_add(added).ok_or(LockerError::MathOverflow)?;
    season.add_weight(added)?;
    let event = updated(position);

    let a = &ctx.accounts;
    deposit_to_vault(&a.token_program, &a.owner_token, &a.stake_vault, &a.owner, amount)?;
    emit!(event);
    Ok(())
}

#[derive(Accounts)]
pub struct ExtendLock<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub season: Account<'info, Season>,
    #[account(mut, has_one = owner, has_one = season)]
    pub position: Account<'info, Position>,
}

/// Pushes the lock end further out. Weight never drops as a result.
pub fn extend_lock(ctx: Context<ExtendLock>, new_lock_end: i64) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let season = &mut ctx.accounts.season;
    season.require_open(now)?;

    let position = &mut ctx.accounts.position;
    require!(new_lock_end > position.lock_end, LockerError::LockNotExtended);
    require!(new_lock_end.saturating_sub(now) <= MAX_LOCK_SECS, LockerError::InvalidLockDuration);

    let weight = lock_weight(position.amount, new_lock_end, now)?.max(position.weight);
    season.add_weight(weight - position.weight)?;
    position.lock_end = new_lock_end;
    position.weight = weight;

    emit!(updated(position));
    Ok(())
}

#[derive(Accounts)]
pub struct MergePositions<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub season: Account<'info, Season>,
    #[account(mut, has_one = owner, has_one = season, close = owner,
        constraint = source.key() != target.key() @ LockerError::SamePosition)]
    pub source: Account<'info, Position>,
    #[account(mut, has_one = owner, has_one = season)]
    pub target: Account<'info, Position>,
}

/// Folds `source` into `target`. The merged tokens share the later of the two lock ends.
pub fn merge_positions(ctx: Context<MergePositions>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    ctx.accounts.season.require_open(now)?;

    let source = &ctx.accounts.source;
    let target = &mut ctx.accounts.target;

    let amount = target.amount.checked_add(source.amount).ok_or(LockerError::MathOverflow)?;
    let lock_end = target.lock_end.max(source.lock_end);
    let combined = target.weight.checked_add(source.weight).ok_or(LockerError::MathOverflow)?;
    let weight = lock_weight(amount, lock_end, now)?.max(combined);

    target.amount = amount;
    target.lock_end = lock_end;
    target.weight = weight;
    ctx.accounts.season.add_weight(weight - combined)?;

    emit!(updated(target));
    Ok(())
}

fn updated(position: &Account<Position>) -> PositionUpdated {
    PositionUpdated {
        position: position.key(),
        owner: position.owner,
        amount: position.amount,
        lock_end: position.lock_end,
        weight: position.weight,
    }
}
