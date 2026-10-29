use anchor_lang::prelude::*;

use crate::errors::LockerError;
use crate::state::MAX_LOCK_SECS;

pub const BPS: u64 = 10_000;
/// Extra weight for a full-length lock: 3x on top of the base 1x.
pub const MAX_BOOST_BPS: u64 = 30_000;

/// Boost earned by tokens that stay locked for `remaining` more seconds.
pub fn boost_bps(remaining: i64) -> u64 {
    let capped = remaining.clamp(0, MAX_LOCK_SECS) as u64;
    capped * MAX_BOOST_BPS / MAX_LOCK_SECS as u64
}

/// Weight of `amount` tokens locked until `lock_end`, evaluated at `now`.
pub fn lock_weight(amount: u64, lock_end: i64, now: i64) -> Result<u128> {
    let multiplier = BPS + boost_bps(lock_end.saturating_sub(now));
    let raw = (amount as u128)
        .checked_mul(multiplier as u128)
        .ok_or(LockerError::MathOverflow)?;
    Ok(raw / BPS as u128)
}

/// Share of `pool` owed to `weight` out of `total`, rounded down.
pub fn pro_rata(pool: u64, weight: u128, total: u128) -> Result<u64> {
    if total == 0 {
        return Ok(0);
    }
    let share = (pool as u128)
        .checked_mul(weight)
        .ok_or(LockerError::MathOverflow)?
        / total;
    u64::try_from(share).map_err(|_| error!(LockerError::MathOverflow))
}
