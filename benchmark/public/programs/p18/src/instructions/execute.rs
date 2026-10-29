use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::invoke_signed;

use crate::{Config, MultisigError, Proposal};

/// Runs the stored instruction with the vault PDA as the signer.
#[derive(Accounts)]
pub struct Execute<'info> {
    pub admin: Signer<'info>,
    #[account(seeds = [b"config"], bump = config.bump, has_one = admin)]
    pub config: Account<'info, Config>,
    #[account(mut, seeds = [b"proposal", proposal.id.to_le_bytes().as_ref()], bump = proposal.bump)]
    pub proposal: Account<'info, Proposal>,
    /// CHECK: the treasury PDA that signs the stored instruction.
    #[account(seeds = [b"vault"], bump = config.vault_bump)]
    pub vault: UncheckedAccount<'info>,
}

pub fn execute<'info>(ctx: Context<'_, '_, 'info, 'info, Execute<'info>>) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    require!(
        proposal.approvals >= ctx.accounts.config.threshold,
        MultisigError::NotApproved
    );
    let metas = proposal
        .metas
        .iter()
        .map(|meta| AccountMeta {
            pubkey: meta.pubkey,
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect();
    let seeds: &[&[u8]] = &[b"vault", &[ctx.accounts.config.vault_bump]];
    invoke_signed(
        &Instruction {
            program_id: proposal.program_id,
            accounts: metas,
            data: proposal.data.clone(),
        },
        ctx.remaining_accounts,
        &[seeds],
    )?;
    ctx.accounts.proposal.executed = true;
    Ok(())
}
