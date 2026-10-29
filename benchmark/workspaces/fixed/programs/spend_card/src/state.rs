use anchor_lang::prelude::*;

pub const CONFIG_SEED: &[u8] = b"config";
pub const VAULT_SEED: &[u8] = b"vault";
pub const CARD_SEED: &[u8] = b"card";

/// Seconds after a withdrawal request when the owner may complete it alone.
pub const EXIT_DELAY: i64 = 48 * 60 * 60;

/// Seconds between queuing a configuration change and applying it.
pub const CONFIG_DELAY: i64 = 72 * 60 * 60;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    /// Token account that receives settled card purchases.
    pub settlement: Pubkey,
    pub pending_settlement: Pubkey,
    /// Signing key of the card processor's settlement service.
    pub processor: Pubkey,
    pub pending_processor: Pubkey,
    /// Unix time after which the queued change may be applied; 0 when nothing is queued.
    pub change_eta: i64,
    /// Sum of all card balances held in the vault.
    pub total_balance: u64,
    pub bump: u8,
}

impl Config {
    pub fn signer_seeds<'a>(&'a self, bump: &'a [u8; 1]) -> [&'a [u8]; 3] {
        [CONFIG_SEED, self.admin.as_ref(), bump]
    }
}

#[account]
#[derive(InitSpace)]
pub struct Card {
    pub config: Pubkey,
    pub owner: Pubkey,
    pub balance: u64,
    /// Lifetime total of settled purchases.
    pub spent: u64,
    /// Highest authorization id settled so far; ids increase per card.
    pub last_auth_id: u64,
    pub withdrawal_amount: u64,
    pub withdrawal_requested_at: i64,
    pub bump: u8,
}

impl Card {
    pub fn debit(&mut self, amount: u64) -> Result<()> {
        self.balance = self
            .balance
            .checked_sub(amount)
            .ok_or(error!(crate::errors::CardError::InsufficientBalance))?;
        Ok(())
    }

    pub fn clear_withdrawal(&mut self) {
        self.withdrawal_amount = 0;
        self.withdrawal_requested_at = 0;
    }
}
