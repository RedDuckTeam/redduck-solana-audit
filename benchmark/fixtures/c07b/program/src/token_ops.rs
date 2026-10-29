use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};

use crate::state::Arena;

impl Arena {
    /// Pays `amount` out of the arena vault, signed by the arena PDA.
    pub fn pay_from_vault<'info>(
        &self,
        arena: AccountInfo<'info>,
        vault: AccountInfo<'info>,
        to: AccountInfo<'info>,
        token_program: AccountInfo<'info>,
        amount: u64,
    ) -> Result<()> {
        let mint = self.token_mint;
        let seeds = self.signer_seeds(&mint);
        let accounts = Transfer { from: vault, to, authority: arena };
        token::transfer(CpiContext::new_with_signer(token_program, accounts, &[&seeds]), amount)
    }
}

/// Moves `amount` tokens with a transfer authorised by the signing `authority`.
pub fn move_tokens<'info>(
    from: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let accounts = Transfer { from, to, authority };
    token::transfer(CpiContext::new(token_program, accounts), amount)
}
