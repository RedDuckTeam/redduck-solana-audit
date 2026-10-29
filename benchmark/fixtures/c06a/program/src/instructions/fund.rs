use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::CrateError;
use crate::state::House;

#[derive(Accounts)]
pub struct FundPrizes<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [House::SEED, house.token_mint.as_ref()],
        bump = house.bump,
        has_one = authority @ CrateError::NotAuthority,
    )]
    pub house: Account<'info, House>,

    #[account(mut, address = house.vault)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = house.token_mint, token::authority = authority)]
    pub source: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn fund_prizes(ctx: Context<FundPrizes>, amount: u64) -> Result<()> {
    require!(amount > 0, CrateError::ZeroAmount);
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;
    Ok(())
}
