use anchor_lang::prelude::*;

use crate::state::{Position, Vault};

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = owner,
        space = Position::SPACE,
        seeds = [Position::SEED, vault.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub position: Account<'info, Position>,

    pub system_program: Program<'info, System>,
}

pub fn open_position(ctx: Context<OpenPosition>) -> Result<()> {
    let position = &mut ctx.accounts.position;
    position.owner = ctx.accounts.owner.key();
    position.vault = ctx.accounts.vault.key();
    position.notes = 0;
    position.unlock_ts = 0;
    position.bump = ctx.bumps.position;
    Ok(())
}
