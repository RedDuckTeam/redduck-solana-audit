use anchor_lang::prelude::*;

#[error_code]
pub enum ArenaError {
    #[msg("amount must be greater than zero")]
    ZeroAmount,
    #[msg("arithmetic overflow")]
    MathOverflow,
    #[msg("not enough free balance")]
    InsufficientBalance,
    #[msg("the player is in an active match")]
    InMatch,
    #[msg("the player is not in a match")]
    NotInMatch,
    #[msg("the match deadline has not been reached")]
    DeadlineNotReached,
    #[msg("only the arena operator may call this")]
    NotOperator,
}
