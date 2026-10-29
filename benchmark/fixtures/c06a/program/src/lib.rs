use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("DURu2KAXJ8BcYTsW1K5CXbzL4uDndfrjEhqoVtqQc2HA");

#[program]
pub mod loot_crate {
    use super::*;

    pub fn init_house(ctx: Context<InitHouse>, args: HouseArgs) -> Result<()> {
        instructions::init_house(ctx, args)
    }

    pub fn fund_prizes(ctx: Context<FundPrizes>, amount: u64) -> Result<()> {
        instructions::fund_prizes(ctx, amount)
    }

    pub fn set_odds(ctx: Context<SetOdds>, odds_bps: u16, prize: u64) -> Result<()> {
        instructions::set_odds(ctx, odds_bps, prize)
    }

    pub fn open_crate(ctx: Context<OpenCrate>) -> Result<()> {
        instructions::open_crate(ctx)
    }

    pub fn reveal(ctx: Context<Reveal>) -> Result<()> {
        instructions::reveal(ctx)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        instructions::cancel(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }
}
