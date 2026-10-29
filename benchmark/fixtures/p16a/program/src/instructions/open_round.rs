use anchor_lang::prelude::*;

use crate::{Config, Round};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct OpenRound<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds = [b"config"], bump = config.bump, has_one = admin)]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        space = 8 + Round::INIT_SPACE,
        seeds = [b"round", id.to_le_bytes().as_ref()],
        bump,
    )]
    pub round: Account<'info, Round>,
    pub system_program: Program<'info, System>,
}

pub fn open_round(ctx: Context<OpenRound>, id: u64, deadline: i64) -> Result<()> {
    ctx.accounts.round.set_inner(Round {
        id,
        deadline,
        pot: 0,
        settled: false,
        bump: ctx.bumps.round,
    });
    Ok(())
}
