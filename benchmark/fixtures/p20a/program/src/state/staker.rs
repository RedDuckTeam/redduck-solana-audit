use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Staker {
    pub owner: Pubkey,
    pub stake: u64,
    /// Rewards credited by `add_rewards` and not yet claimed.
    pub pending: u64,
    pub bump: u8,
}
