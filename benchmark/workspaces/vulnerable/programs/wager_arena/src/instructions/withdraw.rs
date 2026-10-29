use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::ArenaError;
use crate::state::{Arena, Player};

#[derive(Accounts)]
pub struct Withdraw<'info> {
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

    #[account(mut, token::mint = arena.token_mint, token::authority = owner)]
    pub owner_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, ArenaError::ZeroAmount);
    let player = &mut ctx.accounts.player;
    require!(!player.in_match, ArenaError::InMatch);
    require!(player.balance >= amount, ArenaError::InsufficientBalance);
    player.balance -= amount;

    ctx.accounts.arena.pay_from_vault(
        ctx.accounts.arena.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.owner_token.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        amount,
    )?;
    emit!(crate::events::Withdrawn {
        arena: ctx.accounts.arena.key(),
        owner: ctx.accounts.owner.key(),
        amount,
    });
    Ok(())
}
