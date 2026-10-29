use anchor_lang::prelude::*;

#[error_code]
pub enum FxError {
    #[msg("Fee is above the allowed maximum")]
    FeeTooHigh,
    #[msg("Maximum rate age is out of range")]
    InvalidRateAge,
    #[msg("Pause duration is out of range")]
    InvalidPause,
    #[msg("Signer is not authorized")]
    Unauthorized,
    #[msg("Desk is paused")]
    Paused,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Account is not a desk vault")]
    UnknownVault,
    #[msg("No rate has been posted")]
    RateUnavailable,
    #[msg("Posted rate is too old")]
    RateExpired,
    #[msg("Quote timestamp is in the future")]
    QuoteFromFuture,
    #[msg("Rate must be positive")]
    InvalidRate,
    #[msg("Expected an Ed25519 signature check before this instruction")]
    MissingSignature,
    #[msg("Ed25519 instruction is malformed")]
    MalformedSignature,
    #[msg("Quote is not signed by the publisher")]
    WrongPublisher,
    #[msg("Signed message does not match the quote")]
    MessageMismatch,
    #[msg("Output is below the minimum")]
    SlippageExceeded,
    #[msg("Not enough liquidity")]
    InsufficientLiquidity,
    #[msg("Math overflow")]
    MathOverflow,
}
