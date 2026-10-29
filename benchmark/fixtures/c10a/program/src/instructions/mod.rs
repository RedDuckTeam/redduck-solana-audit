pub mod admin;
pub mod convert;
pub mod liquidity;
pub mod rate;

pub use admin::*;
pub use convert::*;
pub use liquidity::*;
pub use rate::*;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{Desk, DESK_SEED};

/// Transfer signed by the token owner.
pub fn owner_transfer<'info>(
    token_program: &Program<'info, Token>,
    from: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    owner: &Signer<'info>,
    amount: u64,
) -> Result<()> {
    let accounts = Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: owner.to_account_info(),
    };
    token::transfer(CpiContext::new(token_program.to_account_info(), accounts), amount)
}

/// Transfer out of a desk vault; the desk PDA signs.
pub fn desk_transfer<'info>(
    token_program: &Program<'info, Token>,
    desk: &Account<'info, Desk>,
    from: &Account<'info, TokenAccount>,
    to: &Account<'info, TokenAccount>,
    amount: u64,
) -> Result<()> {
    let bump = [desk.bump];
    let seeds: &[&[u8]] = &[
        DESK_SEED,
        desk.admin.as_ref(),
        desk.base_mint.as_ref(),
        desk.quote_mint.as_ref(),
        &bump,
    ];
    let accounts = Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: desk.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(token_program.to_account_info(), accounts, &[seeds]),
        amount,
    )
}
