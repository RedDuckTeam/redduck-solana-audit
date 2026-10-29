use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("CAP44gWepUsGVbxT7V764u8odUHQHRMB1SYqBFHb354N");

#[program]
pub mod commission_pool {
    use super::*;

    pub fn init_pool(ctx: Context<InitPool>) -> Result<()> {
        instructions::init_pool(ctx)
    }

    pub fn open_stake(ctx: Context<OpenStake>) -> Result<()> {
        instructions::open_stake(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }

    pub fn claim(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim(ctx)
    }

    pub fn distribute(ctx: Context<Distribute>, amount: u64) -> Result<()> {
        instructions::distribute(ctx, amount)
    }

    pub fn close_stake(ctx: Context<CloseStake>) -> Result<()> {
        instructions::close_stake(ctx)
    }
}
