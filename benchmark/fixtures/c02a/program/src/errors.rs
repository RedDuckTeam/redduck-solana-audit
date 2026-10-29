use anchor_lang::prelude::*;

#[error_code]
pub enum PoolError {
    #[msg("amount must be greater than zero")]
    ZeroAmount,
    #[msg("arithmetic overflow")]
    MathOverflow,
    #[msg("not enough stake")]
    InsufficientStake,
    #[msg("the pool has no stake")]
    EmptyPool,
    #[msg("stake account still active")]
    StakeNotEmpty,
    #[msg("the deposit is still within its lock period")]
    StillLocked,
    #[msg("only the pool authority may call this")]
    NotAuthority,
}
