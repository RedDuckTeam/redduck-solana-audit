use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;
use state::ActionAccount;

declare_id!("9ZnJZqrmbq7dPKGKYeYKtvR1C6Efgiw6EmmuX93Uhbhc");

/// Grants council. Members propose instructions for the council treasury to sign,
/// approve them up to a threshold and queue them behind a timelock. A guardian
/// can stop any action before it runs.
#[program]
pub mod grants_council {
    use super::*;

    pub fn create_council(
        ctx: Context<CreateCouncil>,
        members: Vec<Pubkey>,
        threshold: u8,
        delay_secs: i64,
        grace_secs: i64,
        guardian: Pubkey,
    ) -> Result<()> {
        instructions::council::create_council(ctx, members, threshold, delay_secs, grace_secs, guardian)
    }

    pub fn propose_action(
        ctx: Context<ProposeAction>,
        program_id: Pubkey,
        accounts: Vec<ActionAccount>,
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::approval::propose_action(ctx, program_id, accounts, data)
    }

    pub fn approve_action(ctx: Context<MemberVote>) -> Result<()> {
        instructions::approval::approve_action(ctx)
    }

    pub fn revoke_approval(ctx: Context<MemberVote>) -> Result<()> {
        instructions::approval::revoke_approval(ctx)
    }

    pub fn queue_action(ctx: Context<QueueAction>) -> Result<()> {
        instructions::timelock::queue_action(ctx)
    }

    pub fn cancel_action(ctx: Context<CancelAction>) -> Result<()> {
        instructions::timelock::cancel_action(ctx)
    }

    pub fn execute_action(ctx: Context<ExecuteAction>) -> Result<()> {
        instructions::execute::execute_action(ctx)
    }
}
