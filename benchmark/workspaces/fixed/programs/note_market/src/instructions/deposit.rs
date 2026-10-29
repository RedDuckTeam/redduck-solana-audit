use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::MarketError;
use crate::events::Deposited;
use crate::state::{Position, Vault, HOLD_SECS};

#[derive(Accounts)]
pub struct Deposit<'info> {
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

pub fn deposit(ctx: Context<Deposit>, assets: u64) -> Result<()> {
    require!(assets > 0, MarketError::ZeroAmount);
    let vault = &mut ctx.accounts.vault;

    let notes = if vault.total_notes == 0 {
        assets
    } else {
        let scaled = (assets as u128)
            .checked_mul(vault.total_notes as u128)
            .ok_or(MarketError::MathOverflow)?;
        u64::try_from(scaled / vault.total_assets as u128).map_err(|_| MarketError::MathOverflow)?
    };
    require!(notes > 0, MarketError::ZeroAmount);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.owner_token.to_account_info(),
                to: ctx.accounts.vault_token.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        assets,
    )?;

    let position = &mut ctx.accounts.position;
    position.notes = position.notes.checked_add(notes).ok_or(MarketError::MathOverflow)?;
    position.unlock_ts = Clock::get()?.unix_timestamp + HOLD_SECS;
    vault.total_notes = vault.total_notes.checked_add(notes).ok_or(MarketError::MathOverflow)?;
    vault.total_assets = vault.total_assets.checked_add(assets).ok_or(MarketError::MathOverflow)?;
    emit!(Deposited {
        vault: ctx.accounts.vault.key(),
        owner: ctx.accounts.owner.key(),
        assets,
        notes,
    });
    Ok(())
}
