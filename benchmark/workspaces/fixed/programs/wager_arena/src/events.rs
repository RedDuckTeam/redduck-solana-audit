use anchor_lang::prelude::*;

#[event]
pub struct Deposited {
    pub arena: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[event]
pub struct MatchEntered {
    pub arena: Pubkey,
    pub owner: Pubkey,
    pub stake: u64,
    pub deadline: i64,
}

#[event]
pub struct Withdrawn {
    pub arena: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}
