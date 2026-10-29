use anchor_lang::prelude::*;

/// A loot-crate house. Players pay `cost` to open a crate; a crate pays `prize` on a
/// win, which happens with probability `odds_bps / 10000`. The prize pool and the
/// collected costs share one vault.
#[account]
pub struct House {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub vault: Pubkey,
    pub cost: u64,
    pub prize: u64,
    pub reserved: u64,
    pub odds_bps: u16,
    pub bump: u8,
}

impl House {
    pub const SEED: &'static [u8] = b"house";
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 2 + 1;

    pub fn signer_seeds<'a>(&'a self, mint: &'a Pubkey) -> [&'a [u8]; 3] {
        [Self::SEED, mint.as_ref(), std::slice::from_ref(&self.bump)]
    }
}

/// An open crate waiting to be revealed. The outcome is fixed by the hash of
/// `reveal_slot`, which is not known when the crate is opened.
#[account]
pub struct Commitment {
    pub house: Pubkey,
    pub player: Pubkey,
    pub reveal_slot: u64,
    pub paid: u64,
    pub prize: u64,
    pub odds_bps: u16,
    pub bump: u8,
}

impl Commitment {
    pub const SEED: &'static [u8] = b"crate";
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 2 + 1;
}
