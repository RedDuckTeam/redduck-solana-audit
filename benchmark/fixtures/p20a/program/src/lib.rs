use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("CdBnueUNxmd2eB2uTthggnoKXXnq71GzWBPU3TCjBADi");

#[program]
pub mod stake_rewards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        instructions::unstake(ctx, amount)
    }

    pub fn add_rewards<'info>(
        ctx: Context<'_, '_, 'info, 'info, AddRewards<'info>>,
        amount: u64,
    ) -> Result<()> {
        instructions::add_rewards(ctx, amount)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        instructions::claim(ctx)
    }
}
