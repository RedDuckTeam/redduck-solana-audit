use anchor_lang::prelude::*;

#[error_code]
pub enum CardError {
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Arithmetic overflow")]
    MathOverflow,
    #[msg("Card balance is too low")]
    InsufficientBalance,
    #[msg("No withdrawal has been requested")]
    NoWithdrawal,
    #[msg("Authorization id was already settled")]
    DuplicateAuthorization,
    #[msg("Signer is not the card processor")]
    NotProcessor,
    #[msg("No configuration change is queued")]
    NothingQueued,
    #[msg("Queued change is not ready yet")]
    ChangeNotReady,
}
