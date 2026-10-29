use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::DeskError;
use crate::state::Market;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [Market::SEED, market.base_mint.as_ref(), market.quote_mint.as_ref()],
        bump = market.bump,
        has_one = authority @ DeskError::NotAuthority,
    )]
    pub market: Account<'info, Market>,

    #[account(mut, address = market.pool_base_vault)]
    pub pool_base_vault: Account<'info, TokenAccount>,

    #[account(mut, address = market.pool_quote_vault)]
    pub pool_quote_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = market.base_mint, token::authority = authority)]
    pub source_base: Account<'info, TokenAccount>,

    #[account(mut, token::mint = market.quote_mint, token::authority = authority)]
    pub source_quote: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn add_liquidity(ctx: Context<AddLiquidity>, base_amount: u64, quote_amount: u64) -> Result<()> {
    require!(base_amount > 0 && quote_amount > 0, DeskError::ZeroAmount);

    let program = ctx.accounts.token_program.to_account_info();
    let base_in = Transfer {
        from: ctx.accounts.source_base.to_account_info(),
        to: ctx.accounts.pool_base_vault.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(CpiContext::new(program.clone(), base_in), base_amount)?;
    let quote_in = Transfer {
        from: ctx.accounts.source_quote.to_account_info(),
        to: ctx.accounts.pool_quote_vault.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(CpiContext::new(program, quote_in), quote_amount)?;
    let market = &mut ctx.accounts.market;
    market.reserve_base = market.reserve_base.checked_add(base_amount).ok_or(DeskError::MathOverflow)?;
    market.reserve_quote = market.reserve_quote.checked_add(quote_amount).ok_or(DeskError::MathOverflow)?;
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawLiquidity<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [Market::SEED, market.base_mint.as_ref(), market.quote_mint.as_ref()],
        bump = market.bump,
        has_one = authority @ DeskError::NotAuthority,
    )]
    pub market: Account<'info, Market>,

    #[account(mut, address = market.borrow_vault)]
    pub borrow_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = market.quote_mint)]
    pub destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, amount: u64) -> Result<()> {
    require!(amount > 0, DeskError::ZeroAmount);
    let market = &ctx.accounts.market;
    let (base, quote) = (market.base_mint, market.quote_mint);
    let seeds = market.signer_seeds(&base, &quote);
    let out = Transfer {
        from: ctx.accounts.borrow_vault.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.market.to_account_info(),
    };
    let program = ctx.accounts.token_program.to_account_info();
    token::transfer(CpiContext::new_with_signer(program, out, &[&seeds]), amount)
}
