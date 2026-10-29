use anchor_lang::prelude::*;

#[error_code]
pub enum CouncilError {
    #[msg("Member list is empty, too long or has duplicates")]
    InvalidMembers,
    #[msg("Threshold must be between 1 and the number of members")]
    InvalidThreshold,
    #[msg("Timelock settings are invalid")]
    InvalidTimelock,
    #[msg("Signer is not a council member")]
    NotMember,
    #[msg("Action payload is too large")]
    PayloadTooLarge,
    #[msg("Action may not target this program")]
    SelfInvocation,
    #[msg("Member has already approved")]
    AlreadyApproved,
    #[msg("Member has not approved")]
    NotApproved,
    #[msg("Not enough approvals")]
    ThresholdNotMet,
    #[msg("Action is already queued")]
    AlreadyQueued,
    #[msg("Action is not queued")]
    NotQueued,
    #[msg("Action was cancelled")]
    ActionCancelled,
    #[msg("Action was already executed")]
    AlreadyExecuted,
    #[msg("Timelock delay has not elapsed")]
    DelayNotElapsed,
    #[msg("Execution window has passed")]
    ActionExpired,
    #[msg("Signer may not cancel this action")]
    CancelNotAllowed,
    #[msg("Program account does not match the action")]
    ProgramMismatch,
    #[msg("Accounts do not match the action")]
    AccountMismatch,
    #[msg("Math overflow")]
    MathOverflow,
}
