use anchor_lang::prelude::*;

#[error_code]
pub enum CrateError {
    #[msg("amount must be greater than zero")]
    ZeroAmount,
    #[msg("odds must be between 1 and 10000 bps")]
    BadOdds,
    #[msg("prize must be at least the cost")]
    BadPrize,
    #[msg("the randomness slot has not passed yet")]
    TooEarly,
    #[msg("the slot hashes account is malformed")]
    BadSlotHashes,
    #[msg("withdrawal would dip into reserved funds")]
    WouldUnderfund,
    #[msg("the house cannot cover this crate's prize")]
    Underfunded,
    #[msg("only the house authority may call this")]
    NotAuthority,
    #[msg("arithmetic overflow")]
    MathOverflow,
}
