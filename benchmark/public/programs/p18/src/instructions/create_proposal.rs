use anchor_lang::prelude::*;

use crate::{Config, MultisigError, Proposal, StoredMeta, MAX_DATA, MAX_MEMBERS, MAX_METAS};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = proposer,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal", id.to_le_bytes().as_ref()],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,
    pub system_program: Program<'info, System>,
}

pub fn create_proposal(
    ctx: Context<CreateProposal>,
    id: u64,
    program_id: Pubkey,
    metas: Vec<StoredMeta>,
    data: Vec<u8>,
    expires_at: i64,
) -> Result<()> {
    require!(
        ctx.accounts.config.is_member(&ctx.accounts.proposer.key()),
        MultisigError::NotAMember
    );
    require!(
        metas.len() <= MAX_METAS && data.len() <= MAX_DATA,
        MultisigError::ProposalTooBig
    );
    ctx.accounts.proposal.set_inner(Proposal {
        id,
        proposer: ctx.accounts.proposer.key(),
        program_id,
        metas,
        data,
        approvals: 0,
        approved_by: [Pubkey::default(); MAX_MEMBERS],
        executed: false,
        expires_at,
        bump: ctx.bumps.proposal,
    });
    Ok(())
}
