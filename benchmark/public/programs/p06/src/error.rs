use anchor_lang::prelude::*;

#[error_code]
pub enum RewardsError {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Nothing to pay")]
    NothingToPay,
    #[msg("The price is too old to trade on")]
    StalePrice,
    #[msg("The instruction before this one is not an Ed25519 signature check")]
    NotEd25519,
    #[msg("The Ed25519 instruction has an unexpected layout")]
    MalformedEd25519,
}
