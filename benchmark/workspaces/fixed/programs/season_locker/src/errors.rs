use anchor_lang::prelude::*;

#[error_code]
pub enum LockerError {
    #[msg("Season window is invalid")]
    InvalidSeasonWindow,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Lock duration is outside the allowed range")]
    InvalidLockDuration,
    #[msg("Enrollment for this season is closed")]
    EnrollmentClosed,
    #[msg("Season has ended")]
    SeasonClosed,
    #[msg("Season has not ended yet")]
    SeasonNotEnded,
    #[msg("Season is already finalized")]
    AlreadyFinalized,
    #[msg("Season is not finalized")]
    NotFinalized,
    #[msg("Reward already claimed")]
    AlreadyClaimed,
    #[msg("Claim the season reward before withdrawing")]
    ClaimFirst,
    #[msg("Position is still locked")]
    StillLocked,
    #[msg("New lock end must be later than the current one")]
    LockNotExtended,
    #[msg("Positions must be distinct")]
    SamePosition,
    #[msg("Math overflow")]
    MathOverflow,
}
