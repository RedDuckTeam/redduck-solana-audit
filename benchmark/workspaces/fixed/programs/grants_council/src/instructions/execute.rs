use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke_signed;

use crate::errors::CouncilError;
use crate::events::ActionExecuted;
use crate::state::*;

#[derive(Accounts)]
pub struct ExecuteAction<'info> {
    pub executor: Signer<'info>,
    pub council: Account<'info, Council>,
    #[account(mut, has_one = council)]
    pub action: Account<'info, Action>,
}

/// Signs the action's instruction with the treasury and sends it.
pub fn execute_action(ctx: Context<ExecuteAction>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let council = &ctx.accounts.council;
    let action = &mut ctx.accounts.action;

    require!(action.queued_at > 0, CouncilError::NotQueued);
    require!(!action.cancelled, CouncilError::ActionCancelled);
    require!(!action.executed, CouncilError::AlreadyExecuted);
    let ready_at = action
        .queued_at
        .checked_add(council.delay_secs)
        .ok_or(CouncilError::MathOverflow)?;
    require!(now >= ready_at, CouncilError::DelayNotElapsed);
    require!(now <= ready_at.saturating_add(council.grace_secs), CouncilError::ActionExpired);

    let ix = stored_instruction(action, ctx.remaining_accounts)?;
    action.executed = true;

    let council_key = council.key();
    let bump = [council.treasury_bump];
    let seeds: &[&[u8]] = &[TREASURY_SEED, council_key.as_ref(), &bump];
    invoke_signed(&ix, ctx.remaining_accounts, &[seeds])?;

    emit!(ActionExecuted {
        action: action.key(),
        executor: ctx.accounts.executor.key(),
    });
    Ok(())
}

/// Rebuilds the stored instruction. `remaining` is the target program followed by
/// the stored accounts in order; signer and writable flags come from the stored list.
fn stored_instruction(action: &Action, remaining: &[AccountInfo]) -> Result<Instruction> {
    require!(remaining.len() == action.accounts.len() + 1, CouncilError::AccountMismatch);
    require_keys_eq!(*remaining[0].key, action.program_id, CouncilError::ProgramMismatch);

    let mut metas = Vec::with_capacity(action.accounts.len());
    for (stored, info) in action.accounts.iter().zip(&remaining[1..]) {
        require_keys_eq!(*info.key, stored.pubkey, CouncilError::AccountMismatch);
        metas.push(if stored.is_writable {
            AccountMeta::new(stored.pubkey, stored.is_signer)
        } else {
            AccountMeta::new_readonly(stored.pubkey, stored.is_signer)
        });
    }

    Ok(Instruction {
        program_id: action.program_id,
        accounts: metas,
        data: action.data.clone(),
    })
}
