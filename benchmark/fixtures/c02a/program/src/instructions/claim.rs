use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::PoolError;
use crate::state::{Pool, Stake};

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    pub owner: Signer<'info>,

    #[account(seeds = [Pool::SEED, pool.stake_mint.as_ref()], bump = pool.bump)]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [Stake::SEED, pool.key().as_ref(), owner.key().as_ref()],
        bump = stake.bump,
        has_one = owner,
        has_one = pool,
    )]
    pub stake: Account<'info, Stake>,

    #[account(mut, address = pool.reward_vault)]
    pub reward_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = pool.reward_mint, token::authority = owner)]
    pub owner_reward: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn claim(ctx: Context<ClaimRewards>) -> Result<()> {
    let pool = &ctx.accounts.pool;
    let stake = &mut ctx.accounts.stake;

    let accrued = pool.accrued(stake.staked).ok_or(PoolError::MathOverflow)?;
    let pending = accrued.checked_sub(stake.reward_debt).ok_or(PoolError::MathOverflow)?;

    if pending > 0 {
        let payout = u64::try_from(pending).map_err(|_| PoolError::MathOverflow)?;
        let mint = pool.stake_mint;
        let seeds = pool.signer_seeds(&mint);
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.reward_vault.to_account_info(),
                    to: ctx.accounts.owner_reward.to_account_info(),
                    authority: pool.to_account_info(),
                },
                &[&seeds],
            ),
            payout,
        )?;
    }

    stake.reward_debt = accrued;
    Ok(())
}
