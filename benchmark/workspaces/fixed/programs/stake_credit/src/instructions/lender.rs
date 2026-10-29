use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::CreditError;
use crate::events::AccountUpdated;
use crate::state::*;

use super::{pay_in, pay_out};

#[derive(Accounts)]
pub struct PoolAction<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(mut, has_one = owner, has_one = market)]
    pub credit_account: Account<'info, CreditAccount>,
    #[account(mut, address = market.liquidity_vault)]
    pub liquidity_vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = market.debt_mint, token::authority = owner)]
    pub owner_debt_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn supply(ctx: Context<PoolAction>, amount: u64) -> Result<()> {
    require!(amount > 0, CreditError::ZeroAmount);
    let a = ctx.accounts;
    let shares = a.market.shares_for_deposit(amount)?;
    require!(shares > 0, CreditError::ZeroAmount);

    a.market.cash = add(a.market.cash, amount)?;
    a.market.total_shares = add(a.market.total_shares, shares)?;
    a.credit_account.shares = add(a.credit_account.shares, shares)?;

    pay_in(&a.token_program, &a.owner_debt_account, &a.liquidity_vault, &a.owner, amount)?;
    emit!(AccountUpdated::of(&a.credit_account));
    Ok(())
}

pub fn withdraw_supply(ctx: Context<PoolAction>, shares: u64) -> Result<()> {
    require!(shares > 0, CreditError::ZeroAmount);
    let a = ctx.accounts;
    require!(shares <= a.credit_account.shares, CreditError::InsufficientShares);
    let amount = a.market.assets_for_shares(shares)?;
    require!(amount > 0, CreditError::ZeroAmount);
    require!(amount <= a.market.cash, CreditError::InsufficientLiquidity);

    a.credit_account.shares = sub(a.credit_account.shares, shares)?;
    a.market.total_shares = sub(a.market.total_shares, shares)?;
    a.market.cash = sub(a.market.cash, amount)?;

    pay_out(&a.token_program, &a.market, &a.liquidity_vault, &a.owner_debt_account, amount)?;
    emit!(AccountUpdated::of(&a.credit_account));
    Ok(())
}
