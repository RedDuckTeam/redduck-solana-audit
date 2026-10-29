use anchor_lang::prelude::*;

#[event]
pub struct Deposited {
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub assets: u64,
    pub notes: u64,
}

#[event]
pub struct Withdrawn {
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub assets: u64,
    pub notes_burned: u64,
}

#[event]
pub struct YieldAccrued {
    pub vault: Pubkey,
    pub amount: u64,
    pub total_assets: u64,
}
