use anchor_lang::prelude::*;

#[event]
pub struct SeasonCreated {
    pub season: Pubkey,
    pub starts_at: i64,
    pub ends_at: i64,
    pub reward_amount: u64,
}

#[event]
pub struct PositionUpdated {
    pub position: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub lock_end: i64,
    pub weight: u128,
}

#[event]
pub struct PositionClosed {
    pub position: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[event]
pub struct SeasonFinalized {
    pub season: Pubkey,
    pub final_total_weight: u128,
}

#[event]
pub struct RewardClaimed {
    pub position: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}
