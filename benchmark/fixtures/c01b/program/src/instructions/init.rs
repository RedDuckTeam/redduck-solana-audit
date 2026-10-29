use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::DeskError;
use crate::state::Market;

#[derive(Accounts)]
pub struct InitMarket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub base_mint: Account<'info, Mint>,
    pub quote_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = Market::SPACE,
        seeds = [Market::SEED, base_mint.key().as_ref(), quote_mint.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init, payer = authority, token::mint = base_mint, token::authority = market,
        seeds = [b"pool_base", market.key().as_ref()], bump
    )]
    pub pool_base_vault: Account<'info, TokenAccount>,

    #[account(
        init, payer = authority, token::mint = quote_mint, token::authority = market,
        seeds = [b"pool_quote", market.key().as_ref()], bump
    )]
    pub pool_quote_vault: Account<'info, TokenAccount>,

    #[account(
        init, payer = authority, token::mint = base_mint, token::authority = market,
        seeds = [b"collateral", market.key().as_ref()], bump
    )]
    pub collateral_vault: Account<'info, TokenAccount>,

    #[account(
        init, payer = authority, token::mint = quote_mint, token::authority = market,
        seeds = [b"borrow", market.key().as_ref()], bump
    )]
    pub borrow_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn init_market(ctx: Context<InitMarket>, ltv_bps: u16) -> Result<()> {
    require!(ltv_bps > 0 && ltv_bps <= 10_000, DeskError::BadLtv);
    let market = &mut ctx.accounts.market;
    market.authority = ctx.accounts.authority.key();
    market.base_mint = ctx.accounts.base_mint.key();
    market.quote_mint = ctx.accounts.quote_mint.key();
    market.pool_base_vault = ctx.accounts.pool_base_vault.key();
    market.pool_quote_vault = ctx.accounts.pool_quote_vault.key();
    market.collateral_vault = ctx.accounts.collateral_vault.key();
    market.borrow_vault = ctx.accounts.borrow_vault.key();
    market.reserve_base = 0;
    market.reserve_quote = 0;
    market.ltv_bps = ltv_bps;
    market.bump = ctx.bumps.market;
    Ok(())
}
