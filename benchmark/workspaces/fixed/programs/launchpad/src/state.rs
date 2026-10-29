use anchor_lang::prelude::*;

/// A fixed-price token sale. Buyers pay with the payment token and are credited sale
/// tokens, which they claim after the sale ends.
#[account]
pub struct Sale {
    pub authority: Pubkey,
    pub payment_mint: Pubkey,
    pub sale_mint: Pubkey,
    pub payment_vault: Pubkey,
    pub sale_vault: Pubkey,
    pub price: u64,
    pub per_wallet_cap: u64,
    pub per_tx_limit: u64,
    pub hard_cap: u64,
    pub bonus_bps: u16,
    pub sold: u64,
    pub claimed_total: u64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub bump: u8,
}

impl Sale {
    pub const SEED: &'static [u8] = b"sale";
    pub const SPACE: usize = 8 + 32 * 5 + 8 * 6 + 2 + 8 * 2 + 1;

    pub fn signer_seeds<'a>(&'a self, mint: &'a Pubkey) -> [&'a [u8]; 3] {
        [Self::SEED, mint.as_ref(), std::slice::from_ref(&self.bump)]
    }
}

/// Per-wallet record of how many sale tokens an address has purchased and whether it
/// has claimed them.
#[account]
pub struct Buyer {
    pub sale: Pubkey,
    pub owner: Pubkey,
    pub purchased: u64,
    pub claimed: bool,
    pub bump: u8,
}

impl Buyer {
    pub const SEED: &'static [u8] = b"buyer";
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 1 + 1;
}
