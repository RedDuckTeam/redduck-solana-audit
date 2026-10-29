use anchor_lang::prelude::*;

use crate::errors::FxError;

pub const DESK_SEED: &[u8] = b"desk";
pub const VAULT_SEED: &[u8] = b"vault";

/// Rates are quote-token base units per base-token base unit, scaled by 1e9.
pub const RATE_SCALE: u128 = 1_000_000_000;
pub const BPS: u128 = 10_000;
pub const MAX_FEE_BPS: u16 = 200;
pub const MIN_RATE_AGE_SECS: i64 = 5;
pub const MAX_RATE_AGE_SECS: i64 = 600;
pub const MAX_PAUSE_SECS: i64 = 3 * 86_400;

#[account]
#[derive(InitSpace)]
pub struct Desk {
    pub admin: Pubkey,
    pub operator: Pubkey,
    pub publisher: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
    pub fee_bps: u16,
    pub max_rate_age: i64,
    pub rate: u64,
    pub rate_published_at: i64,
    pub paused_until: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DeskSettings {
    pub operator: Pubkey,
    pub publisher: Pubkey,
    pub fee_bps: u16,
    pub max_rate_age: i64,
}

impl Desk {
    pub fn apply(&mut self, settings: &DeskSettings) -> Result<()> {
        require!(settings.fee_bps <= MAX_FEE_BPS, FxError::FeeTooHigh);
        require!(
            (MIN_RATE_AGE_SECS..=MAX_RATE_AGE_SECS).contains(&settings.max_rate_age),
            FxError::InvalidRateAge
        );
        self.operator = settings.operator;
        self.publisher = settings.publisher;
        self.fee_bps = settings.fee_bps;
        self.max_rate_age = settings.max_rate_age;
        Ok(())
    }

    /// The stored rate, if it is recent enough to trade on.
    pub fn usable_rate(&self, now: i64) -> Result<u64> {
        require!(self.rate > 0, FxError::RateUnavailable);
        require!(
            now.saturating_sub(self.rate_published_at) <= self.max_rate_age,
            FxError::RateExpired
        );
        Ok(self.rate)
    }

    /// Output for `amount_in` after the fee, rounded down.
    pub fn amount_out(&self, amount_in: u64, rate: u64, sell_base: bool) -> Result<u64> {
        let amount_in = amount_in as u128;
        let gross = if sell_base {
            amount_in * rate as u128 / RATE_SCALE
        } else {
            amount_in * RATE_SCALE / rate as u128
        };
        let net = gross * (BPS - self.fee_bps as u128) / BPS;
        u64::try_from(net).map_err(|_| error!(FxError::MathOverflow))
    }
}
