use anchor_lang::prelude::*;

#[event]
pub struct CouncilCreated {
    pub council: Pubkey,
    pub guardian: Pubkey,
    pub members: u8,
    pub threshold: u8,
}

#[event]
pub struct ActionProposed {
    pub action: Pubkey,
    pub proposer: Pubkey,
    pub program_id: Pubkey,
    pub index: u64,
}

#[event]
pub struct ApprovalChanged {
    pub action: Pubkey,
    pub member: Pubkey,
    pub approved: bool,
    pub approvals: u32,
}

#[event]
pub struct ActionQueued {
    pub action: Pubkey,
    pub ready_at: i64,
}

#[event]
pub struct ActionCancelled {
    pub action: Pubkey,
    pub by: Pubkey,
}

#[event]
pub struct ActionExecuted {
    pub action: Pubkey,
    pub executor: Pubkey,
}
