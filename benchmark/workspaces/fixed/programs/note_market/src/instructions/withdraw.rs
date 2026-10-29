use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::MarketError;
use crate::events::Withdrawn;
use crate::state::{Position, Vault};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, seeds = [Vault::SEED, vault.asset_mint.as_ref()], bump = vault.bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [Position::SEED, vault.key().as_ref(), owner.key().as_ref()],
        bump = position.bump,
        has_one = owner,
        has_one = vault,
    )]
    pub position: Account<'info, Position>,

    #[account(mut, address = vault.vault_token)]
    pub vault_token: Account<'info, TokenAccount>,

    #[account(mut, token::mint = vault.asset_mint, token::authority = owner)]
    pub owner_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw(ctx: Context<Withdraw>, assets: u64) -> Result<()> {
    require!(assets > 0, MarketError::ZeroAmount);
    let vault = &mut ctx.accounts.vault;
    require!(vault.total_notes > 0, MarketError::EmptyVault);
    require!(assets <= vault.total_assets, MarketError::InsufficientAssets);

    let scaled = (assets as u128)
        .checked_mul(vault.total_notes as u128)
        .ok_or(MarketError::MathOverflow)?;
    let denom = vault.total_assets as u128;
    let notes_to_burn =
        u64::try_from((scaled + denom - 1) / denom).map_err(|_| MarketError::MathOverflow)?;

    let position = &mut ctx.accounts.position;
    require!(Clock::get()?.unix_timestamp >= position.unlock_ts, MarketError::StillLocked);
    require!(position.notes >= notes_to_burn, MarketError::InsufficientNotes);

    position.notes -= notes_to_burn;
    vault.total_notes -= notes_to_burn;
    vault.total_assets -= assets;

    let mint = vault.asset_mint;
    let seeds = vault.signer_seeds(&mint);
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token.to_account_info(),
                to: ctx.accounts.owner_token.to_account_info(),
                authority: vault.to_account_info(),
            },
            &[&seeds],
        ),
        assets,
    )?;
    emit!(Withdrawn {
        vault: ctx.accounts.vault.key(),
        owner: ctx.accounts.owner.key(),
        assets,
        notes_burned: notes_to_burn,
    });
    Ok(())
}
