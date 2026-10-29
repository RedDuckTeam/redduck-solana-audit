use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::CreditError;
use crate::pricing::Quote;
use crate::state::*;

use super::pay_in;

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub collateral_mint: Account<'info, Mint>,
    #[account(constraint = debt_mint.key() != collateral_mint.key() @ CreditError::InvalidParams)]
    pub debt_mint: Account<'info, Mint>,
    #[account(init, payer = admin, space = 8 + Market::INIT_SPACE,
        seeds = [MARKET_SEED, admin.key().as_ref(), collateral_mint.key().as_ref(), debt_mint.key().as_ref()], bump)]
    pub market: Account<'info, Market>,
    #[account(init, payer = admin, seeds = [b"collateral", market.key().as_ref()], bump,
        token::mint = collateral_mint, token::authority = market)]
    pub collateral_vault: Account<'info, TokenAccount>,
    #[account(init, payer = admin, seeds = [b"liquidity", market.key().as_ref()], bump,
        token::mint = debt_mint, token::authority = market)]
    pub liquidity_vault: Account<'info, TokenAccount>,
    /// CHECK: owner and layout are verified by `Quote::load` in the handler; the key is stored.
    pub collateral_feed: UncheckedAccount<'info>,
    /// CHECK: owner and layout are verified by `Quote::load` in the handler; the key is stored.
    pub debt_feed: UncheckedAccount<'info>,
    #[account(mut, token::mint = debt_mint, token::authority = admin)]
    pub admin_debt_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn init_market(ctx: Context<InitMarket>, p: MarketParams) -> Result<()> {
    let threshold = p.liquidation_threshold_bps as u64;
    require!(p.ltv_bps > 0 && p.ltv_bps < p.liquidation_threshold_bps, CreditError::InvalidParams);
    // A position liquidated at the threshold must still cover the bonus.
    require!(threshold * (BPS + p.liquidation_bonus_bps as u64) < BPS * BPS, CreditError::InvalidParams);
    require!(p.borrow_fee_bps <= MAX_BORROW_FEE_BPS, CreditError::InvalidParams);
    require!(p.seed_liquidity >= MIN_SEED_LIQUIDITY, CreditError::InvalidParams);

    let a = &ctx.accounts;
    let now = Clock::get()?.unix_timestamp;
    Quote::load(&a.collateral_feed, a.collateral_feed.key, now)?;
    Quote::load(&a.debt_feed, a.debt_feed.key, now)?;
    pay_in(&a.token_program, &a.admin_debt_account, &a.liquidity_vault, &a.admin, p.seed_liquidity)?;

    ctx.accounts.market.set_inner(Market {
        admin: ctx.accounts.admin.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        debt_mint: ctx.accounts.debt_mint.key(),
        collateral_vault: ctx.accounts.collateral_vault.key(),
        liquidity_vault: ctx.accounts.liquidity_vault.key(),
        collateral_feed: ctx.accounts.collateral_feed.key(),
        debt_feed: ctx.accounts.debt_feed.key(),
        collateral_decimals: ctx.accounts.collateral_mint.decimals,
        debt_decimals: ctx.accounts.debt_mint.decimals,
        cash: p.seed_liquidity,
        total_debt: 0,
        total_shares: p.seed_liquidity,
        params: p,
        bump: ctx.bumps.market,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct OpenAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub market: Account<'info, Market>,
    #[account(init, payer = owner, space = 8 + CreditAccount::INIT_SPACE,
        seeds = [ACCOUNT_SEED, market.key().as_ref(), owner.key().as_ref()], bump)]
    pub credit_account: Account<'info, CreditAccount>,
    pub system_program: Program<'info, System>,
}

pub fn open_account(ctx: Context<OpenAccount>) -> Result<()> {
    let account = &mut ctx.accounts.credit_account;
    account.market = ctx.accounts.market.key();
    account.owner = ctx.accounts.owner.key();
    Ok(())
}
