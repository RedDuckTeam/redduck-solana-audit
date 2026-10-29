use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::CardError;
use crate::events::SpendSettled;
use crate::state::*;

use super::pay_from_vault;

#[derive(Accounts)]
pub struct SettleSpend<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(address = config.processor @ CardError::NotProcessor)]
    pub processor: Signer<'info>,
    #[account(mut, has_one = config)]
    pub card: Account<'info, Card>,
    #[account(mut, address = config.vault)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, address = config.settlement)]
    pub settlement: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Moves a cleared card purchase from the card balance to the settlement account.
pub fn settle_spend(ctx: Context<SettleSpend>, auth_id: u64, amount: u64) -> Result<()> {
    require!(amount > 0, CardError::ZeroAmount);
    let card = &mut ctx.accounts.card;
    require!(auth_id > card.last_auth_id, CardError::DuplicateAuthorization);

    card.debit(amount)?;
    card.spent = card.spent.checked_add(amount).ok_or(error!(CardError::MathOverflow))?;
    card.last_auth_id = auth_id;
    let config = &mut ctx.accounts.config;
    config.total_balance = config
        .total_balance
        .checked_sub(amount)
        .ok_or(error!(CardError::MathOverflow))?;

    pay_from_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.config,
        &ctx.accounts.vault,
        &ctx.accounts.settlement,
        amount,
    )?;

    emit!(SpendSettled {
        card: ctx.accounts.card.key(),
        auth_id,
        amount,
        balance: ctx.accounts.card.balance,
    });
    Ok(())
}
