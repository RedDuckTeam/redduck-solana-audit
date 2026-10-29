use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("CSu8oFRh5eMXqj7n3MMGQbFcFRj8e89sa3vApigj9ynk");

/// Prepaid card balances. Card holders top up a shared vault, the card
/// processor settles cleared purchases from it, and withdrawals back to the
/// holder are co-signed by the processor once no authorizations are pending.
#[program]
pub mod spend_card {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::admin::initialize(ctx)
    }

    pub fn queue_config(ctx: Context<QueueConfig>) -> Result<()> {
        instructions::admin::queue_config(ctx)
    }

    pub fn apply_config(ctx: Context<ApplyConfig>) -> Result<()> {
        instructions::admin::apply_config(ctx)
    }

    pub fn open_card(ctx: Context<OpenCard>) -> Result<()> {
        instructions::card::open_card(ctx)
    }

    pub fn top_up(ctx: Context<TopUp>, amount: u64) -> Result<()> {
        instructions::card::top_up(ctx, amount)
    }

    pub fn request_withdrawal(ctx: Context<RequestWithdrawal>, amount: u64) -> Result<()> {
        instructions::card::request_withdrawal(ctx, amount)
    }

    pub fn complete_withdrawal(ctx: Context<CompleteWithdrawal>) -> Result<()> {
        instructions::card::complete_withdrawal(ctx)
    }

    pub fn settle_spend(ctx: Context<SettleSpend>, auth_id: u64, amount: u64) -> Result<()> {
        instructions::processor::settle_spend(ctx, auth_id, amount)
    }
}
