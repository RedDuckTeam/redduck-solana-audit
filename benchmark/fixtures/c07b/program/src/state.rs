use anchor_lang::prelude::*;

/// A match arena. Players deposit a token and lock a stake to play ranked matches that
/// are refereed off-chain. The operator reports results; the stake is a refundable bond.
#[account]
pub struct Arena {
    pub authority: Pubkey,
    pub operator: Pubkey,
    pub token_mint: Pubkey,
    pub vault: Pubkey,
    pub executor: Pubkey,
    pub bump: u8,
}

impl Arena {
    pub const SEED: &'static [u8] = b"arena";
    pub const SPACE: usize = 8 + 32 * 5 + 1;

    pub fn signer_seeds<'a>(&'a self, mint: &'a Pubkey) -> [&'a [u8]; 3] {
        [Self::SEED, mint.as_ref(), std::slice::from_ref(&self.bump)]
    }
}

/// A player's funds in an arena. `balance` is the free balance; `staked` is locked while
/// a match is open.
#[account]
pub struct Player {
    pub arena: Pubkey,
    pub owner: Pubkey,
    pub balance: u64,
    pub staked: u64,
    pub in_match: bool,
    pub match_deadline: i64,
    pub wins: u32,
    pub bump: u8,
}

impl Player {
    pub const SEED: &'static [u8] = b"player";
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 8 + 1 + 8 + 4 + 1;
}
