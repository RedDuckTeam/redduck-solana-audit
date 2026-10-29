use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::DealError;
use crate::events::{DealCancelled, DealCreated, ProceedsClaimed};
use crate::state::*;

use super::pay_from_escrow;

#[derive(Accounts)]
pub struct RegisterMerchant<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + Merchant::INIT_SPACE,
        seeds = [MERCHANT_SEED, authority.key().as_ref()],
        bump
    )]
    pub merchant: Account<'info, Merchant>,
    pub system_program: Program<'info, System>,
}

pub fn register_merchant(ctx: Context<RegisterMerchant>) -> Result<()> {
    let merchant = &mut ctx.accounts.merchant;
    merchant.authority = ctx.accounts.authority.key();
    merchant.deal_count = 0;
    merchant.bump = ctx.bumps.merchant;
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct DealParams {
    pub unit_price: u64,
    pub min_units: u32,
    pub max_units: u32,
    pub max_units_per_buyer: u32,
    pub deadline: i64,
}

#[derive(Accounts)]
pub struct CreateDeal<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    pub merchant: Account<'info, Merchant>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        space = 8 + Deal::INIT_SPACE,
        seeds = [DEAL_SEED, merchant.key().as_ref(), &merchant.deal_count.to_le_bytes()],
        bump
    )]
    pub deal: Account<'info, Deal>,
    #[account(
        init,
        payer = authority,
        seeds = [ESCROW_SEED, deal.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = deal
    )]
    pub escrow: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn create_deal(ctx: Context<CreateDeal>, params: DealParams) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(params.unit_price > 0, DealError::InvalidParams);
    require!(params.min_units > 0 && params.min_units <= params.max_units, DealError::InvalidParams);
    require!(params.max_units_per_buyer > 0, DealError::InvalidParams);
    require!(
        params.deadline > now && params.deadline - now <= MAX_DURATION,
        DealError::InvalidParams
    );
    // Every unit must stay payable out of the escrow without overflow.
    params
        .unit_price
        .checked_mul(params.max_units as u64)
        .ok_or(error!(DealError::InvalidParams))?;

    let merchant = &mut ctx.accounts.merchant;
    let deal = &mut ctx.accounts.deal;
    deal.merchant = merchant.key();
    deal.authority = merchant.authority;
    deal.id = merchant.deal_count;
    deal.mint = ctx.accounts.mint.key();
    deal.escrow = ctx.accounts.escrow.key();
    deal.unit_price = params.unit_price;
    deal.min_units = params.min_units;
    deal.max_units = params.max_units;
    deal.max_units_per_buyer = params.max_units_per_buyer;
    deal.deadline = params.deadline;
    deal.total_units = 0;
    deal.status = DealStatus::Open;
    deal.bump = ctx.bumps.deal;
    merchant.deal_count = merchant
        .deal_count
        .checked_add(1)
        .ok_or(error!(DealError::MathOverflow))?;

    emit!(DealCreated {
        deal: deal.key(),
        merchant: deal.merchant,
        id: deal.id,
        unit_price: deal.unit_price,
        min_units: deal.min_units,
        max_units: deal.max_units,
        deadline: deal.deadline,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CancelDeal<'info> {
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    pub deal: Account<'info, Deal>,
}

/// Stops an open deal before its deadline. Every buyer can then take a refund.
pub fn cancel_deal(ctx: Context<CancelDeal>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let deal = &mut ctx.accounts.deal;
    require!(deal.outcome(now) == Outcome::Open, DealError::DealClosed);
    deal.status = DealStatus::Cancelled;
    emit!(DealCancelled { deal: deal.key() });
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimProceeds<'info> {
    pub authority: Signer<'info>,
    #[account(mut, has_one = authority)]
    pub deal: Account<'info, Deal>,
    #[account(mut, address = deal.escrow)]
    pub escrow: Account<'info, TokenAccount>,
    #[account(mut, token::mint = deal.mint)]
    pub proceeds: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// After the deadline of a deal that reached its minimum, pays every committed
/// unit to the merchant. Can be called once.
pub fn claim_proceeds(ctx: Context<ClaimProceeds>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let deal = &mut ctx.accounts.deal;
    require!(
        deal.status == DealStatus::Open && deal.outcome(now) == Outcome::Succeeded,
        DealError::NotSucceeded
    );
    deal.status = DealStatus::Claimed;
    let amount = deal.cost(deal.total_units)?;

    pay_from_escrow(
        &ctx.accounts.token_program,
        &ctx.accounts.deal,
        &ctx.accounts.escrow,
        &ctx.accounts.proceeds,
        amount,
    )?;
    emit!(ProceedsClaimed {
        deal: ctx.accounts.deal.key(),
        amount,
    });
    Ok(())
}
