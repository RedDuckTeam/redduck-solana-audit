use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Entry {
    pub round: Pubkey,
    pub player: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
