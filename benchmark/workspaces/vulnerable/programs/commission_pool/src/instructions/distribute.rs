use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::PoolError;
use crate::events::RewardsDistributed;
use crate::state::{Pool, ACC_SCALE};

#[derive(Accounts)]
pub struct Distribute<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [Pool::SEED, pool.stake_mint.as_ref()],
        bump = pool.bump,
        has_one = authority @ PoolError::NotAuthority,
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut, address = pool.reward_vault)]
    pub reward_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = pool.reward_mint, token::authority = authority)]
    pub source: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn distribute(ctx: Context<Distribute>, amount: u64) -> Result<()> {
    require!(amount > 0, PoolError::ZeroAmount);
    require!(ctx.accounts.pool.total_staked > 0, PoolError::EmptyPool);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source.to_account_info(),
                to: ctx.accounts.reward_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    let pool = &mut ctx.accounts.pool;
    let add = (amount as u128)
        .checked_mul(ACC_SCALE)
        .ok_or(PoolError::MathOverflow)?
        / pool.total_staked as u128;
    pool.acc_reward_per_share = pool
        .acc_reward_per_share
        .checked_add(add)
        .ok_or(PoolError::MathOverflow)?;
    emit!(RewardsDistributed {
        pool: pool.key(),
        amount,
        acc_reward_per_share: pool.acc_reward_per_share,
    });
    Ok(())
}
