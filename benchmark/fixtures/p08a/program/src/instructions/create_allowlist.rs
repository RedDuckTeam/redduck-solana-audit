use anchor_lang::prelude::*;

use crate::{Allowlist, VaultError, MAX_KEYS};

/// Creates an allowlist of signing keys.
#[derive(Accounts)]
pub struct CreateAllowlist<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + Allowlist::INIT_SPACE)]
    pub allowlist: Account<'info, Allowlist>,
    pub system_program: Program<'info, System>,
}

pub fn create_allowlist(ctx: Context<CreateAllowlist>, keys: Vec<Pubkey>) -> Result<()> {
    require!(keys.len() <= MAX_KEYS, VaultError::TooManyKeys);
    ctx.accounts.allowlist.set_inner(Allowlist {
        authority: ctx.accounts.authority.key(),
        keys,
    });
    Ok(())
}
