use anchor_lang::prelude::*;

#[event]
pub struct ConfigQueued {
    pub config: Pubkey,
    pub settlement: Pubkey,
    pub eta: i64,
}

#[event]
pub struct ConfigApplied {
    pub config: Pubkey,
    pub settlement: Pubkey,
}

#[event]
pub struct CardToppedUp {
    pub card: Pubkey,
    pub amount: u64,
    pub balance: u64,
}

#[event]
pub struct SpendSettled {
    pub card: Pubkey,
    pub auth_id: u64,
    pub amount: u64,
    pub balance: u64,
}

#[event]
pub struct WithdrawalRequested {
    pub card: Pubkey,
    pub amount: u64,
    pub requested_at: i64,
}

#[event]
pub struct WithdrawalCompleted {
    pub card: Pubkey,
    pub amount: u64,
    pub balance: u64,
}
