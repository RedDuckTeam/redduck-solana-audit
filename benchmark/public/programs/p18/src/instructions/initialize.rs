use anchor_lang::prelude::*;

use crate::{Config, MultisigError, MAX_MEMBERS};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer = admin, space = 8 + Config::INIT_SPACE, seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,
    /// CHECK: the treasury PDA. It holds nothing itself; token accounts are owned by it.
    #[account(seeds = [b"vault"], bump)]
    pub vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, members: Vec<Pubkey>, threshold: u8) -> Result<()> {
    require!(
        !members.is_empty()
            && members.len() <= MAX_MEMBERS
            && threshold >= 1
            && (threshold as usize) <= members.len(),
        MultisigError::InvalidMembers
    );
    let mut stored = [Pubkey::default(); MAX_MEMBERS];
    stored[..members.len()].copy_from_slice(&members);
    ctx.accounts.config.set_inner(Config {
        admin: ctx.accounts.admin.key(),
        members: stored,
        member_count: members.len() as u8,
        threshold,
        vault_bump: ctx.bumps.vault,
        bump: ctx.bumps.config,
    });
    Ok(())
}
