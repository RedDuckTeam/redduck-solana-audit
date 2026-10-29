use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::CrateError;
use crate::state::House;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [House::SEED, house.token_mint.as_ref()],
        bump = house.bump,
        has_one = authority @ CrateError::NotAuthority,
    )]
    pub house: Account<'info, House>,

    #[account(mut, address = house.vault)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = house.token_mint)]
    pub destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, CrateError::ZeroAmount);
    let free = ctx
        .accounts
        .vault
        .amount
        .checked_sub(ctx.accounts.house.reserved)
        .ok_or(CrateError::WouldUnderfund)?;
    require!(amount <= free, CrateError::WouldUnderfund);
    let mint = ctx.accounts.house.token_mint;
    let seeds = ctx.accounts.house.signer_seeds(&mint);
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.house.to_account_info(),
            },
            &[&seeds],
        ),
        amount,
    )?;
    Ok(())
}
