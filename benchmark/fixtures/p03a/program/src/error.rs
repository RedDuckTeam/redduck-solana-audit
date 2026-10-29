use anchor_lang::prelude::*;

#[error_code]
pub enum ClaimError {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("The instruction before this one is not an Ed25519 signature check")]
    NotEd25519,
    #[msg("The Ed25519 instruction has an unexpected layout")]
    MalformedEd25519,
    #[msg("The message was not signed by the oracle")]
    WrongSigner,
    #[msg("The signed message does not match this claim")]
    MessageMismatch,
    #[msg("The nonce is not the next one")]
    BadNonce,
}
