use anchor_lang::prelude::*;

use crate::state::{Buyer, Sale};

#[derive(Accounts)]
pub struct OpenBuyer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub sale: Account<'info, Sale>,

    #[account(
        init,
        payer = owner,
        space = Buyer::SPACE,
        seeds = [Buyer::SEED, sale.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub buyer: Account<'info, Buyer>,

    pub system_program: Program<'info, System>,
}

pub fn open_buyer(ctx: Context<OpenBuyer>) -> Result<()> {
    let buyer = &mut ctx.accounts.buyer;
    buyer.sale = ctx.accounts.sale.key();
    buyer.owner = ctx.accounts.owner.key();
    buyer.purchased = 0;
    buyer.claimed = false;
    buyer.bump = ctx.bumps.buyer;
    Ok(())
}
