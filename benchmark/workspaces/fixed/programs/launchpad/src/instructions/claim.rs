use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::SaleError;
use crate::events::Claimed;
use crate::state::{Buyer, Sale};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
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

    #[account(mut, address = sale.sale_vault)]
    pub sale_vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = sale.sale_mint, token::authority = owner)]
    pub owner_sale_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn claim(ctx: Context<Claim>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(now >= ctx.accounts.sale.end_ts, SaleError::SaleNotEnded);
    require!(!ctx.accounts.buyer.claimed, SaleError::NothingToClaim);
    let amount = ctx.accounts.buyer.purchased;
    require!(amount > 0, SaleError::NothingToClaim);

    let mint = ctx.accounts.sale.sale_mint;
    let seeds = ctx.accounts.sale.signer_seeds(&mint);
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sale_vault.to_account_info(),
                to: ctx.accounts.owner_sale_token.to_account_info(),
                authority: ctx.accounts.sale.to_account_info(),
            },
            &[&seeds],
        ),
        amount,
    )?;

    ctx.accounts.buyer.claimed = true;
    let sale = &mut ctx.accounts.sale;
    sale.claimed_total = sale.claimed_total.checked_add(amount).ok_or(SaleError::MathOverflow)?;
    emit!(Claimed {
        sale: sale.key(),
        owner: ctx.accounts.owner.key(),
        amount,
    });
    Ok(())
}
