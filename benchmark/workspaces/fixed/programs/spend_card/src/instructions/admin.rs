use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::CardError;
use crate::events::{ConfigApplied, ConfigQueued};
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [CONFIG_SEED, admin.key().as_ref()],
        bump
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        seeds = [VAULT_SEED, config.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = config
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(token::mint = mint)]
    pub settlement: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, processor: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.admin = ctx.accounts.admin.key();
    config.mint = ctx.accounts.mint.key();
    config.vault = ctx.accounts.vault.key();
    config.settlement = ctx.accounts.settlement.key();
    config.pending_settlement = Pubkey::default();
    config.processor = processor;
    config.change_eta = 0;
    config.total_balance = 0;
    config.bump = ctx.bumps.config;
    Ok(())
}

#[derive(Accounts)]
pub struct QueueConfig<'info> {
    pub admin: Signer<'info>,
    #[account(mut, has_one = admin)]
    pub config: Account<'info, Config>,
    #[account(token::mint = config.mint)]
    pub new_settlement: Account<'info, TokenAccount>,
}

/// Queues a new settlement account and processor key. They take effect through
/// `apply_config` once `CONFIG_DELAY` has passed; queuing again replaces the pending change.
pub fn queue_config(ctx: Context<QueueConfig>, new_processor: Pubkey) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let config = &mut ctx.accounts.config;
    config.pending_settlement = ctx.accounts.new_settlement.key();
    config.pending_processor = new_processor;
    config.change_eta = now
        .checked_add(CONFIG_DELAY)
        .ok_or(error!(CardError::MathOverflow))?;

    emit!(ConfigQueued {
        config: config.key(),
        settlement: config.pending_settlement,
        eta: config.change_eta,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct ApplyConfig<'info> {
    pub admin: Signer<'info>,
    #[account(mut, has_one = admin)]
    pub config: Account<'info, Config>,
}

pub fn apply_config(ctx: Context<ApplyConfig>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let config = &mut ctx.accounts.config;
    require!(config.change_eta != 0, CardError::NothingQueued);
    require!(now >= config.change_eta, CardError::ChangeNotReady);

    config.settlement = config.pending_settlement;
    config.processor = config.pending_processor;
    config.pending_settlement = Pubkey::default();
    config.change_eta = 0;

    emit!(ConfigApplied {
        config: config.key(),
        settlement: config.settlement,
    });
    Ok(())
}
