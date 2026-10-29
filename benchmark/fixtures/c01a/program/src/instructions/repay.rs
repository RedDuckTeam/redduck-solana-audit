use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::DeskError;
use crate::state::{Market, Position};

#[derive(Accounts)]
pub struct Repay<'info> {
    pub owner: Signer<'info>,

    #[account(seeds = [Market::SEED, market.base_mint.as_ref(), market.quote_mint.as_ref()], bump = market.bump)]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [Position::SEED, market.key().as_ref(), owner.key().as_ref()],
        bump = position.bump,
        has_one = owner,
        has_one = market,
    )]
    pub position: Account<'info, Position>,

    #[account(mut, address = market.borrow_vault)]
    pub borrow_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = market.quote_mint, token::authority = owner)]
    pub owner_quote: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
    require!(amount > 0, DeskError::ZeroAmount);
    require!(amount <= ctx.accounts.position.debt, DeskError::RepayTooLarge);

    let repay_in = Transfer {
        from: ctx.accounts.owner_quote.to_account_info(),
        to: ctx.accounts.borrow_vault.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), repay_in), amount)?;
    ctx.accounts.position.debt -= amount;
    Ok(())
}
