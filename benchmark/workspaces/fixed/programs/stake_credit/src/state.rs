use anchor_lang::prelude::*;

use crate::errors::CreditError;

pub const BPS: u64 = 10_000;
pub const MARKET_SEED: &[u8] = b"market";
pub const ACCOUNT_SEED: &[u8] = b"account";
/// Liquidity locked by the market creator; its shares belong to no account and are never redeemed.
pub const MIN_SEED_LIQUIDITY: u64 = 1_000_000;
/// Feed updates older than this many seconds are rejected.
pub const MAX_PRICE_AGE: i64 = 60;
pub const MAX_BORROW_FEE_BPS: u16 = 200;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct MarketParams {
    pub ltv_bps: u16,
    pub liquidation_threshold_bps: u16,
    pub liquidation_bonus_bps: u16,
    pub borrow_fee_bps: u16,
    pub seed_liquidity: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub admin: Pubkey,
    pub collateral_mint: Pubkey,
    pub debt_mint: Pubkey,
    pub collateral_vault: Pubkey,
    pub liquidity_vault: Pubkey,
    pub collateral_feed: Pubkey,
    pub debt_feed: Pubkey,
    pub collateral_decimals: u8,
    pub debt_decimals: u8,
    pub params: MarketParams,
    /// Debt tokens held by the liquidity vault on behalf of lenders.
    pub cash: u64,
    /// Sum of all borrower debt, fees included.
    pub total_debt: u64,
    pub total_shares: u64,
    pub bump: u8,
}

impl Market {
    pub fn shares_for_deposit(&self, amount: u64) -> Result<u64> {
        let assets = add(self.cash, self.total_debt)? as u128;
        to_u64(mul_div(amount as u128, self.total_shares as u128, assets, Rounding::Down)?)
    }

    pub fn assets_for_shares(&self, shares: u64) -> Result<u64> {
        let assets = add(self.cash, self.total_debt)? as u128;
        to_u64(mul_div(shares as u128, assets, self.total_shares as u128, Rounding::Down)?)
    }

    pub fn borrow_fee(&self, amount: u64) -> Result<u64> {
        let bps = self.params.borrow_fee_bps as u128;
        to_u64(mul_div(amount as u128, bps, BPS as u128, Rounding::Up)?)
    }
}

#[account]
#[derive(InitSpace)]
pub struct CreditAccount {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub shares: u64,
    pub collateral: u64,
    pub debt: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rounding { Down, Up }

pub fn mul_div(a: u128, b: u128, c: u128, rounding: Rounding) -> Result<u128> {
    require!(c > 0, CreditError::MathOverflow);
    let product = a.checked_mul(b).ok_or_else(|| error!(CreditError::MathOverflow))?;
    let round_up = rounding == Rounding::Up && product % c != 0;
    Ok(product / c + round_up as u128)
}

pub fn to_u64(value: u128) -> Result<u64> {
    u64::try_from(value).map_err(|_| error!(CreditError::MathOverflow))
}

pub fn add(a: u64, b: u64) -> Result<u64> {
    a.checked_add(b).ok_or_else(|| error!(CreditError::MathOverflow))
}

pub fn sub(a: u64, b: u64) -> Result<u64> {
    a.checked_sub(b).ok_or_else(|| error!(CreditError::MathOverflow))
}
