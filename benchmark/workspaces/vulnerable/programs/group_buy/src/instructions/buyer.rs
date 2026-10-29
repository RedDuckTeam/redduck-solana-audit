use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::DealError;
use crate::events::{BuyerJoined, Refunded, UnitsCommitted};
use crate::state::*;

use super::pay_from_escrow;

#[derive(Accounts)]
pub struct JoinDeal<'info> {
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub deal: Account<'info, Deal>,
}

/// Registers the buyer on an open deal so they can commit units. Joining again has no effect.
pub fn join_deal(ctx: Context<JoinDeal>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let buyer = ctx.accounts.buyer.key();
    let deal = &mut ctx.accounts.deal;
    require!(deal.outcome(now) == Outcome::Open, DealError::DealClosed);
    if deal.buyers.iter().any(|c| c.buyer == buyer) {
        return Ok(());
    }
    require!(deal.buyers.len() < MAX_BUYERS, DealError::DealFull);
    deal.buyers.push(Commitment {
        buyer,
        units: 0,
        refunded: false,
    });

    emit!(BuyerJoined { deal: deal.key(), buyer });
    Ok(())
}

#[derive(Accounts)]
pub struct Commit<'info> {
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub deal: Account<'info, Deal>,
    #[account(mut, address = deal.escrow)]
    pub escrow: Account<'info, TokenAccount>,
    #[account(mut, token::mint = deal.mint, token::authority = buyer)]
    pub buyer_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Pays for `units` more units of the deal at the fixed unit price.
pub fn commit(ctx: Context<Commit>, units: u32) -> Result<()> {
    require!(units > 0, DealError::ZeroUnits);
    let now = Clock::get()?.unix_timestamp;
    let buyer = ctx.accounts.buyer.key();
    let deal = &mut ctx.accounts.deal;
    require!(deal.outcome(now) == Outcome::Open, DealError::DealClosed);

    let total = deal.total_units.checked_add(units).ok_or(error!(DealError::MathOverflow))?;
    require!(total <= deal.max_units, DealError::DealFull);
    let cost = deal.cost(units)?;
    let limit = deal.max_units_per_buyer;

    let entry = deal.buyers.iter_mut().find(|c| c.buyer == buyer).ok_or(error!(DealError::NotJoined))?;
    entry.units = entry.units.checked_add(units).ok_or(error!(DealError::MathOverflow))?;
    require!(entry.units <= limit, DealError::BuyerLimit);
    deal.total_units = total;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer_token.to_account_info(),
                to: ctx.accounts.escrow.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
        ),
        cost,
    )?;

    emit!(UnitsCommitted {
        deal: ctx.accounts.deal.key(),
        buyer,
        units,
        total_units: total,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct Refund<'info> {
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub deal: Account<'info, Deal>,
    #[account(mut, address = deal.escrow)]
    pub escrow: Account<'info, TokenAccount>,
    #[account(mut, token::mint = deal.mint)]
    pub buyer_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Returns everything the buyer paid into a deal that failed or was cancelled.
pub fn refund(ctx: Context<Refund>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let buyer = ctx.accounts.buyer.key();
    let deal = &mut ctx.accounts.deal;
    require!(deal.outcome(now) == Outcome::Failed, DealError::NotFailed);

    let entry = deal.buyers.iter_mut().find(|c| c.buyer == buyer).ok_or(error!(DealError::NotJoined))?;
    require!(!entry.refunded && entry.units > 0, DealError::NothingToRefund);
    entry.refunded = true;
    let units = entry.units;
    let amount = deal.cost(units)?;

    pay_from_escrow(
        &ctx.accounts.token_program,
        &ctx.accounts.deal,
        &ctx.accounts.escrow,
        &ctx.accounts.buyer_token,
        amount,
    )?;
    emit!(Refunded {
        deal: ctx.accounts.deal.key(),
        buyer,
        amount,
    });
    Ok(())
}
