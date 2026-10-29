use anchor_lang::prelude::*;

#[event]
pub struct Loaded {
    pub hub: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Transferred {
    pub hub: Pubkey,
    pub from: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Redeemed {
    pub hub: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}
