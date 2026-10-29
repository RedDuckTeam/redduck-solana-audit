use anchor_lang::prelude::*;

pub const COUNCIL_SEED: &[u8] = b"council";
pub const TREASURY_SEED: &[u8] = b"treasury";
pub const ACTION_SEED: &[u8] = b"action";

pub const MAX_MEMBERS: usize = 10;
pub const MAX_ACTION_ACCOUNTS: usize = 12;
pub const MAX_ACTION_DATA: usize = 256;
pub const MIN_DELAY_SECS: i64 = 86_400;

#[account]
#[derive(InitSpace)]
pub struct Council {
    pub creator: Pubkey,
    pub guardian: Pubkey,
    #[max_len(MAX_MEMBERS)]
    pub members: Vec<Pubkey>,
    pub threshold: u8,
    /// Seconds between `queue_action` and the earliest execution.
    pub delay_secs: i64,
    /// Seconds after the delay during which a queued action may still run.
    pub grace_secs: i64,
    pub action_count: u64,
    pub bump: u8,
    pub treasury_bump: u8,
}

impl Council {
    pub fn member_index(&self, key: &Pubkey) -> Option<usize> {
        self.members.iter().position(|m| m == key)
    }
}

/// One account of the stored instruction, with the privileges it is approved for.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ActionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

/// An instruction the treasury will sign once the council approves it.
#[account]
#[derive(InitSpace)]
pub struct Action {
    pub council: Pubkey,
    pub proposer: Pubkey,
    pub index: u64,
    pub program_id: Pubkey,
    #[max_len(MAX_ACTION_ACCOUNTS)]
    pub accounts: Vec<ActionAccount>,
    #[max_len(MAX_ACTION_DATA)]
    pub data: Vec<u8>,
    /// Bit `i` is set when `council.members[i]` has approved.
    pub approvals: u16,
    pub queued_at: i64,
    pub executed: bool,
    pub cancelled: bool,
    pub bump: u8,
}

impl Action {
    pub fn approval_count(&self) -> u32 {
        self.approvals.count_ones()
    }

    pub fn has_approved(&self, member_index: usize) -> bool {
        self.approvals & (1u16 << member_index) != 0
    }
}
