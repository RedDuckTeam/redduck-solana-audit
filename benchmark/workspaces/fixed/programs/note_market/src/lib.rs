use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("EKgHChHvry5DxYcbiS8YFJDUy8BVHzjYiLGWTLhNmZaz");

#[program]
pub mod note_market {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::initialize_vault(ctx)
    }

    pub fn open_position(ctx: Context<OpenPosition>) -> Result<()> {
        instructions::open_position(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, assets: u64) -> Result<()> {
        instructions::deposit(ctx, assets)
    }

    pub fn accrue_yield(ctx: Context<AccrueYield>, amount: u64) -> Result<()> {
        instructions::accrue_yield(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, assets: u64) -> Result<()> {
        instructions::withdraw(ctx, assets)
    }

    pub fn redeem(ctx: Context<Redeem>, notes: u64) -> Result<()> {
        instructions::redeem(ctx, notes)
    }

    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        instructions::close_position(ctx)
    }
}
