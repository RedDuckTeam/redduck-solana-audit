use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use super::{desk_transfer, owner_transfer};
use crate::errors::FxError;
use crate::events::Converted;
use crate::state::*;

#[derive(Accounts)]
pub struct Convert<'info> {
    pub user: Signer<'info>,
    pub desk: Account<'info, Desk>,
    #[account(mut, address = desk.base_vault)]
    pub base_vault: Account<'info, TokenAccount>,
    #[account(mut, address = desk.quote_vault)]
    pub quote_vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = desk.base_mint, token::authority = user)]
    pub user_base: Account<'info, TokenAccount>,
    #[account(mut, token::mint = desk.quote_mint, token::authority = user)]
    pub user_quote: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Sells `amount_in` of the base token (`sell_base`) or of the quote token for the
/// other side at the desk rate, less the fee.
pub fn convert(ctx: Context<Convert>, amount_in: u64, min_out: u64, sell_base: bool) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let a = &ctx.accounts;
    let desk = &a.desk;
    require!(now >= desk.paused_until, FxError::Paused);
    require!(amount_in > 0, FxError::ZeroAmount);

    let rate = desk.usable_rate(now)?;
    let amount_out = desk.amount_out(amount_in, rate, sell_base)?;
    require!(amount_out > 0 && amount_out >= min_out, FxError::SlippageExceeded);

    let (user_in, vault_in, vault_out, user_out) = if sell_base {
        (&a.user_base, &a.base_vault, &a.quote_vault, &a.user_quote)
    } else {
        (&a.user_quote, &a.quote_vault, &a.base_vault, &a.user_base)
    };
    require!(vault_out.amount >= amount_out, FxError::InsufficientLiquidity);

    owner_transfer(&a.token_program, user_in, vault_in, &a.user, amount_in)?;
    desk_transfer(&a.token_program, desk, vault_out, user_out, amount_out)?;

    emit!(Converted {
        desk: desk.key(),
        user: a.user.key(),
        sell_base,
        amount_in,
        amount_out,
        rate,
    });
    Ok(())
}
