use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::Hub;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = Hub::SPACE,
        seeds = [Hub::SEED, token_mint.key().as_ref()],
        bump
    )]
    pub hub: Account<'info, Hub>,

    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = hub,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let hub = &mut ctx.accounts.hub;
    hub.authority = ctx.accounts.authority.key();
    hub.token_mint = ctx.accounts.token_mint.key();
    hub.vault = ctx.accounts.vault.key();
    hub.bump = ctx.bumps.hub;
    Ok(())
}
