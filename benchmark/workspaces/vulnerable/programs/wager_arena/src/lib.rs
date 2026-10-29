use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod token_ops;

use instructions::*;

declare_id!("BmBZTqedT9KLNDHWUQUhsBSDe425cxsRnYJqfRdAB2ks");

/// Longest a match may stay open before it is considered abandoned.
pub const MAX_MATCH_SECS: i64 = 60 * 60;

#[program]
pub mod wager_arena {
    use super::*;

    pub fn init_arena(ctx: Context<InitArena>, operator: Pubkey) -> Result<()> {
        instructions::init_arena(ctx, operator)
    }

    pub fn open_player(ctx: Context<OpenPlayer>) -> Result<()> {
        instructions::open_player(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn enter_match(ctx: Context<EnterMatch>, stake: u64) -> Result<()> {
        instructions::enter_match(ctx, stake)
    }

    pub fn settle_match(ctx: Context<SettleMatch>, won: bool) -> Result<()> {
        instructions::settle_match(ctx, won)
    }

    pub fn expire_match(ctx: Context<ExpireMatch>) -> Result<()> {
        instructions::expire_match(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }
}
