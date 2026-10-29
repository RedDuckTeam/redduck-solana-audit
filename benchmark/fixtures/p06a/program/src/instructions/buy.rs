use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{Config, RewardsError};

/// A user buys tokens from the vault with lamports at the stored price.
#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut, seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,
    #[account(address = config.mint)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = buyer)]
    pub buyer_tokens: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn buy(ctx: Context<Buy>, lamports: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    let now = Clock::get()?.unix_timestamp;
    require!(
        config.price > 0 && now - config.price_time <= config.max_price_age,
        RewardsError::StalePrice
    );
    let tokens = (lamports as u128)
        .checked_mul(10u128.pow(ctx.accounts.mint.decimals as u32))
        .ok_or(RewardsError::MathOverflow)?
        / config.price as u128;
    let tokens = u64::try_from(tokens).map_err(|_| RewardsError::MathOverflow)?;

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.config.to_account_info(),
            },
        ),
        lamports,
    )?;
    let seeds: &[&[u8]] = &[b"config", &[ctx.accounts.config.bump]];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.buyer_tokens.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[seeds],
        ),
        tokens,
        ctx.accounts.mint.decimals,
    )
}
