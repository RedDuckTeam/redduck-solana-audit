use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{Pool, PoolError, Staker};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, seeds = [b"pool"], bump = pool.bump)]
    pub pool: Account<'info, Pool>,
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + Staker::INIT_SPACE,
        seeds = [b"staker", owner.key().as_ref()],
        bump,
    )]
    pub staker: Account<'info, Staker>,
    #[account(address = pool.mint)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = owner)]
    pub owner_tokens: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.owner_tokens.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;
    let staker = &mut ctx.accounts.staker;
    staker.owner = ctx.accounts.owner.key();
    staker.bump = ctx.bumps.staker;
    staker.stake = staker
        .stake
        .checked_add(amount)
        .ok_or(PoolError::MathOverflow)?;
    let pool = &mut ctx.accounts.pool;
    pool.total_staked = pool
        .total_staked
        .checked_add(amount)
        .ok_or(PoolError::MathOverflow)?;
    Ok(())
}
