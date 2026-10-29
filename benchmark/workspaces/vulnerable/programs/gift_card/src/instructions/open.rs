use anchor_lang::prelude::*;

use crate::state::{Card, Hub};

#[derive(Accounts)]
pub struct OpenCard<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub hub: Account<'info, Hub>,

    #[account(
        init,
        payer = owner,
        space = Card::SPACE,
        seeds = [Card::SEED, hub.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub card: Account<'info, Card>,

    pub system_program: Program<'info, System>,
}

pub fn open_card(ctx: Context<OpenCard>) -> Result<()> {
    let card = &mut ctx.accounts.card;
    card.hub = ctx.accounts.hub.key();
    card.owner = ctx.accounts.owner.key();
    card.balance = 0;
    card.bump = ctx.bumps.card;
    Ok(())
}
