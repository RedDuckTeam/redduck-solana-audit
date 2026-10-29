use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::DeskError;
use crate::events::Borrowed;
use crate::state::{Market, Position, PRICE_SCALE};

#[derive(Accounts)]
pub struct Borrow<'info> {
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

    #[account(address = market.pool_base_vault)]
    pub pool_base_vault: Account<'info, TokenAccount>,

    #[account(address = market.pool_quote_vault)]
    pub pool_quote_vault: Account<'info, TokenAccount>,

    #[account(mut, address = market.borrow_vault)]
    pub borrow_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = market.quote_mint, token::authority = owner)]
    pub owner_quote: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
    require!(amount > 0, DeskError::ZeroAmount);
    let base_liquidity = ctx.accounts.pool_base_vault.amount;
    let quote_liquidity = ctx.accounts.pool_quote_vault.amount;
    require!(base_liquidity > 0 && quote_liquidity > 0, DeskError::NoLiquidity);
    let price = (quote_liquidity as u128)
        .checked_mul(PRICE_SCALE)
        .ok_or(DeskError::MathOverflow)?
        / base_liquidity as u128;
    let market = &ctx.accounts.market;
    let position = &ctx.accounts.position;
    let collateral_value = (position.collateral as u128)
        .checked_mul(price)
        .ok_or(DeskError::MathOverflow)?
        / PRICE_SCALE;
    let max_debt = collateral_value
        .checked_mul(market.ltv_bps as u128)
        .ok_or(DeskError::MathOverflow)?
        / 10_000u128;
    let new_debt = position.debt.checked_add(amount).ok_or(DeskError::MathOverflow)?;
    require!((new_debt as u128) <= max_debt, DeskError::Undercollateralized);
    let base = market.base_mint;
    let quote = market.quote_mint;
    let seeds = market.signer_seeds(&base, &quote);
    let out = Transfer {
        from: ctx.accounts.borrow_vault.to_account_info(),
        to: ctx.accounts.owner_quote.to_account_info(),
        authority: ctx.accounts.market.to_account_info(),
    };
    let program = ctx.accounts.token_program.to_account_info();
    token::transfer(CpiContext::new_with_signer(program, out, &[&seeds]), amount)?;

    ctx.accounts.position.debt = new_debt;
    emit!(Borrowed {
        market: ctx.accounts.market.key(),
        owner: ctx.accounts.owner.key(),
        amount,
        debt: new_debt,
    });
    Ok(())
}
