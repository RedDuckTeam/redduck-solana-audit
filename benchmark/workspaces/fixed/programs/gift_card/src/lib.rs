use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("B2H7JoyQHvc2LoFhMpV2GJ2zyvsV9zLmQF94HScDYenB");

#[program]
pub mod gift_card {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn open_card(ctx: Context<OpenCard>) -> Result<()> {
        instructions::open_card(ctx)
    }

    pub fn load(ctx: Context<Load>, amount: u64) -> Result<()> {
        instructions::load(ctx, amount)
    }

    pub fn transfer(ctx: Context<TransferBalance>, amount: u64, recipient: Pubkey) -> Result<()> {
        instructions::transfer(ctx, amount, recipient)
    }

    pub fn merge_cards(ctx: Context<MergeCards>) -> Result<()> {
        instructions::merge_cards(ctx)
    }

    pub fn redeem(ctx: Context<Redeem>, amount: u64) -> Result<()> {
        instructions::redeem(ctx, amount)
    }

    pub fn close_card(ctx: Context<CloseCard>) -> Result<()> {
        instructions::close_card(ctx)
    }
}
