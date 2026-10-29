use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("9ChtMQAbXYYRzK8fJbNaaQzsKZbQKu7jhdDWQsxrdmU5");

#[program]
pub mod prize_rounds {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, executor: Pubkey) -> Result<()> {
        instructions::initialize(ctx, executor)
    }

    pub fn open_round(ctx: Context<OpenRound>, id: u64, deadline: i64) -> Result<()> {
        instructions::open_round(ctx, id, deadline)
    }

    pub fn join(ctx: Context<Join>, amount: u64) -> Result<()> {
        instructions::join(ctx, amount)
    }

    pub fn join_with_swap<'info>(
        ctx: Context<'_, '_, 'info, 'info, JoinWithSwap<'info>>,
        swap_data: Vec<u8>,
        min_amount: u64,
    ) -> Result<()> {
        instructions::join_with_swap(ctx, swap_data, min_amount)
    }

    pub fn settle(ctx: Context<Settle>) -> Result<()> {
        instructions::settle(ctx)
    }
}
