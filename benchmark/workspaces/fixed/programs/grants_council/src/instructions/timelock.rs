use anchor_lang::prelude::*;

use super::{require_member, require_pending};
use crate::errors::CouncilError;
use crate::events::{ActionCancelled, ActionQueued};
use crate::state::*;

#[derive(Accounts)]
pub struct QueueAction<'info> {
    pub member: Signer<'info>,
    pub council: Account<'info, Council>,
    #[account(mut, has_one = council)]
    pub action: Account<'info, Action>,
}

/// Starts the timelock for an action that has reached the approval threshold.
pub fn queue_action(ctx: Context<QueueAction>) -> Result<()> {
    let council = &ctx.accounts.council;
    require_member(council, &ctx.accounts.member.key())?;

    let action = &mut ctx.accounts.action;
    require_pending(action)?;
    require!(
        action.approval_count() >= u32::from(council.threshold),
        CouncilError::ThresholdNotMet
    );

    let now = Clock::get()?.unix_timestamp;
    action.queued_at = now;
    emit!(ActionQueued {
        action: action.key(),
        ready_at: now.checked_add(council.delay_secs).ok_or(CouncilError::MathOverflow)?,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CancelAction<'info> {
    pub authority: Signer<'info>,
    pub council: Account<'info, Council>,
    #[account(mut, has_one = council)]
    pub action: Account<'info, Action>,
}

/// The guardian may cancel any action that has not run. A proposer may withdraw
/// its own action until it is queued.
pub fn cancel_action(ctx: Context<CancelAction>) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    let council = &ctx.accounts.council;
    let action = &mut ctx.accounts.action;
    require!(!action.executed, CouncilError::AlreadyExecuted);
    require!(!action.cancelled, CouncilError::ActionCancelled);

    let by_guardian = authority == council.guardian;
    let by_proposer = authority == action.proposer && action.queued_at == 0;
    require!(by_guardian || by_proposer, CouncilError::CancelNotAllowed);

    action.cancelled = true;
    emit!(ActionCancelled { action: action.key(), by: authority });
    Ok(())
}
