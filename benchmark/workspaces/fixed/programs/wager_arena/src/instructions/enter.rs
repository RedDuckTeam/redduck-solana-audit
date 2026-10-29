use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::ArenaError;
use crate::state::{Arena, Player};
use crate::MAX_MATCH_SECS;

#[derive(Accounts)]
pub struct EnterMatch<'info> {
    pub owner: Signer<'info>,

    #[account(seeds = [Arena::SEED, arena.token_mint.as_ref()], bump = arena.bump)]
    pub arena: Account<'info, Arena>,

    #[account(
        mut,
        seeds = [Player::SEED, arena.key().as_ref(), owner.key().as_ref()],
        bump = player.bump,
        has_one = owner,
        has_one = arena,
    )]
    pub player: Account<'info, Player>,

    #[account(mut, address = arena.vault)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut, address = arena.executor)]
    pub executor: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn enter_match(ctx: Context<EnterMatch>, stake: u64) -> Result<()> {
    require!(stake > 0, ArenaError::ZeroAmount);
    let now = Clock::get()?.unix_timestamp;
    let player = &mut ctx.accounts.player;
    require!(!player.in_match, ArenaError::InMatch);
    require!(player.balance >= stake, ArenaError::InsufficientBalance);

    player.balance -= stake;
    player.staked = stake;
    player.in_match = true;
    player.match_deadline = now.checked_add(MAX_MATCH_SECS).ok_or(ArenaError::MathOverflow)?;

    emit!(crate::events::MatchEntered {
        arena: ctx.accounts.arena.key(),
        owner: ctx.accounts.owner.key(),
        stake,
        deadline: ctx.accounts.player.match_deadline,
    });
    Ok(())
}
