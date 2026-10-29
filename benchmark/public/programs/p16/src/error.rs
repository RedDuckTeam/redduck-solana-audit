use anchor_lang::prelude::*;

#[error_code]
pub enum RoundError {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("The round is closed for new entries")]
    RoundClosed,
    #[msg("The round has not ended yet")]
    RoundNotEnded,
    #[msg("The round is already settled")]
    AlreadySettled,
    #[msg("The swap returned less than the minimum")]
    SlippageExceeded,
}
