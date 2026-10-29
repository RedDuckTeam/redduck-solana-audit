use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod pricing;
pub mod state;

use instructions::*;
use state::MarketParams;

declare_id!("3SDQyw9p4i9dHGxLAVf6JmVUP9q5QDCnGjsfkkxAVkXp");

/// Lenders supply the debt token for pool shares; borrowers lock a staking token and borrow against it.
#[program]
pub mod stake_credit {
    use super::*;

    pub fn init_market(ctx: Context<InitMarket>, params: MarketParams) -> Result<()> {
        instructions::market::init_market(ctx, params)
    }

    pub fn open_account(ctx: Context<OpenAccount>) -> Result<()> {
        instructions::market::open_account(ctx)
    }

    pub fn supply(ctx: Context<PoolAction>, amount: u64) -> Result<()> {
        instructions::lender::supply(ctx, amount)
    }

    pub fn withdraw_supply(ctx: Context<PoolAction>, shares: u64) -> Result<()> {
        instructions::lender::withdraw_supply(ctx, shares)
    }

    /// Locks `deposit` collateral, then draws `amount`. Either may be zero.
    pub fn borrow(ctx: Context<PositionAction>, deposit: u64, amount: u64) -> Result<()> {
        instructions::borrower::borrow(ctx, deposit, amount)
    }

    /// Repays up to `amount`, then releases `withdraw` collateral. Either may be zero.
    pub fn repay(ctx: Context<PositionAction>, amount: u64, withdraw: u64) -> Result<()> {
        instructions::borrower::repay(ctx, amount, withdraw)
    }

    pub fn liquidate(ctx: Context<Liquidate>, max_repay: u64) -> Result<()> {
        instructions::liquidate::liquidate(ctx, max_repay)
    }
}
