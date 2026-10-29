use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("4snwYzHWi7ffbzQmRGwkd7DSHgYPduBqVXgmfFfFcckg");

#[program]
pub mod lending_desk {
    use super::*;

    pub fn init_market(ctx: Context<InitMarket>, ltv_bps: u16) -> Result<()> {
        instructions::init_market(ctx, ltv_bps)
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, base_amount: u64, quote_amount: u64) -> Result<()> {
        instructions::add_liquidity(ctx, base_amount, quote_amount)
    }

    pub fn withdraw_liquidity(ctx: Context<WithdrawLiquidity>, amount: u64) -> Result<()> {
        instructions::withdraw_liquidity(ctx, amount)
    }

    pub fn swap_base_for_quote(ctx: Context<Swap>, base_in: u64, min_quote_out: u64) -> Result<()> {
        instructions::swap_base_for_quote(ctx, base_in, min_quote_out)
    }

    pub fn open_position(ctx: Context<OpenPosition>) -> Result<()> {
        instructions::open_position(ctx)
    }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        instructions::deposit_collateral(ctx, amount)
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        instructions::borrow(ctx, amount)
    }

    pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
        instructions::repay(ctx, amount)
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> Result<()> {
        instructions::withdraw_collateral(ctx, amount)
    }
}
