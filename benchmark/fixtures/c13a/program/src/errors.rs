use anchor_lang::prelude::*;

#[error_code]
pub enum CreditError {
    ZeroAmount,
    MathOverflow,
    #[msg("Market parameters are out of range")]
    InvalidParams,
    #[msg("Feed account is not the one configured for this market")]
    InvalidFeed,
    #[msg("Feed price is not usable")]
    BadPrice,
    #[msg("Feed price is too old")]
    StalePrice,
    #[msg("Not enough liquidity in the pool")]
    InsufficientLiquidity,
    InsufficientShares,
    InsufficientCollateral,
    #[msg("Position would exceed the loan-to-value limit")]
    LtvExceeded,
    #[msg("Position is healthy")]
    PositionHealthy,
}
