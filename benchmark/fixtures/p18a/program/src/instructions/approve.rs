use anchor_lang::prelude::*;

use crate::{Config, MultisigError, Proposal};

#[derive(Accounts)]
pub struct Approve<'info> {
    pub member: Signer<'info>,
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,
    #[account(mut, seeds = [b"proposal", proposal.id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,
}

pub fn approve(ctx: Context<Approve>) -> Result<()> {
    let member = ctx.accounts.member.key();
    require!(
        ctx.accounts.config.is_member(&member),
        MultisigError::NotAMember
    );
    let proposal = &mut ctx.accounts.proposal;
    let approvals = proposal.approvals as usize;
    let already = proposal.approved_by[..approvals].contains(&member);
    require!(!already, MultisigError::AlreadyApproved);
    proposal.approved_by[approvals] = member;
    proposal.approvals += 1;
    Ok(())
}
