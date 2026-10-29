use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::ArenaError;
use crate::state::{Arena, Player};

#[derive(Accounts)]
pub struct SettleMatch<'info> {
    pub operator: Signer<'info>,

    #[account(
        seeds = [Arena::SEED, arena.token_mint.as_ref()],
        bump = arena.bump,
        constraint = arena.operator == operator.key() @ ArenaError::NotOperator,
    )]
    pub arena: Account<'info, Arena>,

    #[account(mut, has_one = arena)]
    pub player: Account<'info, Player>,

    #[account(mut, address = arena.vault)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut, address = arena.executor)]
    pub executor: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn settle_match(ctx: Context<SettleMatch>, won: bool) -> Result<()> {
    require!(ctx.accounts.player.in_match, ArenaError::NotInMatch);
    let stake = ctx.accounts.player.staked;


    let player = &mut ctx.accounts.player;
    player.balance = player.balance.checked_add(stake).ok_or(ArenaError::MathOverflow)?;
    player.staked = 0;
    player.in_match = false;
    if won {
        player.wins = player.wins.checked_add(1).ok_or(ArenaError::MathOverflow)?;
    }
    Ok(())
}
