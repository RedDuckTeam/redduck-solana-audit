use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::CardError;
use crate::events::{CardToppedUp, WithdrawalCompleted, WithdrawalRequested};
use crate::state::*;

use super::pay_from_vault;

#[derive(Accounts)]
pub struct OpenCard<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = owner,
        space = 8 + Card::INIT_SPACE,
        seeds = [CARD_SEED, config.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub card: Account<'info, Card>,
    pub system_program: Program<'info, System>,
}

pub fn open_card(ctx: Context<OpenCard>) -> Result<()> {
    let card = &mut ctx.accounts.card;
    card.config = ctx.accounts.config.key();
    card.owner = ctx.accounts.owner.key();
    card.balance = 0;
    card.spent = 0;
    card.last_auth_id = 0;
    card.clear_withdrawal();
    card.bump = ctx.bumps.card;
    Ok(())
}

#[derive(Accounts)]
pub struct TopUp<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(mut, has_one = owner, has_one = config)]
    pub card: Account<'info, Card>,
    #[account(mut, address = config.vault)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = config.mint, token::authority = owner)]
    pub owner_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn top_up(ctx: Context<TopUp>, amount: u64) -> Result<()> {
    require!(amount > 0, CardError::ZeroAmount);
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.owner_token.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    let card = &mut ctx.accounts.card;
    card.balance = card.balance.checked_add(amount).ok_or(error!(CardError::MathOverflow))?;
    let config = &mut ctx.accounts.config;
    config.total_balance = config
        .total_balance
        .checked_add(amount)
        .ok_or(error!(CardError::MathOverflow))?;

    emit!(CardToppedUp {
        card: card.key(),
        amount,
        balance: card.balance,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct RequestWithdrawal<'info> {
    pub owner: Signer<'info>,
    #[account(mut, has_one = owner)]
    pub card: Account<'info, Card>,
}

/// Records how much the owner wants back. A new request replaces the previous
/// one; an amount of zero cancels it.
pub fn request_withdrawal(ctx: Context<RequestWithdrawal>, amount: u64) -> Result<()> {
    let card = &mut ctx.accounts.card;
    require!(amount <= card.balance, CardError::InsufficientBalance);
    if amount == 0 {
        card.clear_withdrawal();
    } else {
        card.withdrawal_amount = amount;
        card.withdrawal_requested_at = Clock::get()?.unix_timestamp;
    }

    emit!(WithdrawalRequested {
        card: card.key(),
        amount,
        requested_at: card.withdrawal_requested_at,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CompleteWithdrawal<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub config: Account<'info, Config>,
    #[account(address = PROCESSOR @ CardError::NotProcessor)]
    pub processor: Signer<'info>,
    #[account(mut, has_one = owner, has_one = config)]
    pub card: Account<'info, Card>,
    #[account(mut, address = config.vault)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = config.mint)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Pays out the requested amount, or what is left of the balance if purchases
/// settled after the request.
pub fn complete_withdrawal(ctx: Context<CompleteWithdrawal>) -> Result<()> {
    let card = &mut ctx.accounts.card;
    require!(card.withdrawal_amount > 0, CardError::NoWithdrawal);

    let amount = card.withdrawal_amount.min(card.balance);
    card.debit(amount)?;
    card.clear_withdrawal();
    let config = &mut ctx.accounts.config;
    config.total_balance = config
        .total_balance
        .checked_sub(amount)
        .ok_or(error!(CardError::MathOverflow))?;

    pay_from_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.config,
        &ctx.accounts.vault,
        &ctx.accounts.destination,
        amount,
    )?;

    emit!(WithdrawalCompleted {
        card: ctx.accounts.card.key(),
        amount,
        balance: ctx.accounts.card.balance,
    });
    Ok(())
}
