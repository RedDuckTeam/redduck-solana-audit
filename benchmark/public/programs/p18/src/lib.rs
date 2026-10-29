use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("F55g44G7zaF65cpKJ1zb2enJqmeBhwqGaV75g1GQ79ya");

#[program]
pub mod treasury_council {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, members: Vec<Pubkey>, threshold: u8) -> Result<()> {
        instructions::initialize(ctx, members, threshold)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        id: u64,
        program_id: Pubkey,
        metas: Vec<StoredMeta>,
        data: Vec<u8>,
        expires_at: i64,
    ) -> Result<()> {
        instructions::create_proposal(ctx, id, program_id, metas, data, expires_at)
    }

    pub fn approve(ctx: Context<Approve>) -> Result<()> {
        instructions::approve(ctx)
    }

    pub fn execute<'info>(ctx: Context<'_, '_, 'info, 'info, Execute<'info>>) -> Result<()> {
        instructions::execute(ctx)
    }
}
