use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use super::pay_from_vault;
use crate::errors::LockerError;
use crate::events::{PositionClosed, RewardClaimed};
use crate::math::pro_rata;
use crate::state::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub season: Account<'info, Season>,
    #[account(mut, has_one = owner, has_one = season, close = owner)]
    pub position: Account<'info, Position>,
    #[account(mut, seeds = [STAKE_VAULT_SEED, season.key().as_ref()], bump = season.stake_vault_bump)]
    pub stake_vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = season.stake_mint, token::authority = owner)]
    pub owner_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Returns the locked tokens once the lock has run out and closes the position.
pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let position = &ctx.accounts.position;
    require!(now >= position.lock_end, LockerError::StillLocked);

    let season = &mut ctx.accounts.season;
    if season.finalized {
        require!(position.reward_claimed, LockerError::ClaimFirst);
    } else {
        season.remove_weight(position.weight)?;
    }

    let event = PositionClosed { position: position.key(), owner: position.owner, amount: position.amount };
    let a = &ctx.accounts;
    pay_from_vault(&a.token_program, &a.season, &a.stake_vault, &a.owner_token, position.amount)?;
    emit!(event);
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    pub owner: Signer<'info>,
    pub season: Account<'info, Season>,
    #[account(mut, has_one = owner, has_one = season)]
    pub position: Account<'info, Position>,
    #[account(mut, seeds = [REWARD_VAULT_SEED, season.key().as_ref()], bump = season.reward_vault_bump)]
    pub reward_vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = season.stake_mint, token::authority = owner)]
    pub owner_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Pays the position its share of the season reward.
pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
    let season = &ctx.accounts.season;
    require!(season.finalized, LockerError::NotFinalized);

    let position = &mut ctx.accounts.position;
    require!(!position.reward_claimed, LockerError::AlreadyClaimed);

    let amount = pro_rata(season.reward_amount, position.weight, season.final_total_weight)?;
    position.reward_claimed = true;
    let event = RewardClaimed { position: position.key(), owner: position.owner, amount };

    if amount > 0 {
        let a = &ctx.accounts;
        pay_from_vault(&a.token_program, season, &a.reward_vault, &a.owner_token, amount)?;
    }
    emit!(event);
    Ok(())
}
