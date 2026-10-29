use anchor_lang::prelude::*;

pub mod ed25519;
pub mod error;
pub mod instructions;
pub mod state;
pub mod sysvar;

pub use ed25519::*;
pub use error::*;
pub use instructions::*;
pub use state::*;
pub use sysvar::*;

declare_id!("GfxMGdUbRtXDtG8fx2HGLoHhDjpa9syxZPvtefw7yksq");

#[program]
pub mod payout_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn create_allowlist(ctx: Context<CreateAllowlist>, keys: Vec<Pubkey>) -> Result<()> {
        instructions::create_allowlist(ctx, keys)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }
}
