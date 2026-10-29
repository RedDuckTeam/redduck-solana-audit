use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::CardError;
use crate::state::{Card, Hub};

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(seeds = [Hub::SEED, hub.token_mint.as_ref()], bump = hub.bump)]
    pub hub: Account<'info, Hub>,

    #[account(
        mut,
        seeds = [Card::SEED, hub.key().as_ref(), owner.key().as_ref()],
        bump = card.bump,
        has_one = owner,
        has_one = hub,
    )]
    pub card: Account<'info, Card>,

    #[account(mut, address = hub.vault)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = hub.token_mint, token::authority = owner)]
    pub owner_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn redeem(ctx: Context<Redeem>, amount: u64) -> Result<()> {
    require!(amount > 0, CardError::ZeroAmount);
    require!(ctx.accounts.card.balance >= amount, CardError::InsufficientBalance);

    ctx.accounts.card.balance -= amount;

    let mint = ctx.accounts.hub.token_mint;
    let seeds = ctx.accounts.hub.signer_seeds(&mint);
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.owner_token.to_account_info(),
                authority: ctx.accounts.hub.to_account_info(),
            },
            &[&seeds],
        ),
        amount,
    )?;
    emit!(crate::events::Redeemed {
        hub: ctx.accounts.hub.key(),
        owner: ctx.accounts.owner.key(),
        amount,
    });
    Ok(())
}
