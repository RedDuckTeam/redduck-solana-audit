use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{Pool, PoolError, Staker};

#[derive(Accounts)]
pub struct Unstake<'info> {
    pub owner: Signer<'info>,
    #[account(mut, seeds = [b"pool"], bump = pool.bump)]
    pub pool: Account<'info, Pool>,
    #[account(mut, seeds = [b"staker", owner.key().as_ref()], bump = staker.bump, has_one = owner)]
    pub staker: Account<'info, Staker>,
    #[account(address = pool.mint)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = owner)]
    pub owner_tokens: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    let staker = &mut ctx.accounts.staker;
    require!(staker.stake >= amount, PoolError::InsufficientStake);
    staker.stake -= amount;
    let pool = &mut ctx.accounts.pool;
    pool.total_staked = pool
        .total_staked
        .checked_sub(amount)
        .ok_or(PoolError::MathOverflow)?;
    let seeds: &[&[u8]] = &[b"pool", &[pool.bump]];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.owner_tokens.to_account_info(),
                authority: ctx.accounts.pool.to_account_info(),
            },
            &[seeds],
        ),
        amount,
        ctx.accounts.mint.decimals,
    )
}
