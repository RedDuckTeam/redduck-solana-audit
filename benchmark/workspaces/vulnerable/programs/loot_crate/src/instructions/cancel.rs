use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{Commitment, House};

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(mut, seeds = [House::SEED, house.token_mint.as_ref()], bump = house.bump)]
    pub house: Account<'info, House>,

    #[account(
        mut,
        close = player,
        has_one = house,
        has_one = player,
        seeds = [Commitment::SEED, house.key().as_ref(), player.key().as_ref()],
        bump = commitment.bump,
    )]
    pub commitment: Account<'info, Commitment>,

    #[account(mut, address = house.vault)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = house.token_mint, token::authority = player)]
    pub player_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
    let paid = ctx.accounts.commitment.paid;
    let prize = ctx.accounts.commitment.prize;
    let mint = ctx.accounts.house.token_mint;
    let seeds = ctx.accounts.house.signer_seeds(&mint);
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.player_token.to_account_info(),
                authority: ctx.accounts.house.to_account_info(),
            },
            &[&seeds],
        ),
        paid,
    )?;
    let house = &mut ctx.accounts.house;
    house.reserved = house.reserved.saturating_sub(prize);
    Ok(())
}
