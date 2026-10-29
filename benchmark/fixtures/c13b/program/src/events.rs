use anchor_lang::prelude::*;

use crate::state::CreditAccount;

#[event]
pub struct AccountUpdated {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub shares: u64,
    pub collateral: u64,
    pub debt: u64,
}

impl AccountUpdated {
    pub fn of(a: &CreditAccount) -> Self {
        Self { market: a.market, owner: a.owner, shares: a.shares, collateral: a.collateral, debt: a.debt }
    }
}

#[event]
pub struct Liquidated {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub liquidator: Pubkey,
    pub repaid: u64,
    pub seized: u64,
    pub written_off: u64,
}
