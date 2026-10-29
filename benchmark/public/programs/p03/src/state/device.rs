use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Device {
    pub owner: Pubkey,
    /// The nonce of the last claim.
    pub nonce: u64,
    pub bump: u8,
}
