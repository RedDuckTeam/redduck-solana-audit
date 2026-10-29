use anchor_lang::prelude::*;

use crate::MAX_MEMBERS;

pub const MAX_METAS: usize = 8;
pub const MAX_DATA: usize = 256;

/// One account of the stored instruction.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct StoredMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

/// A proposed instruction: the program to call, its accounts and its instruction data.
#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub id: u64,
    pub proposer: Pubkey,
    pub program_id: Pubkey,
    #[max_len(MAX_METAS)]
    pub metas: Vec<StoredMeta>,
    #[max_len(MAX_DATA)]
    pub data: Vec<u8>,
    pub approvals: u8,
    pub approved_by: [Pubkey; MAX_MEMBERS],
    pub executed: bool,
    pub expires_at: i64,
    pub bump: u8,
}
