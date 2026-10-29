use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::ArenaError;
use crate::state::{Arena, Player};

#[derive(Accounts)]
pub struct ExpireMatch<'info> {
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

pub fn expire_match(ctx: Context<ExpireMatch>) -> Result<()> {
    require!(ctx.accounts.player.in_match, ArenaError::NotInMatch);
    require!(
        Clock::get()?.unix_timestamp >= ctx.accounts.player.match_deadline,
        ArenaError::DeadlineNotReached
    );
    let stake = ctx.accounts.player.staked;

    let (executor, vault) = (ctx.accounts.executor.to_account_info(), ctx.accounts.vault.to_account_info());
    crate::token_ops::move_tokens(executor, vault, ctx.accounts.operator.to_account_info(), ctx.accounts.token_program.to_account_info(), stake)?;

    let player = &mut ctx.accounts.player;
    player.balance = player.balance.checked_add(stake).ok_or(ArenaError::MathOverflow)?;
    player.staked = 0;
    player.in_match = false;
    Ok(())
}
