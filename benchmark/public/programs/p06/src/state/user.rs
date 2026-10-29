use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub owner: Pubkey,
    /// Reward accrued and not yet paid.
    pub pending: u64,
    pub bump: u8,
}
