use anchor_lang::prelude::*;

#[event]
pub struct Purchased {
    pub sale: Pubkey,
    pub buyer: Pubkey,
    pub credited: u64,
    pub total_purchased: u64,
}

#[event]
pub struct Claimed {
    pub sale: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ProceedsWithdrawn {
    pub sale: Pubkey,
    pub amount: u64,
}
