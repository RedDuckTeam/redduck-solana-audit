use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::CreditError;
use crate::events::Liquidated;
use crate::pricing::{load_quotes, within_limit};
use crate::state::*;

use super::{pay_in, pay_out};

#[derive(Accounts)]
pub struct Liquidate<'info> {
    pub liquidator: Signer<'info>,
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(mut, has_one = market)]
    pub borrower: Account<'info, CreditAccount>,
    #[account(mut, address = market.liquidity_vault)]
    pub liquidity_vault: Account<'info, TokenAccount>,
    #[account(mut, address = market.collateral_vault)]
    pub collateral_vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = market.debt_mint, token::authority = liquidator)]
    pub liquidator_debt_account: Account<'info, TokenAccount>,
    #[account(mut, token::mint = market.collateral_mint, token::authority = liquidator)]
    pub liquidator_collateral_account: Account<'info, TokenAccount>,
    /// CHECK: checked against `market.collateral_feed` by `Quote::load`.
    pub collateral_feed: UncheckedAccount<'info>,
    /// CHECK: checked against `market.debt_feed` by `Quote::load`.
    pub debt_feed: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn liquidate(ctx: Context<Liquidate>, max_repay: u64) -> Result<()> {
    let a = ctx.accounts;
    let (coll, debt) = load_quotes(&a.market, &a.collateral_feed, &a.debt_feed)?;
    let healthy = within_limit(&a.market, &a.borrower, &coll, &debt, a.market.params.liquidation_threshold_bps)?;
    require!(!healthy, CreditError::PositionHealthy);

    let repaid = max_repay.min(a.borrower.debt);
    require!(repaid > 0, CreditError::ZeroAmount);

    // Repaid value plus the bonus, paid in collateral and capped at what the position holds.
    let bonus = BPS as u128 + a.market.params.liquidation_bonus_bps as u128;
    let repaid_value = debt.value(repaid, a.market.debt_decimals, Rounding::Down)?;
    let seize_value = mul_div(repaid_value, bonus, BPS as u128, Rounding::Down)?;
    let seized = coll.amount_for(seize_value, a.market.collateral_decimals)?.min(a.borrower.collateral);

    a.borrower.debt = sub(a.borrower.debt, repaid)?;
    a.borrower.collateral = sub(a.borrower.collateral, seized)?;
    a.market.total_debt = sub(a.market.total_debt, repaid)?;
    a.market.cash = add(a.market.cash, repaid)?;

    // Debt left without collateral is written off so the share price reflects the loss at once.
    let mut written_off = 0;
    if a.borrower.collateral == 0 && a.borrower.debt > 0 {
        written_off = a.borrower.debt;
        a.market.total_debt = sub(a.market.total_debt, written_off)?;
        a.borrower.debt = 0;
    }

    pay_in(&a.token_program, &a.liquidator_debt_account, &a.liquidity_vault, &a.liquidator, repaid)?;
    if seized > 0 {
        pay_out(&a.token_program, &a.market, &a.collateral_vault, &a.liquidator_collateral_account, seized)?;
    }
    emit!(Liquidated {
        market: a.market.key(),
        owner: a.borrower.owner,
        liquidator: a.liquidator.key(),
        repaid,
        seized,
        written_off,
    });
    Ok(())
}
