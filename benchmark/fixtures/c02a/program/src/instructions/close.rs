use anchor_lang::prelude::*;

use crate::errors::PoolError;
use crate::state::{Pool, Stake};

#[derive(Accounts)]
pub struct CloseStake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(seeds = [Pool::SEED, pool.stake_mint.as_ref()], bump = pool.bump)]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        close = owner,
        seeds = [Stake::SEED, pool.key().as_ref(), owner.key().as_ref()],
        bump = stake.bump,
        has_one = owner,
        has_one = pool,
    )]
    pub stake: Account<'info, Stake>,
}

pub fn close_stake(ctx: Context<CloseStake>) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let stake = &ctx.accounts.stake;
    require!(stake.staked == 0, PoolError::StakeNotEmpty);
    let accrued = pool.accrued(stake.staked).ok_or(PoolError::MathOverflow)?;
    require!(accrued == stake.reward_debt, PoolError::StakeNotEmpty);
    Ok(())
}
