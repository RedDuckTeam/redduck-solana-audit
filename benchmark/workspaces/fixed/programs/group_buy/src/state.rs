use anchor_lang::prelude::*;

use crate::errors::DealError;

pub const MERCHANT_SEED: &[u8] = b"merchant";
pub const DEAL_SEED: &[u8] = b"deal";
pub const ESCROW_SEED: &[u8] = b"escrow";
pub const COMMITMENT_SEED: &[u8] = b"commitment";

/// Longest time a deal may stay open for commitments.
pub const MAX_DURATION: i64 = 30 * 24 * 60 * 60;

#[account]
#[derive(InitSpace)]
pub struct Merchant {
    pub authority: Pubkey,
    /// Number of deals created so far; the next deal uses this value as its id.
    pub deal_count: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum DealStatus {
    Open,
    Cancelled,
    Claimed,
}

#[derive(PartialEq, Eq)]
pub enum Outcome {
    Open,
    Succeeded,
    Failed,
}

#[account]
#[derive(InitSpace)]
pub struct Commitment {
    pub deal: Pubkey,
    pub buyer: Pubkey,
    pub units: u32,
    pub refunded: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Deal {
    pub merchant: Pubkey,
    pub authority: Pubkey,
    pub id: u64,
    pub mint: Pubkey,
    pub escrow: Pubkey,
    pub unit_price: u64,
    pub min_units: u32,
    pub max_units: u32,
    pub max_units_per_buyer: u32,
    pub deadline: i64,
    pub total_units: u32,
    pub status: DealStatus,
    pub bump: u8,
    pub buyer_count: u32,
}

impl Deal {
    /// Open until the deadline; afterwards it succeeded if the minimum was reached.
    /// A cancelled deal counts as failed so buyers can take their money back.
    pub fn outcome(&self, now: i64) -> Outcome {
        match self.status {
            DealStatus::Cancelled => Outcome::Failed,
            DealStatus::Claimed => Outcome::Succeeded,
            DealStatus::Open if now < self.deadline => Outcome::Open,
            DealStatus::Open if self.total_units >= self.min_units => Outcome::Succeeded,
            DealStatus::Open => Outcome::Failed,
        }
    }

    pub fn cost(&self, units: u32) -> Result<u64> {
        self.unit_price
            .checked_mul(units as u64)
            .ok_or(error!(DealError::MathOverflow))
    }

    pub fn signer_seeds<'a>(&'a self, id: &'a [u8; 8], bump: &'a [u8; 1]) -> [&'a [u8]; 4] {
        [DEAL_SEED, self.merchant.as_ref(), id, bump]
    }
}
