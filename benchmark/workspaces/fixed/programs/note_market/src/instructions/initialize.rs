use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::Vault;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub asset_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = Vault::SPACE,
        seeds = [Vault::SEED, asset_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = authority,
        token::mint = asset_mint,
        token::authority = vault,
        seeds = [b"vault_token", asset_mint.key().as_ref()],
        bump
    )]
    pub vault_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.authority = ctx.accounts.authority.key();
    vault.asset_mint = ctx.accounts.asset_mint.key();
    vault.vault_token = ctx.accounts.vault_token.key();
    vault.total_notes = 0;
    vault.total_assets = 0;
    vault.bump = ctx.bumps.vault;
    Ok(())
}
