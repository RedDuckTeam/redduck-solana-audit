use anchor_lang::prelude::*;

pub const MAX_KEYS: usize = 4;

#[account]
#[derive(InitSpace)]
pub struct Allowlist {
    pub authority: Pubkey,
    #[max_len(MAX_KEYS)]
    pub keys: Vec<Pubkey>,
}
