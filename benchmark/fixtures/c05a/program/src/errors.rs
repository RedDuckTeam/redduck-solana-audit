use anchor_lang::prelude::*;

#[error_code]
pub enum CardError {
    #[msg("amount must be greater than zero")]
    ZeroAmount,
    #[msg("arithmetic overflow")]
    MathOverflow,
    #[msg("card balance is too low")]
    InsufficientBalance,
    #[msg("card still holds a balance")]
    CardNotEmpty,
    #[msg("the two cards must be different")]
    SameCard,
}
