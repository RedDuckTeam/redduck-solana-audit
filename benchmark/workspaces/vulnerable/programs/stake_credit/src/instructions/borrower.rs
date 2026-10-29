use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::CreditError;
use crate::events::AccountUpdated;
use crate::pricing::{load_quotes, within_limit};
use crate::state::*;

use super::{pay_in, pay_out};

#[derive(Accounts)]
pub struct PositionAction<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(mut, has_one = owner, has_one = market)]
    pub credit_account: Account<'info, CreditAccount>,
    #[account(mut, address = market.collateral_vault)]
    pub collateral_vault: Account<'info, TokenAccount>,
    #[account(mut, address = market.liquidity_vault)]
    pub liquidity_vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = market.collateral_mint, token::authority = owner)]
    pub owner_collateral_account: Account<'info, TokenAccount>,
    #[account(mut, token::mint = market.debt_mint, token::authority = owner)]
    pub owner_debt_account: Account<'info, TokenAccount>,
    /// CHECK: checked against `market.collateral_feed` by `Quote::load`.
    pub collateral_feed: UncheckedAccount<'info>,
    /// CHECK: checked against `market.debt_feed` by `Quote::load`.
    pub debt_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

impl PositionAction<'_> {
    /// Fails unless the account stays within the market's loan-to-value limit.
    fn check_ltv(&self) -> Result<()> {
        let (coll, debt) = load_quotes(&self.market, &self.collateral_feed, &self.debt_feed)?;
        let limit = self.market.params.ltv_bps;
        require!(within_limit(&self.market, &self.credit_account, &coll, &debt, limit)?, CreditError::LtvExceeded);
        Ok(())
    }
}

pub fn borrow(ctx: Context<PositionAction>, deposit: u64, amount: u64) -> Result<()> {
    require!(deposit > 0 || amount > 0, CreditError::ZeroAmount);
    let a = ctx.accounts;
    a.credit_account.collateral = add(a.credit_account.collateral, deposit)?;
    if deposit > 0 {
        pay_in(&a.token_program, &a.owner_collateral_account, &a.collateral_vault, &a.owner, deposit)?;
    }
    if amount > 0 {
        require!(amount <= a.market.cash, CreditError::InsufficientLiquidity);
        let owed = add(amount, a.market.borrow_fee(amount)?)?;
        a.credit_account.debt = add(a.credit_account.debt, owed)?;
        a.market.total_debt = add(a.market.total_debt, owed)?;
        a.market.cash = sub(a.market.cash, amount)?;
        a.check_ltv()?;
        pay_out(&a.token_program, &a.market, &a.liquidity_vault, &a.owner_debt_account, amount)?;
    }
    emit!(AccountUpdated::of(&a.credit_account));
    Ok(())
}

pub fn repay(ctx: Context<PositionAction>, amount: u64, withdraw: u64) -> Result<()> {
    let a = ctx.accounts;
    let repaid = amount.min(a.credit_account.debt);
    require!(repaid > 0 || withdraw > 0, CreditError::ZeroAmount);
    if repaid > 0 {
        a.credit_account.debt = sub(a.credit_account.debt, repaid)?;
        a.market.total_debt = sub(a.market.total_debt, repaid)?;
        a.market.cash = add(a.market.cash, repaid)?;
        pay_in(&a.token_program, &a.owner_debt_account, &a.liquidity_vault, &a.owner, repaid)?;
    }
    if withdraw > 0 {
        require!(withdraw <= a.credit_account.collateral, CreditError::InsufficientCollateral);
        a.credit_account.collateral = sub(a.credit_account.collateral, withdraw)?;
        if a.credit_account.debt > 0 {
            a.check_ltv()?;
        }
        pay_out(&a.token_program, &a.market, &a.collateral_vault, &a.owner_collateral_account, withdraw)?;
    }
    emit!(AccountUpdated::of(&a.credit_account));
    Ok(())
}
