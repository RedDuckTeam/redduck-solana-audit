use anchor_lang::prelude::*;

/// Fixed-point scale for the per-share accumulator.
pub const ACC_SCALE: u128 = 1_000_000_000_000;

/// Minimum time a deposit must remain staked before it can be withdrawn.
pub const WITHDRAW_LOCK_SECS: i64 = 60 * 60;

/// A commission-sharing pool. Delegators stake a token and share a reward token that
/// the operator periodically distributes. Rewards are tracked with a running
/// per-share accumulator rather than pushed to each staker.
#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub stake_mint: Pubkey,
    pub reward_mint: Pubkey,
    pub stake_vault: Pubkey,
    pub reward_vault: Pubkey,
    pub total_staked: u64,
    pub acc_reward_per_share: u128,
    pub bump: u8,
}

impl Pool {
    pub const SEED: &'static [u8] = b"pool";
    pub const SPACE: usize = 8 + 32 * 5 + 8 + 16 + 1;

    pub fn signer_seeds<'a>(&'a self, mint: &'a Pubkey) -> [&'a [u8]; 3] {
        [Self::SEED, mint.as_ref(), std::slice::from_ref(&self.bump)]
    }

    pub fn accrued(&self, staked: u64) -> Option<u128> {
        (staked as u128)
            .checked_mul(self.acc_reward_per_share)
            .map(|v| v / ACC_SCALE)
    }
}

/// A single delegator's stake and its reward marker. `reward_debt` is the amount of
/// the accumulator this account has already been paid for.
#[account]
pub struct Stake {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub staked: u64,
    pub reward_debt: u128,
    pub unlock_ts: i64,
    pub bump: u8,
}

impl Stake {
    pub const SEED: &'static [u8] = b"stake";
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 16 + 8 + 1;
}
