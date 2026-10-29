use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("BF3M3x7i4SPuqGJYjCPAYsZjczYda8MJp9C1J87Z6aY3");

#[program]
pub mod launchpad {
    use super::*;

    pub fn initialize_sale(ctx: Context<InitializeSale>, args: SaleArgs) -> Result<()> {
        instructions::initialize_sale(ctx, args)
    }

    pub fn open_buyer(ctx: Context<OpenBuyer>) -> Result<()> {
        instructions::open_buyer(ctx)
    }

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        instructions::buy(ctx, amount)
    }

    pub fn buy_bundle(ctx: Context<BuyBundle>, amount: u64) -> Result<()> {
        instructions::buy_bundle(ctx, amount)
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        instructions::claim(ctx)
    }

    pub fn withdraw_proceeds(ctx: Context<WithdrawProceeds>, amount: u64) -> Result<()> {
        instructions::withdraw_proceeds(ctx, amount)
    }

    pub fn withdraw_unsold(ctx: Context<WithdrawUnsold>, amount: u64) -> Result<()> {
        instructions::withdraw_unsold(ctx, amount)
    }
}
