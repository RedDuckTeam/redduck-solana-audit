use anchor_lang::prelude::*;

#[error_code]
pub enum SaleError {
    #[msg("amount must be greater than zero")]
    ZeroAmount,
    #[msg("arithmetic overflow")]
    MathOverflow,
    #[msg("the sale is not open right now")]
    SaleClosed,
    #[msg("the sale has not ended yet")]
    SaleNotEnded,
    #[msg("this purchase exceeds the hard cap")]
    HardCapExceeded,
    #[msg("the sale vault does not hold enough tokens for this purchase")]
    Unbacked,
    #[msg("amount exceeds the unsold balance")]
    ExceedsUnsold,
    #[msg("this purchase exceeds the per-wallet cap")]
    WalletCapExceeded,
    #[msg("this purchase exceeds the per-transaction limit")]
    TxLimitExceeded,
    #[msg("the sale window is invalid")]
    BadWindow,
    #[msg("nothing left to claim")]
    NothingToClaim,
    #[msg("only the sale authority may call this")]
    NotAuthority,
}
