use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::DeskError;
use crate::state::{Market, Position};

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    pub owner: Signer<'info>,

    #[account(seeds = [Market::SEED, market.base_mint.as_ref(), market.quote_mint.as_ref()], bump = market.bump)]
    pub market: Account<'info, Market>,

    #[account(mut, seeds = [Position::SEED, market.key().as_ref(), owner.key().as_ref()],
        bump = position.bump, has_one = owner, has_one = market)]
    pub position: Account<'info, Position>,

    #[account(mut, address = market.collateral_vault)]
    pub collateral_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = market.base_mint, token::authority = owner)]
    pub owner_base: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
    require!(amount > 0, DeskError::ZeroAmount);
    let deposit_in = Transfer {
        from: ctx.accounts.owner_base.to_account_info(),
        to: ctx.accounts.collateral_vault.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), deposit_in), amount)?;
    let position = &mut ctx.accounts.position;
    position.collateral = position.collateral.checked_add(amount).ok_or(DeskError::MathOverflow)?;
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
    pub owner: Signer<'info>,

    #[account(seeds = [Market::SEED, market.base_mint.as_ref(), market.quote_mint.as_ref()], bump = market.bump)]
    pub market: Account<'info, Market>,

    #[account(mut, seeds = [Position::SEED, market.key().as_ref(), owner.key().as_ref()],
        bump = position.bump, has_one = owner, has_one = market)]
    pub position: Account<'info, Position>,

    #[account(mut, address = market.collateral_vault)]
    pub collateral_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = market.base_mint, token::authority = owner)]
    pub owner_base: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> Result<()> {
    require!(amount > 0, DeskError::ZeroAmount);
    require!(ctx.accounts.position.debt == 0, DeskError::OutstandingDebt);
    require!(ctx.accounts.position.collateral >= amount, DeskError::InsufficientCollateral);
    ctx.accounts.position.collateral -= amount;
    let market = &ctx.accounts.market;
    let base = market.base_mint;
    let quote = market.quote_mint;
    let seeds = market.signer_seeds(&base, &quote);
    let out = Transfer {
        from: ctx.accounts.collateral_vault.to_account_info(),
        to: ctx.accounts.owner_base.to_account_info(),
        authority: ctx.accounts.market.to_account_info(),
    };
    let program = ctx.accounts.token_program.to_account_info();
    token::transfer(CpiContext::new_with_signer(program, out, &[&seeds]), amount)?;
    Ok(())
}
