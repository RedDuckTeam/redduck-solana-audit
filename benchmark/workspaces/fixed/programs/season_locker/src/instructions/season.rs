use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use super::deposit_to_vault;
use crate::errors::LockerError;
use crate::events::{SeasonCreated, SeasonFinalized};
use crate::state::*;

#[derive(Accounts)]
#[instruction(season_id: u64)]
pub struct CreateSeason<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub stake_mint: Account<'info, Mint>,
    #[account(init, payer = authority, space = 8 + Season::INIT_SPACE,
        seeds = [SEASON_SEED, authority.key().as_ref(), &season_id.to_le_bytes()], bump)]
    pub season: Account<'info, Season>,
    #[account(init, payer = authority, seeds = [STAKE_VAULT_SEED, season.key().as_ref()], bump,
        token::mint = stake_mint, token::authority = season)]
    pub stake_vault: Account<'info, TokenAccount>,
    #[account(init, payer = authority, seeds = [REWARD_VAULT_SEED, season.key().as_ref()], bump,
        token::mint = stake_mint, token::authority = season)]
    pub reward_vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = stake_mint, token::authority = authority)]
    pub authority_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// Opens a season funded with `reward_amount`. Positions enroll before `starts_at`;
/// lock changes are accepted until `ends_at`.
pub fn create_season(ctx: Context<CreateSeason>, season_id: u64, starts_at: i64, ends_at: i64, reward_amount: u64) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(now < starts_at && starts_at < ends_at, LockerError::InvalidSeasonWindow);
    require!(reward_amount > 0, LockerError::ZeroAmount);

    let season = &mut ctx.accounts.season;
    season.authority = ctx.accounts.authority.key();
    season.stake_mint = ctx.accounts.stake_mint.key();
    season.id = season_id;
    season.starts_at = starts_at;
    season.ends_at = ends_at;
    season.reward_amount = reward_amount;
    season.total_weight = 0;
    season.final_total_weight = 0;
    season.finalized = false;
    season.bump = ctx.bumps.season;
    season.stake_vault_bump = ctx.bumps.stake_vault;
    season.reward_vault_bump = ctx.bumps.reward_vault;

    let a = &ctx.accounts;
    deposit_to_vault(&a.token_program, &a.authority_token, &a.reward_vault, &a.authority, reward_amount)?;

    emit!(SeasonCreated { season: ctx.accounts.season.key(), starts_at, ends_at, reward_amount });
    Ok(())
}

#[derive(Accounts)]
pub struct FinalizeSeason<'info> {
    pub caller: Signer<'info>,
    #[account(mut)]
    pub season: Account<'info, Season>,
}

/// Records the season's total weight once it has ended. Anyone may call it.
pub fn finalize_season(ctx: Context<FinalizeSeason>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let season = &mut ctx.accounts.season;
    require!(now >= season.ends_at, LockerError::SeasonNotEnded);
    require!(!season.finalized, LockerError::AlreadyFinalized);

    season.final_total_weight = season.total_weight;
    season.finalized = true;

    emit!(SeasonFinalized { season: season.key(), final_total_weight: season.final_total_weight });
    Ok(())
}
