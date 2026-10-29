use anchor_lang::prelude::*;

use crate::errors::CardError;
use crate::events::Transferred;
use crate::state::{Card, Hub};

#[derive(Accounts)]
#[instruction(amount: u64, recipient: Pubkey)]
pub struct TransferBalance<'info> {
    pub owner: Signer<'info>,

    #[account(seeds = [Hub::SEED, hub.token_mint.as_ref()], bump = hub.bump)]
    pub hub: Account<'info, Hub>,

    #[account(
        mut,
        seeds = [Card::SEED, hub.key().as_ref(), owner.key().as_ref()],
        bump = from.bump,
        has_one = owner,
        has_one = hub,
    )]
    pub from: Account<'info, Card>,

    #[account(
        mut,
        seeds = [Card::SEED, hub.key().as_ref(), recipient.as_ref()],
        bump = to.bump,
        has_one = hub,
        constraint = to.key() != from.key() @ CardError::SameCard,
    )]
    pub to: Account<'info, Card>,
}

pub fn transfer(ctx: Context<TransferBalance>, amount: u64, recipient: Pubkey) -> Result<()> {
    require!(amount > 0, CardError::ZeroAmount);

    let from = &mut ctx.accounts.from;
    require!(from.balance >= amount, CardError::InsufficientBalance);
    from.balance -= amount;

    let to = &mut ctx.accounts.to;
    to.balance = to.balance.checked_add(amount).ok_or(CardError::MathOverflow)?;

    emit!(Transferred {
        hub: ctx.accounts.hub.key(),
        from: ctx.accounts.owner.key(),
        recipient,
        amount,
    });
    Ok(())
}
