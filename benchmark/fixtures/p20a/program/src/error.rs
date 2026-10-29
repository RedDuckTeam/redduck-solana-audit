use anchor_lang::prelude::*;

#[error_code]
pub enum PoolError {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Not enough staked")]
    InsufficientStake,
    #[msg("Nothing is staked, the reward has nobody to go to")]
    NothingStaked,
    #[msg("A remaining account is not a staker of this pool")]
    NotAStaker,
}
