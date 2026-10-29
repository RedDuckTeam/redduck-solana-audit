use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::PoolError;
use crate::events::Deposited;
use crate::state::{Pool, Stake, WITHDRAW_LOCK_SECS};

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub owner: Signer<'info>,

    #[account(mut, seeds = [Pool::SEED, pool.stake_mint.as_ref()], bump = pool.bump)]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [Stake::SEED, pool.key().as_ref(), owner.key().as_ref()],
        bump = stake.bump,
        has_one = owner,
        has_one = pool,
    )]
    pub stake: Account<'info, Stake>,

    #[account(mut, address = pool.stake_vault)]
    pub stake_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = pool.stake_mint, token::authority = owner)]
    pub owner_stake: Account<'info, TokenAccount>,

    #[account(mut, address = pool.reward_vault)]
    pub reward_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = pool.reward_mint, token::authority = owner)]
    pub owner_reward: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, PoolError::ZeroAmount);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.owner_stake.to_account_info(),
                to: ctx.accounts.stake_vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    let pool = &mut ctx.accounts.pool;
    let stake = &mut ctx.accounts.stake;

    stake.staked = stake.staked.checked_add(amount).ok_or(PoolError::MathOverflow)?;
    pool.total_staked = pool.total_staked.checked_add(amount).ok_or(PoolError::MathOverflow)?;

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
    stake.unlock_ts = Clock::get()?.unix_timestamp + WITHDRAW_LOCK_SECS;
    emit!(Deposited {
        pool: ctx.accounts.pool.key(),
        owner: ctx.accounts.owner.key(),
        amount,
    });
    Ok(())
}
