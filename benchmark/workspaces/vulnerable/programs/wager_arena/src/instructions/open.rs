use anchor_lang::prelude::*;

use crate::state::{Arena, Player};

#[derive(Accounts)]
pub struct OpenPlayer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub arena: Account<'info, Arena>,

    #[account(
        init,
        payer = owner,
        space = Player::SPACE,
        seeds = [Player::SEED, arena.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub player: Account<'info, Player>,

    pub system_program: Program<'info, System>,
}

pub fn open_player(ctx: Context<OpenPlayer>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    player.arena = ctx.accounts.arena.key();
    player.owner = ctx.accounts.owner.key();
    player.balance = 0;
    player.staked = 0;
    player.in_match = false;
    player.match_deadline = 0;
    player.wins = 0;
    player.bump = ctx.bumps.player;
    Ok(())
}
