use anchor_lang::prelude::*;

/// Hub configuration for one gift-card program instance. All cards issued under a hub
/// share one escrow vault and one token mint.
#[account]
pub struct Hub {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub vault: Pubkey,
    pub bump: u8,
}

impl Hub {
    pub const SEED: &'static [u8] = b"hub";
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 1;

    pub fn signer_seeds<'a>(&'a self, mint: &'a Pubkey) -> [&'a [u8]; 3] {
        [Self::SEED, mint.as_ref(), std::slice::from_ref(&self.bump)]
    }
}

/// A single prepaid card. Its balance is spendable credit denominated in the hub's
/// token; the tokens themselves live in the hub vault.
#[account]
pub struct Card {
    pub hub: Pubkey,
    pub owner: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

impl Card {
    pub const SEED: &'static [u8] = b"card";
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 1;
}
