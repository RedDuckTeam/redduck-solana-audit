use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::CrateError;
use crate::state::{Commitment, House};

#[derive(Accounts)]
pub struct OpenCrate<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(mut, seeds = [House::SEED, house.token_mint.as_ref()], bump = house.bump)]
    pub house: Account<'info, House>,

    #[account(
        init,
        payer = player,
        space = Commitment::SPACE,
        seeds = [Commitment::SEED, house.key().as_ref(), player.key().as_ref()],
        bump
    )]
    pub commitment: Account<'info, Commitment>,

    #[account(mut, address = house.vault)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut, token::mint = house.token_mint, token::authority = player)]
    pub player_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn open_crate(ctx: Context<OpenCrate>) -> Result<()> {
    let cost = ctx.accounts.house.cost;
    let prize = ctx.accounts.house.prize;
    let odds_bps = ctx.accounts.house.odds_bps;
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.player_token.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            },
        ),
        cost,
    )?;
    ctx.accounts.vault.reload()?;
    let needed = ctx.accounts.house.reserved.checked_add(prize).ok_or(CrateError::MathOverflow)?;
    require!(ctx.accounts.vault.amount >= needed, CrateError::Underfunded);

    let clock = Clock::get()?;
    let commitment = &mut ctx.accounts.commitment;
    commitment.house = ctx.accounts.house.key();
    commitment.player = ctx.accounts.player.key();
    commitment.reveal_slot = clock.slot.checked_add(1).ok_or(CrateError::MathOverflow)?;
    commitment.paid = cost;
    commitment.prize = prize;
    commitment.odds_bps = odds_bps;
    commitment.bump = ctx.bumps.commitment;

    let house = &mut ctx.accounts.house;
    house.reserved = house.reserved.checked_add(prize).ok_or(CrateError::MathOverflow)?;
    emit!(crate::events::CrateOpened {
        house: house.key(),
        player: ctx.accounts.player.key(),
        reveal_slot: ctx.accounts.commitment.reveal_slot,
    });
    Ok(())
}
