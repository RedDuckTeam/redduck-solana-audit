use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{load_instruction, parse_ed25519, Allowlist, Config, VaultError};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, Config>,
    /// The keys allowed to sign a withdrawal.
    pub allowlist: Account<'info, Allowlist>,
    #[account(address = config.mint)]
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    /// CHECK: the instructions sysvar, read to find the signature check that ran before us.
    pub instructions: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}

/// The message an oracle signs for one withdrawal: vault, recipient token account, amount.
pub fn withdrawal_message(vault: &Pubkey, recipient_tokens: &Pubkey, amount: u64) -> Vec<u8> {
    let mut message = Vec::with_capacity(72);
    message.extend_from_slice(vault.as_ref());
    message.extend_from_slice(recipient_tokens.as_ref());
    message.extend_from_slice(&amount.to_le_bytes());
    message
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let signature_check = load_instruction(0, &ctx.accounts.instructions)?;
    let signed = parse_ed25519(&signature_check)?;
    require!(
        ctx.accounts.allowlist.keys.contains(&signed.pubkey),
        VaultError::KeyNotAllowed
    );
    let expected = withdrawal_message(
        &ctx.accounts.vault.key(),
        &ctx.accounts.recipient_tokens.key(),
        amount,
    );
    require!(signed.message == expected, VaultError::MessageMismatch);

    let seeds: &[&[u8]] = &[b"config", &[ctx.accounts.config.bump]];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.recipient_tokens.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[seeds],
        ),
        amount,
        ctx.accounts.mint.decimals,
    )
}
