use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod quote;
pub mod state;

use instructions::*;
use state::DeskSettings;

declare_id!("5FNKvRbHFoFToHKnPyDgDQBYyTH9f7JWDbZTaJ9ckzzU");

/// FX corridor desk. Converts between two stable tokens at the rate published by an
/// external FX publisher, less a fee. Liquidity on both sides is supplied by the desk admin.
#[program]
pub mod fx_corridor {
    use super::*;

    pub fn initialize_desk(ctx: Context<InitializeDesk>, settings: DeskSettings) -> Result<()> {
        instructions::admin::initialize_desk(ctx, settings)
    }

    pub fn update_settings(ctx: Context<UpdateSettings>, settings: DeskSettings) -> Result<()> {
        instructions::admin::update_settings(ctx, settings)
    }

    pub fn set_pause(ctx: Context<SetPause>, duration_secs: i64) -> Result<()> {
        instructions::admin::set_pause(ctx, duration_secs)
    }

    pub fn provide_liquidity(ctx: Context<MoveLiquidity>, amount: u64) -> Result<()> {
        instructions::liquidity::provide_liquidity(ctx, amount)
    }

    pub fn withdraw_liquidity(ctx: Context<MoveLiquidity>, amount: u64) -> Result<()> {
        instructions::liquidity::withdraw_liquidity(ctx, amount)
    }

    pub fn post_rate(ctx: Context<PostRate>, rate: u64, published_at: i64) -> Result<()> {
        instructions::rate::post_rate(ctx, rate, published_at)
    }

    pub fn convert(ctx: Context<Convert>, amount_in: u64, min_out: u64, sell_base: bool) -> Result<()> {
        instructions::convert::convert(ctx, amount_in, min_out, sell_base)
    }
}
