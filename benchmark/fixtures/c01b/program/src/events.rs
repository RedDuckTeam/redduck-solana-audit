use anchor_lang::prelude::*;

#[event]
pub struct Borrowed {
    pub market: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub debt: u64,
}
