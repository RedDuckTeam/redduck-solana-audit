use anchor_lang::prelude::*;

use crate::errors::CardError;
use crate::state::{Card, Hub};

#[derive(Accounts)]
pub struct CloseCard<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(seeds = [Hub::SEED, hub.token_mint.as_ref()], bump = hub.bump)]
    pub hub: Account<'info, Hub>,

    #[account(
        mut,
        close = owner,
        seeds = [Card::SEED, hub.key().as_ref(), owner.key().as_ref()],
        bump = card.bump,
        has_one = owner,
        has_one = hub,
    )]
    pub card: Account<'info, Card>,
}

pub fn close_card(ctx: Context<CloseCard>) -> Result<()> {
    require!(ctx.accounts.card.balance == 0, CardError::CardNotEmpty);
    Ok(())
}
