use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::Deal;

pub mod buyer;
pub mod merchant;

pub use buyer::*;
pub use merchant::*;

/// Pays `amount` out of a deal's escrow, signed by the deal PDA.
pub(crate) fn pay_from_escrow<'info>(
    token_program: &Program<'info, Token>,
    deal: &Account<'info, Deal>,
    escrow: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    amount: u64,
) -> Result<()> {
    let id = deal.id.to_le_bytes();
    let bump = [deal.bump];
    let seeds = deal.signer_seeds(&id, &bump);
    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: escrow.to_account_info(),
                to: to.to_account_info(),
                authority: deal.to_account_info(),
            },
            &[&seeds],
        ),
        amount,
    )
}
