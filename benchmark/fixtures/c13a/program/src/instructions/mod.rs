use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{Market, MARKET_SEED};

pub mod borrower;
pub mod lender;
pub mod liquidate;
pub mod market;

pub use borrower::*;
pub use lender::*;
pub use liquidate::*;
pub use market::*;

type TokenAcc<'info> = Account<'info, TokenAccount>;

/// Moves tokens out of a market vault, signed by the market PDA.
pub(crate) fn pay_out<'info>(
    program: &Program<'info, Token>, market: &Account<'info, Market>, vault: &TokenAcc<'info>, to: &TokenAcc<'info>, amount: u64,
) -> Result<()> {
    let bump = [market.bump];
    let seeds: &[&[u8]] =
        &[MARKET_SEED, market.admin.as_ref(), market.collateral_mint.as_ref(), market.debt_mint.as_ref(), &bump];
    let accounts = Transfer { from: vault.to_account_info(), to: to.to_account_info(), authority: market.to_account_info() };
    token::transfer(CpiContext::new_with_signer(program.to_account_info(), accounts, &[seeds]), amount)
}

/// Moves tokens from an account the signer owns.
pub(crate) fn pay_in<'info>(
    program: &Program<'info, Token>, from: &TokenAcc<'info>, to: &TokenAcc<'info>, authority: &Signer<'info>, amount: u64,
) -> Result<()> {
    let accounts = Transfer { from: from.to_account_info(), to: to.to_account_info(), authority: authority.to_account_info() };
    token::transfer(CpiContext::new(program.to_account_info(), accounts), amount)
}
