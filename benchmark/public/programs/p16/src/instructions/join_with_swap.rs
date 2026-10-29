use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::TokenAccount;

use crate::{record_entry, Config, Entry, Round, RoundError, DEX_PROGRAM_ID};

#[derive(Accounts)]
pub struct JoinWithSwap<'info> {
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
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, TokenAccount>,
    #[account(address = DEX_PROGRAM_ID)]
    pub dex_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn join_with_swap<'info>(
    ctx: Context<'_, '_, 'info, 'info, JoinWithSwap<'info>>,
    swap_data: Vec<u8>,
    min_amount: u64,
) -> Result<()> {
    require!(
        Clock::get()?.unix_timestamp < ctx.accounts.round.deadline,
        RoundError::RoundClosed
    );
    let before = ctx.accounts.vault.amount;
    let metas = ctx
        .remaining_accounts
        .iter()
        .map(|account| AccountMeta {
            pubkey: account.key(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect();
    invoke(
        &Instruction {
            program_id: ctx.accounts.dex_program.key(),
            accounts: metas,
            data: swap_data,
        },
        ctx.remaining_accounts,
    )?;
    ctx.accounts.vault.reload()?;
    let received = ctx
        .accounts
        .vault
        .amount
        .checked_sub(before)
        .ok_or(RoundError::MathOverflow)?;
    require!(received >= min_amount, RoundError::SlippageExceeded);
    record_entry(
        &mut ctx.accounts.round,
        &mut ctx.accounts.entry,
        ctx.accounts.player.key(),
        received,
        ctx.bumps.entry,
    )
}
