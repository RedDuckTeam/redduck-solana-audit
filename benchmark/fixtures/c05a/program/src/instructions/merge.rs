use anchor_lang::prelude::*;

use crate::errors::CardError;
use crate::state::{Card, Hub};

#[derive(Accounts)]
pub struct MergeCards<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(seeds = [Hub::SEED, hub.token_mint.as_ref()], bump = hub.bump)]
    pub hub: Account<'info, Hub>,

    #[account(
        mut,
        close = owner,
        has_one = owner,
        has_one = hub,
        constraint = source.key() != dest.key() @ CardError::SameCard,
    )]
    pub source: Account<'info, Card>,

    #[account(mut, has_one = hub)]
    pub dest: Account<'info, Card>,
}

pub fn merge_cards(ctx: Context<MergeCards>) -> Result<()> {
    let amount = ctx.accounts.source.balance;
    let dest = &mut ctx.accounts.dest;
    dest.balance = dest.balance.checked_add(amount).ok_or(CardError::MathOverflow)?;
    ctx.accounts.source.balance = 0;
    Ok(())
}
