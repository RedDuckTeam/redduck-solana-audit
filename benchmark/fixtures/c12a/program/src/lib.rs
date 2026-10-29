use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("F39E4Pp7P8TanUDM57upYWiwzmR5eMchWXu6CXP9gNsh");

/// Group purchases. A merchant offers units at a fixed price with a minimum
/// that must be reached by a deadline. Buyers pay into the deal's escrow; the
/// merchant collects if the minimum is reached, otherwise every buyer is refunded.
#[program]
pub mod group_buy {
    use super::*;

    pub fn register_merchant(ctx: Context<RegisterMerchant>) -> Result<()> {
        instructions::merchant::register_merchant(ctx)
    }

    pub fn create_deal(ctx: Context<CreateDeal>, params: DealParams) -> Result<()> {
        instructions::merchant::create_deal(ctx, params)
    }

    pub fn cancel_deal(ctx: Context<CancelDeal>) -> Result<()> {
        instructions::merchant::cancel_deal(ctx)
    }

    pub fn claim_proceeds(ctx: Context<ClaimProceeds>) -> Result<()> {
        instructions::merchant::claim_proceeds(ctx)
    }

    pub fn join_deal(ctx: Context<JoinDeal>) -> Result<()> {
        instructions::buyer::join_deal(ctx)
    }

    pub fn commit(ctx: Context<Commit>, units: u32) -> Result<()> {
        instructions::buyer::commit(ctx, units)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::buyer::refund(ctx)
    }
}
