use anchor_lang::prelude::*;

pub mod ed25519;
pub mod error;
pub mod instructions;
pub mod state;

pub use ed25519::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("2egvSswNkKRYTDEcK9hKkN1Wz5iSDCLQCijY1XjJ6aVY");

#[program]
pub mod device_rewards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, oracle: Pubkey) -> Result<()> {
        instructions::initialize(ctx, oracle)
    }

    pub fn register_device(ctx: Context<RegisterDevice>) -> Result<()> {
        instructions::register_device(ctx)
    }

    pub fn claim(ctx: Context<Claim>, amount: u64, nonce: u64) -> Result<()> {
        instructions::claim(ctx, amount, nonce)
    }
}
