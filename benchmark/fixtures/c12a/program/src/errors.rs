use anchor_lang::prelude::*;

#[error_code]
pub enum DealError {
    #[msg("Unit count must be greater than zero")]
    ZeroUnits,
    #[msg("Arithmetic overflow")]
    MathOverflow,
    #[msg("Deal parameters are out of range")]
    InvalidParams,
    #[msg("Deal is not open for commitments")]
    DealClosed,
    #[msg("Deal is full")]
    DealFull,
    #[msg("Buyer would exceed the per-buyer limit")]
    BuyerLimit,
    #[msg("Buyer has not joined this deal")]
    NotJoined,
    #[msg("Deal did not succeed")]
    NotSucceeded,
    #[msg("Deal has not failed")]
    NotFailed,
    #[msg("Nothing to refund")]
    NothingToRefund,
}
