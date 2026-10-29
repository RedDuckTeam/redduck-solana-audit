use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use super::{desk_transfer, owner_transfer};
use crate::errors::FxError;
use crate::events::LiquidityMoved;
use crate::state::*;

#[derive(Accounts)]
pub struct MoveLiquidity<'info> {
    pub admin: Signer<'info>,
    #[account(has_one = admin)]
    pub desk: Account<'info, Desk>,
    #[account(
        mut,
        constraint = vault.key() == desk.base_vault || vault.key() == desk.quote_vault @ FxError::UnknownVault
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = vault.mint, token::authority = admin)]
    pub admin_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn provide_liquidity(ctx: Context<MoveLiquidity>, amount: u64) -> Result<()> {
    require!(amount > 0, FxError::ZeroAmount);
    let a = &ctx.accounts;
    owner_transfer(&a.token_program, &a.admin_token, &a.vault, &a.admin, amount)?;
    emit!(LiquidityMoved { desk: a.desk.key(), vault: a.vault.key(), amount, deposit: true });
    Ok(())
}

pub fn withdraw_liquidity(ctx: Context<MoveLiquidity>, amount: u64) -> Result<()> {
    require!(amount > 0, FxError::ZeroAmount);
    let a = &ctx.accounts;
    desk_transfer(&a.token_program, &a.desk, &a.vault, &a.admin_token, amount)?;
    emit!(LiquidityMoved { desk: a.desk.key(), vault: a.vault.key(), amount, deposit: false });
    Ok(())
}
