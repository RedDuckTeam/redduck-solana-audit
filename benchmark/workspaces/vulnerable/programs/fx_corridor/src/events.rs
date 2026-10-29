use anchor_lang::prelude::*;

#[event]
pub struct SettingsChanged {
    pub desk: Pubkey,
    pub operator: Pubkey,
    pub publisher: Pubkey,
    pub fee_bps: u16,
    pub max_rate_age: i64,
}

#[event]
pub struct PauseChanged {
    pub desk: Pubkey,
    pub paused_until: i64,
}

#[event]
pub struct LiquidityMoved {
    pub desk: Pubkey,
    pub vault: Pubkey,
    pub amount: u64,
    pub deposit: bool,
}

#[event]
pub struct RatePosted {
    pub desk: Pubkey,
    pub rate: u64,
    pub published_at: i64,
    pub submitted_by: Pubkey,
}

#[event]
pub struct Converted {
    pub desk: Pubkey,
    pub user: Pubkey,
    pub sell_base: bool,
    pub amount_in: u64,
    pub amount_out: u64,
    pub rate: u64,
}
