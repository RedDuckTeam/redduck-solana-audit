use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::DeskError;
use crate::state::Market;

#[derive(Accounts)]
pub struct Swap<'info> {
    pub trader: Signer<'info>,

    #[account(
        mut,
        seeds = [Market::SEED, market.base_mint.as_ref(), market.quote_mint.as_ref()],
        bump = market.bump,
    )]
    pub market: Account<'info, Market>,

    #[account(mut, address = market.pool_base_vault)]
    pub pool_base_vault: Account<'info, TokenAccount>,

    #[account(mut, address = market.pool_quote_vault)]
    pub pool_quote_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = market.base_mint, token::authority = trader)]
    pub trader_base: Account<'info, TokenAccount>,

    #[account(mut, token::mint = market.quote_mint, token::authority = trader)]
    pub trader_quote: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn swap_base_for_quote(ctx: Context<Swap>, base_in: u64, min_quote_out: u64) -> Result<()> {
    require!(base_in > 0, DeskError::ZeroAmount);
    let market = &ctx.accounts.market;
    require!(market.reserve_base > 0 && market.reserve_quote > 0, DeskError::NoLiquidity);

    let new_base = (market.reserve_base as u128)
        .checked_add(base_in as u128)
        .ok_or(DeskError::MathOverflow)?;
    let quote_out = (market.reserve_quote as u128)
        .checked_mul(base_in as u128)
        .ok_or(DeskError::MathOverflow)?
        / new_base;
    let quote_out = u64::try_from(quote_out).map_err(|_| DeskError::MathOverflow)?;
    require!(quote_out >= min_quote_out, DeskError::Slippage);

    let program = ctx.accounts.token_program.to_account_info();
    let pay_in = Transfer {
        from: ctx.accounts.trader_base.to_account_info(),
        to: ctx.accounts.pool_base_vault.to_account_info(),
        authority: ctx.accounts.trader.to_account_info(),
    };
    token::transfer(CpiContext::new(program.clone(), pay_in), base_in)?;
    let (base, quote) = (market.base_mint, market.quote_mint);
    let seeds = market.signer_seeds(&base, &quote);
    let pay_out = Transfer {
        from: ctx.accounts.pool_quote_vault.to_account_info(),
        to: ctx.accounts.trader_quote.to_account_info(),
        authority: ctx.accounts.market.to_account_info(),
    };
    token::transfer(CpiContext::new_with_signer(program, pay_out, &[&seeds]), quote_out)?;
    let market = &mut ctx.accounts.market;
    market.reserve_base = market.reserve_base.checked_add(base_in).ok_or(DeskError::MathOverflow)?;
    market.reserve_quote = market.reserve_quote.checked_sub(quote_out).ok_or(DeskError::MathOverflow)?;
    Ok(())
}
