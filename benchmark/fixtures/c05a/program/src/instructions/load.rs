use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::CardError;
use crate::state::{Card, Hub};

#[derive(Accounts)]
pub struct Load<'info> {
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

pub fn load(ctx: Context<Load>, amount: u64) -> Result<()> {
    require!(amount > 0, CardError::ZeroAmount);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.owner_token.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    let card = &mut ctx.accounts.card;
    card.balance = card.balance.checked_add(amount).ok_or(CardError::MathOverflow)?;
    emit!(crate::events::Loaded {
        hub: ctx.accounts.hub.key(),
        owner: ctx.accounts.owner.key(),
        amount,
    });
    Ok(())
}
