use anchor_lang::prelude::*;

use crate::errors::CreditError;
use crate::state::{mul_div, to_u64, CreditAccount, Market, Rounding, BPS, MAX_PRICE_AGE};

/// Program that owns every price feed account this program reads.
pub const FEED_PROGRAM_ID: Pubkey = pubkey!("2NiEA9sYuVe8Uo7vGSSXxwqt7s7RxR2SCtVTMMBYvJvj");

/// Values are expressed in quote units with this many decimals.
pub const VALUE_DECIMALS: u32 = 6;

/// Layout of a feed account as written by the feed program.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct FeedData {
    pub price: i64,
    pub conf: u64,
    pub expo: i32,
    pub publish_time: i64,
}

#[derive(Clone, Copy)]
pub struct Quote {
    pub price: u64,
    pub conf: u64,
    pub expo: i32,
}

impl Quote {
    pub fn load(feed: &AccountInfo, expected: &Pubkey, now: i64) -> Result<Self> {
        require_keys_eq!(feed.key(), *expected, CreditError::InvalidFeed);
        require_keys_eq!(*feed.owner, FEED_PROGRAM_ID, CreditError::InvalidFeed);
        let data = feed.try_borrow_data()?;
        let raw = FeedData::deserialize(&mut &data[..]).map_err(|_| error!(CreditError::InvalidFeed))?;
        require!(raw.price > 0, CreditError::BadPrice);
        require!((-12..=0).contains(&raw.expo), CreditError::BadPrice);
        require!(now.saturating_sub(raw.publish_time) <= MAX_PRICE_AGE, CreditError::StalePrice);
        Ok(Self { price: raw.price as u64, conf: raw.conf, expo: raw.expo })
    }

    /// The same quote moved to the bottom or the top of its confidence range.
    pub fn low(&self) -> Self {
        Self { price: self.price.saturating_sub(self.conf), ..*self }
    }

    pub fn high(&self) -> Self {
        Self { price: self.price.saturating_add(self.conf), ..*self }
    }

    /// Value of `amount` base units in quote units.
    pub fn value(&self, amount: u64, decimals: u8, rounding: Rounding) -> Result<u128> {
        let raw = (amount as u128) * (self.price as u128);
        mul_div(raw, 10u128.pow(VALUE_DECIMALS), unit_scale(decimals, self.expo)?, rounding)
    }

    /// Token amount (base units) worth `value`, rounded down.
    pub fn amount_for(&self, value: u128, decimals: u8) -> Result<u64> {
        let per_unit = (self.price as u128) * 10u128.pow(VALUE_DECIMALS);
        to_u64(mul_div(value, unit_scale(decimals, self.expo)?, per_unit, Rounding::Down)?)
    }
}

/// Collateral and debt quotes for a market, read from the feeds it was created with.
pub fn load_quotes(market: &Market, coll_feed: &AccountInfo, debt_feed: &AccountInfo) -> Result<(Quote, Quote)> {
    let now = Clock::get()?.unix_timestamp;
    Ok((Quote::load(coll_feed, &market.collateral_feed, now)?, Quote::load(debt_feed, &market.debt_feed, now)?))
}

/// 10^(decimals - expo): base units per whole token times the price scale.
fn unit_scale(decimals: u8, expo: i32) -> Result<u128> {
    10u128.checked_pow(decimals as u32 + expo.unsigned_abs()).ok_or_else(|| error!(CreditError::MathOverflow))
}

/// True while the account's debt stays within `limit_bps` of its collateral value.
pub fn within_limit(market: &Market, account: &CreditAccount, coll: &Quote, debt: &Quote, limit_bps: u16) -> Result<bool> {
    let collateral_value = coll.low().value(account.collateral, market.collateral_decimals, Rounding::Down)?;
    let debt_value = debt.high().value(account.debt, market.debt_decimals, Rounding::Up)?;
    Ok(debt_value * BPS as u128 <= collateral_value * limit_bps as u128)
}
