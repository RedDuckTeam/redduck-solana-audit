use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::ArenaError;
use crate::state::{Arena, Player};

#[derive(Accounts)]
pub struct Deposit<'info> {
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

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, ArenaError::ZeroAmount);
    crate::token_ops::move_tokens(
        ctx.accounts.owner_token.to_account_info(),
        ctx.accounts.vault.to_account_info(),
        ctx.accounts.owner.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        amount,
    )?;
    let player = &mut ctx.accounts.player;
    player.balance = player.balance.checked_add(amount).ok_or(ArenaError::MathOverflow)?;
    emit!(crate::events::Deposited {
        arena: ctx.accounts.arena.key(),
        owner: ctx.accounts.owner.key(),
        amount,
    });
    Ok(())
}
