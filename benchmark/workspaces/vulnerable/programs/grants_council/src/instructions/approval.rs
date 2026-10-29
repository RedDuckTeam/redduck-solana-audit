use anchor_lang::prelude::*;

use super::{require_member, require_pending};
use crate::errors::CouncilError;
use crate::events::{ActionProposed, ApprovalChanged};
use crate::state::*;

#[derive(Accounts)]
pub struct ProposeAction<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(mut, seeds = [COUNCIL_SEED, council.creator.as_ref()], bump = council.bump)]
    pub council: Account<'info, Council>,
    #[account(
        init,
        payer = proposer,
        space = 8 + Action::INIT_SPACE,
        seeds = [ACTION_SEED, council.key().as_ref(), &council.action_count.to_le_bytes()],
        bump
    )]
    pub action: Account<'info, Action>,
    pub system_program: Program<'info, System>,
}

/// Stores an instruction for the treasury to sign. The proposer's approval is recorded.
pub fn propose_action(
    ctx: Context<ProposeAction>,
    program_id: Pubkey,
    accounts: Vec<ActionAccount>,
    data: Vec<u8>,
) -> Result<()> {
    let council = &mut ctx.accounts.council;
    let proposer = ctx.accounts.proposer.key();
    let index = require_member(council, &proposer)?;
    require!(
        accounts.len() <= MAX_ACTION_ACCOUNTS && data.len() <= MAX_ACTION_DATA,
        CouncilError::PayloadTooLarge
    );
    require!(program_id != crate::ID, CouncilError::SelfInvocation);

    let action = &mut ctx.accounts.action;
    action.council = council.key();
    action.proposer = proposer;
    action.index = council.action_count;
    action.program_id = program_id;
    action.accounts = accounts;
    action.data = data;
    action.approvals = 1u16 << index;
    action.queued_at = 0;
    action.executed = false;
    action.cancelled = false;
    action.bump = ctx.bumps.action;
    council.action_count = council.action_count.checked_add(1).ok_or(CouncilError::MathOverflow)?;

    emit!(ActionProposed {
        action: action.key(),
        proposer,
        program_id,
        index: action.index,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct MemberVote<'info> {
    pub member: Signer<'info>,
    pub council: Account<'info, Council>,
    #[account(mut, has_one = council)]
    pub action: Account<'info, Action>,
}

pub fn approve_action(ctx: Context<MemberVote>) -> Result<()> {
    let member = ctx.accounts.member.key();
    let index = require_member(&ctx.accounts.council, &member)?;
    let action = &mut ctx.accounts.action;
    require_pending(action)?;
    require!(!action.has_approved(index), CouncilError::AlreadyApproved);

    action.approvals |= 1u16 << index;
    emit!(ApprovalChanged {
        action: action.key(),
        member,
        approved: true,
        approvals: action.approval_count(),
    });
    Ok(())
}

pub fn revoke_approval(ctx: Context<MemberVote>) -> Result<()> {
    let member = ctx.accounts.member.key();
    let index = require_member(&ctx.accounts.council, &member)?;
    let action = &mut ctx.accounts.action;
    require_pending(action)?;
    require!(action.has_approved(index), CouncilError::NotApproved);

    action.approvals &= !(1u16 << index);
    emit!(ApprovalChanged {
        action: action.key(),
        member,
        approved: false,
        approvals: action.approval_count(),
    });
    Ok(())
}
