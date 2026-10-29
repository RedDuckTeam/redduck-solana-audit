pub mod exit;
pub mod position;
pub mod season;

pub use exit::*;
pub use position::*;
pub use season::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{Season, SEASON_SEED};

/// Moves tokens from a holder's account into a season vault.
pub fn deposit_to_vault<'info>(
    token_program: &Program<'info, Token>,
    from: &Account<'info, TokenAccount>,
    vault: &Account<'info, TokenAccount>,
    owner: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    let accounts = Transfer {
        from: from.to_account_info(),
        to: vault.to_account_info(),
        authority: owner.to_account_info(),
    };
    token::transfer(CpiContext::new(token_program.to_account_info(), accounts), amount)
}

/// Moves tokens out of a season vault; the season PDA signs.
pub fn pay_from_vault<'info>(
    token_program: &Program<'info, Token>,
    season: &Account<'info, Season>,
    vault: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    amount: u64,
) -> Result<()> {
    let id_bytes = season.id.to_le_bytes();
    let bump = [season.bump];
    let seeds: &[&[u8]] = &[SEASON_SEED, season.authority.as_ref(), &id_bytes, &bump];
    let accounts = Transfer {
        from: vault.to_account_info(),
        to: to.to_account_info(),
        authority: season.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(token_program.to_account_info(), accounts, &[seeds]),
        amount,
    )
}
