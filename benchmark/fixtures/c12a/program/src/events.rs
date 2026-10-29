use anchor_lang::prelude::*;

#[event]
pub struct DealCreated {
    pub deal: Pubkey,
    pub merchant: Pubkey,
    pub id: u64,
    pub unit_price: u64,
    pub min_units: u32,
    pub max_units: u32,
    pub deadline: i64,
}

#[event]
pub struct BuyerJoined {
    pub deal: Pubkey,
    pub buyer: Pubkey,
}

#[event]
pub struct UnitsCommitted {
    pub deal: Pubkey,
    pub buyer: Pubkey,
    pub units: u32,
    pub total_units: u32,
}

#[event]
pub struct DealCancelled {
    pub deal: Pubkey,
}

#[event]
pub struct ProceedsClaimed {
    pub deal: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Refunded {
    pub deal: Pubkey,
    pub buyer: Pubkey,
    pub amount: u64,
}
