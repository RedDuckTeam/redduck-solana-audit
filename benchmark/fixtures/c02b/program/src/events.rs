use anchor_lang::prelude::*;

#[event]
pub struct Deposited {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Withdrawn {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[event]
pub struct RewardsDistributed {
    pub pool: Pubkey,
    pub amount: u64,
    pub acc_reward_per_share: u128,
}
