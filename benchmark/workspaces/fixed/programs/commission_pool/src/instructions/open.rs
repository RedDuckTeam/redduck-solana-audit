use anchor_lang::prelude::*;

use crate::state::{Pool, Stake};

#[derive(Accounts)]
pub struct OpenStake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = owner,
        space = Stake::SPACE,
        seeds = [Stake::SEED, pool.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub stake: Account<'info, Stake>,

    pub system_program: Program<'info, System>,
}

pub fn open_stake(ctx: Context<OpenStake>) -> Result<()> {
    let stake = &mut ctx.accounts.stake;
    stake.pool = ctx.accounts.pool.key();
    stake.owner = ctx.accounts.owner.key();
    stake.staked = 0;
    stake.reward_debt = ctx.accounts.pool.accrued(0).unwrap_or(0);
    stake.unlock_ts = 0;
    stake.bump = ctx.bumps.stake;
    Ok(())
}
