use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::MarketError;
use crate::events::YieldAccrued;
use crate::state::Vault;

#[derive(Accounts)]
pub struct AccrueYield<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [Vault::SEED, vault.asset_mint.as_ref()],
        bump = vault.bump,
        has_one = authority @ MarketError::NotAuthority,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut, address = vault.vault_token)]
    pub vault_token: Account<'info, TokenAccount>,

    #[account(mut, token::mint = vault.asset_mint, token::authority = authority)]
    pub source_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn accrue_yield(ctx: Context<AccrueYield>, amount: u64) -> Result<()> {
    require!(amount > 0, MarketError::ZeroAmount);
    require!(ctx.accounts.vault.total_notes > 0, MarketError::EmptyVault);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.source_token.to_account_info(),
                to: ctx.accounts.vault_token.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;

    let vault = &mut ctx.accounts.vault;
    vault.total_assets = vault.total_assets.checked_add(amount).ok_or(MarketError::MathOverflow)?;
    emit!(YieldAccrued {
        vault: vault.key(),
        amount,
        total_assets: vault.total_assets,
    });
    Ok(())
}
