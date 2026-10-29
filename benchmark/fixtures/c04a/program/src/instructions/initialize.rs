use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::SaleError;
use crate::state::Sale;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SaleArgs {
    pub price: u64,
    pub per_wallet_cap: u64,
    pub per_tx_limit: u64,
    pub hard_cap: u64,
    pub bonus_bps: u16,
    pub start_ts: i64,
    pub end_ts: i64,
}

#[derive(Accounts)]
pub struct InitializeSale<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub payment_mint: Account<'info, Mint>,
    pub sale_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = Sale::SPACE,
        seeds = [Sale::SEED, sale_mint.key().as_ref()],
        bump
    )]
    pub sale: Account<'info, Sale>,

    #[account(
        init,
        payer = authority,
        token::mint = payment_mint,
        token::authority = sale,
        seeds = [b"payment_vault", sale_mint.key().as_ref()],
        bump
    )]
    pub payment_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = sale_mint,
        token::authority = sale,
        seeds = [b"sale_vault", sale_mint.key().as_ref()],
        bump
    )]
    pub sale_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_sale(ctx: Context<InitializeSale>, args: SaleArgs) -> Result<()> {
    require!(args.end_ts > args.start_ts, SaleError::BadWindow);
    require!(args.price > 0, SaleError::ZeroAmount);
    require!(args.per_wallet_cap > 0, SaleError::ZeroAmount);
    require!(args.per_tx_limit > 0, SaleError::ZeroAmount);
    require!(args.hard_cap > 0, SaleError::ZeroAmount);

    let sale = &mut ctx.accounts.sale;
    sale.authority = ctx.accounts.authority.key();
    sale.payment_mint = ctx.accounts.payment_mint.key();
    sale.sale_mint = ctx.accounts.sale_mint.key();
    sale.payment_vault = ctx.accounts.payment_vault.key();
    sale.sale_vault = ctx.accounts.sale_vault.key();
    sale.price = args.price;
    sale.per_wallet_cap = args.per_wallet_cap;
    sale.per_tx_limit = args.per_tx_limit;
    sale.hard_cap = args.hard_cap;
    sale.bonus_bps = args.bonus_bps;
    sale.sold = 0;
    sale.claimed_total = 0;
    sale.start_ts = args.start_ts;
    sale.end_ts = args.end_ts;
    sale.bump = ctx.bumps.sale;
    Ok(())
}
