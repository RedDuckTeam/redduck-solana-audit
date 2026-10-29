use anchor_lang::prelude::*;

/// Minimum time a deposit must be held before its notes can be withdrawn or redeemed.
pub const HOLD_SECS: i64 = 60 * 60;

/// A yield-bearing vault. Depositors receive notes that represent a share of the
/// pooled assets. As yield is added the assets per note grows, so one note is
/// generally worth more than one asset once the vault has been running.
#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub asset_mint: Pubkey,
    pub vault_token: Pubkey,
    pub total_notes: u64,
    pub total_assets: u64,
    pub bump: u8,
}

impl Vault {
    pub const SEED: &'static [u8] = b"vault";
    pub const SPACE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 1;

    pub fn signer_seeds<'a>(&'a self, mint: &'a Pubkey) -> [&'a [u8]; 3] {
        [Self::SEED, mint.as_ref(), std::slice::from_ref(&self.bump)]
    }
}

/// A single depositor's holding of notes in a vault.
#[account]
pub struct Position {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub notes: u64,
    pub unlock_ts: i64,
    pub bump: u8,
}

impl Position {
    pub const SEED: &'static [u8] = b"position";
    pub const SPACE: usize = 8 + 32 + 32 + 8 + 8 + 1;
}
