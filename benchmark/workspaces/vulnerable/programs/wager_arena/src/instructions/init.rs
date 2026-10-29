use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::Arena;

#[derive(Accounts)]
pub struct InitArena<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = Arena::SPACE,
        seeds = [Arena::SEED, token_mint.key().as_ref()],
        bump
    )]
    pub arena: Account<'info, Arena>,

    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = arena,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(token::mint = token_mint)]
    pub executor: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn init_arena(ctx: Context<InitArena>, operator: Pubkey) -> Result<()> {
    let arena = &mut ctx.accounts.arena;
    arena.authority = ctx.accounts.authority.key();
    arena.operator = operator;
    arena.token_mint = ctx.accounts.token_mint.key();
    arena.vault = ctx.accounts.vault.key();
    arena.executor = ctx.accounts.executor.key();
    arena.bump = ctx.bumps.arena;
    Ok(())
}
