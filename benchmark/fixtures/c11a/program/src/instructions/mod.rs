use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::Config;

pub mod admin;
pub mod card;
pub mod processor;

pub use admin::*;
pub use card::*;
pub use processor::*;

/// Transfers `amount` out of the shared vault, signed by the config PDA.
pub(crate) fn pay_from_vault<'info>(
    token_program: &Program<'info, Token>,
    config: &Account<'info, Config>,
    vault: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    amount: u64,
) -> Result<()> {
    let bump = [config.bump];
    let seeds = config.signer_seeds(&bump);
    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: vault.to_account_info(),
                to: to.to_account_info(),
                authority: config.to_account_info(),
            },
            &[&seeds],
        ),
        amount,
    )
}
