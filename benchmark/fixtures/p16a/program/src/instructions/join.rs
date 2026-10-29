use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{Config, Entry, Round, RoundError};

#[derive(Accounts)]
pub struct Join<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,
    #[account(mut, seeds = [b"round", round.id.to_le_bytes().as_ref()], bump = round.bump)]
    pub round: Account<'info, Round>,
    #[account(
        init,
        payer = player,
        space = 8 + Entry::INIT_SPACE,
        seeds = [b"entry", round.key().as_ref(), player.key().as_ref()],
        bump,
    )]
    pub entry: Account<'info, Entry>,
    #[account(address = config.mint)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = player)]
    pub player_tokens: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn join(ctx: Context<Join>, amount: u64) -> Result<()> {
    require!(
        Clock::get()?.unix_timestamp < ctx.accounts.round.deadline,
        RoundError::RoundClosed
    );
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.player_tokens.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.player.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;
    record_entry(
        &mut ctx.accounts.round,
        &mut ctx.accounts.entry,
        ctx.accounts.player.key(),
        amount,
        ctx.bumps.entry,
    )
}

pub fn record_entry(
    round: &mut Account<Round>,
    entry: &mut Account<Entry>,
    player: Pubkey,
    amount: u64,
    bump: u8,
) -> Result<()> {
    round.pot = round
        .pot
        .checked_add(amount)
        .ok_or(RoundError::MathOverflow)?;
    entry.set_inner(Entry {
        round: round.key(),
        player,
        amount,
        bump,
    });
    Ok(())
}
