use anchor_lang::prelude::*;

use crate::errors::LockerError;

pub const SEASON_SEED: &[u8] = b"season";
pub const POSITION_SEED: &[u8] = b"position";
pub const STAKE_VAULT_SEED: &[u8] = b"stake_vault";
pub const REWARD_VAULT_SEED: &[u8] = b"reward_vault";

pub const MIN_LOCK_SECS: i64 = 7 * 86_400;
pub const MAX_LOCK_SECS: i64 = 4 * 365 * 86_400;

#[account]
#[derive(InitSpace)]
pub struct Season {
    pub authority: Pubkey,
    pub stake_mint: Pubkey,
    pub id: u64,
    pub starts_at: i64,
    pub ends_at: i64,
    pub reward_amount: u64,
    /// Sum of the weights of all open positions in this season.
    pub total_weight: u128,
    /// `total_weight` as recorded by `finalize_season`; rewards are paid against it.
    pub final_total_weight: u128,
    pub finalized: bool,
    pub bump: u8,
    pub stake_vault_bump: u8,
    pub reward_vault_bump: u8,
}

impl Season {
    /// New stake is accepted only before the season starts.
    pub fn require_enrollment(&self, now: i64) -> Result<()> {
        require!(now < self.starts_at, LockerError::EnrollmentClosed);
        Ok(())
    }

    /// Lock changes are accepted until the season ends.
    pub fn require_open(&self, now: i64) -> Result<()> {
        require!(now < self.ends_at, LockerError::SeasonClosed);
        Ok(())
    }

    pub fn add_weight(&mut self, weight: u128) -> Result<()> {
        self.total_weight = self.total_weight.checked_add(weight).ok_or(LockerError::MathOverflow)?;
        Ok(())
    }

    pub fn remove_weight(&mut self, weight: u128) -> Result<()> {
        self.total_weight = self.total_weight.checked_sub(weight).ok_or(LockerError::MathOverflow)?;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Position {
    pub season: Pubkey,
    pub owner: Pubkey,
    pub id: u64,
    pub amount: u64,
    pub lock_end: i64,
    pub weight: u128,
    pub reward_claimed: bool,
    pub bump: u8,
}
