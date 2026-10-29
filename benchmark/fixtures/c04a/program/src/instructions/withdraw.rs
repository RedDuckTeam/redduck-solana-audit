use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::SaleError;
use crate::events::ProceedsWithdrawn;
use crate::state::Sale;

#[derive(Accounts)]
pub struct WithdrawProceeds<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [Sale::SEED, sale.sale_mint.as_ref()],
        bump = sale.bump,
        has_one = authority @ SaleError::NotAuthority,
    )]
    pub sale: Account<'info, Sale>,

    #[account(mut, address = sale.payment_vault)]
    pub payment_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = sale.payment_mint)]
    pub destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw_proceeds(ctx: Context<WithdrawProceeds>, amount: u64) -> Result<()> {
    require!(amount > 0, SaleError::ZeroAmount);
    require!(
        Clock::get()?.unix_timestamp >= ctx.accounts.sale.end_ts,
        SaleError::SaleNotEnded
    );

    let mint = ctx.accounts.sale.sale_mint;
    let seeds = ctx.accounts.sale.signer_seeds(&mint);
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payment_vault.to_account_info(),
                to: ctx.accounts.destination.to_account_info(),
                authority: ctx.accounts.sale.to_account_info(),
            },
            &[&seeds],
        ),
        amount,
    )?;
    emit!(ProceedsWithdrawn {
        sale: ctx.accounts.sale.key(),
        amount,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawUnsold<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [Sale::SEED, sale.sale_mint.as_ref()],
        bump = sale.bump,
        has_one = authority @ SaleError::NotAuthority,
    )]
    pub sale: Account<'info, Sale>,

    #[account(mut, address = sale.sale_vault)]
    pub sale_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = sale.sale_mint)]
    pub destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn withdraw_unsold(ctx: Context<WithdrawUnsold>, amount: u64) -> Result<()> {
    require!(amount > 0, SaleError::ZeroAmount);
    let sale = &ctx.accounts.sale;
    require!(Clock::get()?.unix_timestamp >= sale.end_ts, SaleError::SaleNotEnded);
    let owed = sale.sold.checked_sub(sale.claimed_total).ok_or(SaleError::MathOverflow)?;
    let unsold = ctx.accounts.sale_vault.amount.checked_sub(owed).ok_or(SaleError::ExceedsUnsold)?;
    require!(amount <= unsold, SaleError::ExceedsUnsold);

    let mint = sale.sale_mint;
    let seeds = sale.signer_seeds(&mint);
    let out = Transfer {
        from: ctx.accounts.sale_vault.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.sale.to_account_info(),
    };
    let program = ctx.accounts.token_program.to_account_info();
    token::transfer(CpiContext::new_with_signer(program, out, &[&seeds]), amount)
}
