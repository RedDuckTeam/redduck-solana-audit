use anchor_lang::prelude::*;

#[error_code]
pub enum MarketError {
    #[msg("amount must be greater than zero")]
    ZeroAmount,
    #[msg("arithmetic overflow")]
    MathOverflow,
    #[msg("position does not have enough notes")]
    InsufficientNotes,
    #[msg("vault does not hold enough assets")]
    InsufficientAssets,
    #[msg("position still holds notes")]
    PositionNotEmpty,
    #[msg("only the vault authority may call this")]
    NotAuthority,
    #[msg("vault has no outstanding notes")]
    EmptyVault,
    #[msg("the position is still within its hold period")]
    StillLocked,
}
