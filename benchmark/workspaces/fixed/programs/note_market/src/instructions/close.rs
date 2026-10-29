use anchor_lang::prelude::*;

use crate::errors::MarketError;
use crate::state::{Position, Vault};

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        close = owner,
        seeds = [Position::SEED, vault.key().as_ref(), owner.key().as_ref()],
        bump = position.bump,
        has_one = owner,
        has_one = vault,
    )]
    pub position: Account<'info, Position>,
}

pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
    require!(ctx.accounts.position.notes == 0, MarketError::PositionNotEmpty);
    Ok(())
}
