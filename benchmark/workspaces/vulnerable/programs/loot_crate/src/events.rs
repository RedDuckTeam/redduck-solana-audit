use anchor_lang::prelude::*;

#[event]
pub struct CrateOpened {
    pub house: Pubkey,
    pub player: Pubkey,
    pub reveal_slot: u64,
}

#[event]
pub struct Revealed {
    pub house: Pubkey,
    pub player: Pubkey,
    pub won: bool,
    pub payout: u64,
}
