use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::SaleError;
use crate::events::Purchased;
use crate::state::{Buyer, Sale};

#[derive(Accounts)]
pub struct Buy<'info> {
    pub owner: Signer<'info>,

    #[account(mut, seeds = [Sale::SEED, sale.sale_mint.as_ref()], bump = sale.bump)]
    pub sale: Account<'info, Sale>,

    #[account(
        mut,
        seeds = [Buyer::SEED, sale.key().as_ref(), owner.key().as_ref()],
        bump = buyer.bump,
        has_one = owner,
        has_one = sale,
    )]
    pub buyer: Account<'info, Buyer>,

    #[account(address = sale.sale_vault)]
    pub sale_vault: Account<'info, TokenAccount>,

    #[account(mut, address = sale.payment_vault)]
    pub payment_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = sale.payment_mint, token::authority = owner)]
    pub owner_payment: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
    require!(amount > 0, SaleError::ZeroAmount);
    let now = Clock::get()?.unix_timestamp;
    let sale = &mut ctx.accounts.sale;
    require!(now >= sale.start_ts && now < sale.end_ts, SaleError::SaleClosed);

    let new_sold = sale.sold.checked_add(amount).ok_or(SaleError::MathOverflow)?;
    require!(new_sold <= sale.hard_cap, SaleError::HardCapExceeded);
    require!(ctx.accounts.sale_vault.amount >= new_sold, SaleError::Unbacked);

    let buyer = &mut ctx.accounts.buyer;
    let new_purchased = buyer.purchased.checked_add(amount).ok_or(SaleError::MathOverflow)?;
    require!(new_purchased <= sale.per_wallet_cap, SaleError::WalletCapExceeded);

    let cost = amount.checked_mul(sale.price).ok_or(SaleError::MathOverflow)?;
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.owner_payment.to_account_info(),
                to: ctx.accounts.payment_vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        cost,
    )?;

    buyer.purchased = new_purchased;
    sale.sold = new_sold;
    emit!(Purchased {
        sale: sale.key(),
        buyer: ctx.accounts.owner.key(),
        credited: amount,
        total_purchased: new_purchased,
    });
    Ok(())
}
