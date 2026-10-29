use anchor_lang::prelude::*;

#[error_code]
pub enum DeskError {
    #[msg("amount must be greater than zero")]
    ZeroAmount,
    #[msg("arithmetic overflow")]
    MathOverflow,
    #[msg("the price pool has no liquidity")]
    NoLiquidity,
    #[msg("requested borrow exceeds the collateral limit")]
    Undercollateralized,
    #[msg("not enough collateral to withdraw")]
    InsufficientCollateral,
    #[msg("the position still has debt")]
    OutstandingDebt,
    #[msg("repayment exceeds the outstanding debt")]
    RepayTooLarge,
    #[msg("slippage: output below minimum")]
    Slippage,
    #[msg("loan-to-value must be between 1 and 10000 bps")]
    BadLtv,
    #[msg("only the market authority may call this")]
    NotAuthority,
}
