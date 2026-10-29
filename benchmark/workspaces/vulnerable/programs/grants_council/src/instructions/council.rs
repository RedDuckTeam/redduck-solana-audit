use anchor_lang::prelude::*;

use crate::errors::CouncilError;
use crate::events::CouncilCreated;
use crate::state::*;

#[derive(Accounts)]
pub struct CreateCouncil<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = 8 + Council::INIT_SPACE,
        seeds = [COUNCIL_SEED, creator.key().as_ref()],
        bump
    )]
    pub council: Account<'info, Council>,
    /// CHECK: data-less PDA that holds funds and signs approved actions.
    #[account(seeds = [TREASURY_SEED, council.key().as_ref()], bump)]
    pub treasury: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_council(
    ctx: Context<CreateCouncil>,
    members: Vec<Pubkey>,
    threshold: u8,
    delay_secs: i64,
    grace_secs: i64,
    guardian: Pubkey,
) -> Result<()> {
    require!(!members.is_empty() && members.len() <= MAX_MEMBERS, CouncilError::InvalidMembers);
    for (i, member) in members.iter().enumerate() {
        require!(!members[..i].contains(member), CouncilError::InvalidMembers);
    }
    require!(
        threshold >= 1 && threshold as usize <= members.len(),
        CouncilError::InvalidThreshold
    );
    require!(delay_secs >= MIN_DELAY_SECS && grace_secs > 0, CouncilError::InvalidTimelock);

    let council = &mut ctx.accounts.council;
    council.creator = ctx.accounts.creator.key();
    council.guardian = guardian;
    council.members = members;
    council.threshold = threshold;
    council.delay_secs = delay_secs;
    council.grace_secs = grace_secs;
    council.action_count = 0;
    council.bump = ctx.bumps.council;
    council.treasury_bump = ctx.bumps.treasury;

    emit!(CouncilCreated {
        council: council.key(),
        guardian,
        members: council.members.len() as u8,
        threshold,
    });
    Ok(())
}
