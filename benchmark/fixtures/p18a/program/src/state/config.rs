use anchor_lang::prelude::*;

pub const MAX_MEMBERS: usize = 5;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub members: [Pubkey; MAX_MEMBERS],
    pub member_count: u8,
    pub threshold: u8,
    /// The PDA that owns the treasury and signs executed proposals.
    pub vault_bump: u8,
    pub bump: u8,
}

impl Config {
    pub fn is_member(&self, key: &Pubkey) -> bool {
        self.members[..self.member_count as usize].contains(key)
    }
}
