use anchor_lang::prelude::*;

use crate::state::{Market, Position};

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = owner,
        space = Position::SPACE,
        seeds = [Position::SEED, market.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub position: Account<'info, Position>,

    pub system_program: Program<'info, System>,
}

pub fn open_position(ctx: Context<OpenPosition>) -> Result<()> {
    let position = &mut ctx.accounts.position;
    position.market = ctx.accounts.market.key();
    position.owner = ctx.accounts.owner.key();
    position.collateral = 0;
    position.debt = 0;
    position.bump = ctx.bumps.position;
    Ok(())
}
