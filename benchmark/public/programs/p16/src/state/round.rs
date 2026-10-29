use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Round {
    pub id: u64,
    pub deadline: i64,
    pub pot: u64,
    pub settled: bool,
    pub bump: u8,
}
