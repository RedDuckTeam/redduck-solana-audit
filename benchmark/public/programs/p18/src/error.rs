use anchor_lang::prelude::*;

#[error_code]
pub enum MultisigError {
    #[msg("Between one and five members, and a threshold within that")]
    InvalidMembers,
    #[msg("Only a member can do this")]
    NotAMember,
    #[msg("This member already approved")]
    AlreadyApproved,
    #[msg("Too many accounts or too much data for one proposal")]
    ProposalTooBig,
    #[msg("Not enough approvals yet")]
    NotApproved,
    #[msg("The proposal was already executed")]
    AlreadyExecuted,
    #[msg("The accounts passed do not match the stored proposal")]
    AccountMismatch,
}
