use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::SaleError;
use crate::events::Purchased;
use crate::state::{Buyer, Sale};

#[derive(Accounts)]
pub struct BuyBundle<'info> {
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

pub fn buy_bundle(ctx: Context<BuyBundle>, amount: u64) -> Result<()> {
    require!(amount > 0, SaleError::ZeroAmount);
    let now = Clock::get()?.unix_timestamp;
    let sale = &mut ctx.accounts.sale;
    require!(now >= sale.start_ts && now < sale.end_ts, SaleError::SaleClosed);
    require!(amount <= sale.per_tx_limit, SaleError::TxLimitExceeded);

    let bonus = (amount as u128)
        .checked_mul(sale.bonus_bps as u128)
        .ok_or(SaleError::MathOverflow)?
        / 10_000u128;
    let credited = (amount as u128)
        .checked_add(bonus)
        .ok_or(SaleError::MathOverflow)?;
    let credited = u64::try_from(credited).map_err(|_| SaleError::MathOverflow)?;

    let new_sold = sale.sold.checked_add(credited).ok_or(SaleError::MathOverflow)?;
    require!(new_sold <= sale.hard_cap, SaleError::HardCapExceeded);
    require!(ctx.accounts.sale_vault.amount >= new_sold, SaleError::Unbacked);

    let new_purchased = ctx.accounts.buyer.purchased.checked_add(credited).ok_or(SaleError::MathOverflow)?;
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

    let buyer = &mut ctx.accounts.buyer;
    buyer.purchased = buyer.purchased.checked_add(credited).ok_or(SaleError::MathOverflow)?;
    let total_purchased = buyer.purchased;
    sale.sold = new_sold;
    emit!(Purchased {
        sale: sale.key(),
        buyer: ctx.accounts.owner.key(),
        credited,
        total_purchased,
    });
    Ok(())
}
