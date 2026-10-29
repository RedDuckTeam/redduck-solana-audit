use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::*;

declare_id!("BhUM6cVe5a98qXeYrnJiEv3DLBkp81oMpx62myP6XYLC");

/// Seasonal lockup rewards. Holders lock the stake token for a chosen period and
/// receive a weight that grows with the remaining lock. When a season ends, its
/// reward budget is split across positions in proportion to weight.
#[program]
pub mod season_locker {
    use super::*;

    pub fn create_season(ctx: Context<CreateSeason>, season_id: u64, starts_at: i64, ends_at: i64, reward_amount: u64) -> Result<()> {
        instructions::season::create_season(ctx, season_id, starts_at, ends_at, reward_amount)
    }

    pub fn open_position(ctx: Context<OpenPosition>, position_id: u64, amount: u64, lock_secs: i64) -> Result<()> {
        instructions::position::open_position(ctx, position_id, amount, lock_secs)
    }

    pub fn add_stake(ctx: Context<AddStake>, amount: u64) -> Result<()> {
        instructions::position::add_stake(ctx, amount)
    }

    pub fn extend_lock(ctx: Context<ExtendLock>, new_lock_end: i64) -> Result<()> {
        instructions::position::extend_lock(ctx, new_lock_end)
    }

    pub fn merge_positions(ctx: Context<MergePositions>) -> Result<()> {
        instructions::position::merge_positions(ctx)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::exit::withdraw(ctx)
    }

    pub fn finalize_season(ctx: Context<FinalizeSeason>) -> Result<()> {
        instructions::season::finalize_season(ctx)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::exit::claim_reward(ctx)
    }
}
