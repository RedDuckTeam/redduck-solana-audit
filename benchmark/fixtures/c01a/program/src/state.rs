use anchor_lang::prelude::*;

/// Scale applied to the base/quote price so it can be held as an integer.
pub const PRICE_SCALE: u128 = 1_000_000;

/// A lending market with a small constant-product price pool.
#[account]
pub struct Market {
    pub authority: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub pool_base_vault: Pubkey,
    pub pool_quote_vault: Pubkey,
    pub collateral_vault: Pubkey,
    pub borrow_vault: Pubkey,
    pub reserve_base: u64,
    pub reserve_quote: u64,
    pub ltv_bps: u16,
    pub bump: u8,
}

impl Market {
    pub const SEED: &'static [u8] = b"market";
    pub const SPACE: usize = 8 + 32 * 7 + 8 + 8 + 2 + 1;

    pub fn signer_seeds<'a>(&'a self, base: &'a Pubkey, quote: &'a Pubkey) -> [&'a [u8]; 4] {
        [Self::SEED, base.as_ref(), quote.as_ref(), std::slice::from_ref(&self.bump)]
    }
}

/// A borrower's collateral and debt in a market.
#[account]
pub struct Position {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub collateral: u64,
    pub debt: u64,
    pub bump: u8,
}

impl Position {
    pub const SEED: &'static [u8] = b"position";
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 8 + 1;
}
