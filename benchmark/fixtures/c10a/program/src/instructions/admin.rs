use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::FxError;
use crate::events::{PauseChanged, SettingsChanged};
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeDesk<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub base_mint: Account<'info, Mint>,
    #[account(constraint = quote_mint.key() != base_mint.key())]
    pub quote_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        space = 8 + Desk::INIT_SPACE,
        seeds = [DESK_SEED, admin.key().as_ref(), base_mint.key().as_ref(), quote_mint.key().as_ref()],
        bump
    )]
    pub desk: Account<'info, Desk>,
    #[account(
        init,
        payer = admin,
        seeds = [VAULT_SEED, desk.key().as_ref(), base_mint.key().as_ref()],
        bump,
        token::mint = base_mint,
        token::authority = desk
    )]
    pub base_vault: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = admin,
        seeds = [VAULT_SEED, desk.key().as_ref(), quote_mint.key().as_ref()],
        bump,
        token::mint = quote_mint,
        token::authority = desk
    )]
    pub quote_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_desk(ctx: Context<InitializeDesk>, settings: DeskSettings) -> Result<()> {
    let desk = &mut ctx.accounts.desk;
    desk.admin = ctx.accounts.admin.key();
    desk.base_mint = ctx.accounts.base_mint.key();
    desk.quote_mint = ctx.accounts.quote_mint.key();
    desk.base_vault = ctx.accounts.base_vault.key();
    desk.quote_vault = ctx.accounts.quote_vault.key();
    desk.rate = 0;
    desk.rate_published_at = 0;
    desk.paused_until = 0;
    desk.bump = ctx.bumps.desk;
    desk.apply(&settings)?;

    emit!(settings_changed(desk));
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateSettings<'info> {
    pub admin: Signer<'info>,
    #[account(mut, has_one = admin)]
    pub desk: Account<'info, Desk>,
}

pub fn update_settings(ctx: Context<UpdateSettings>, settings: DeskSettings) -> Result<()> {
    let desk = &mut ctx.accounts.desk;
    desk.apply(&settings)?;
    emit!(settings_changed(desk));
    Ok(())
}

#[derive(Accounts)]
pub struct SetPause<'info> {
    pub authority: Signer<'info>,
    #[account(mut)]
    pub desk: Account<'info, Desk>,
}

/// Pauses conversions for `duration_secs` seconds; zero lifts the pause.
pub fn set_pause(ctx: Context<SetPause>, duration_secs: i64) -> Result<()> {
    let authority = ctx.accounts.authority.key();
    let desk = &mut ctx.accounts.desk;
    require!(
        authority == desk.admin || authority == desk.operator,
        FxError::Unauthorized
    );
    require!((0..=MAX_PAUSE_SECS).contains(&duration_secs), FxError::InvalidPause);

    let now = Clock::get()?.unix_timestamp;
    desk.paused_until = if duration_secs == 0 { 0 } else { now.saturating_add(duration_secs) };
    emit!(PauseChanged { desk: desk.key(), paused_until: desk.paused_until });
    Ok(())
}

fn settings_changed(desk: &Account<Desk>) -> SettingsChanged {
    SettingsChanged {
        desk: desk.key(),
        operator: desk.operator,
        publisher: desk.publisher,
        fee_bps: desk.fee_bps,
        max_rate_age: desk.max_rate_age,
    }
}
