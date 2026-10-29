use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions::ID as INSTRUCTIONS_SYSVAR_ID;

use crate::errors::FxError;
use crate::events::RatePosted;
use crate::quote::{quote_message, verify_signed_message};
use crate::state::*;

#[derive(Accounts)]
pub struct PostRate<'info> {
    pub submitter: Signer<'info>,
    #[account(mut, constraint = desk.operator == submitter.key() @ FxError::Unauthorized)]
    pub desk: Account<'info, Desk>,
    /// CHECK: the instructions sysvar, pinned by address.
    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    pub instructions: UncheckedAccount<'info>,
}

/// Records a publisher-signed quote as the desk's current rate.
pub fn post_rate(ctx: Context<PostRate>, rate: u64, published_at: i64) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let desk = &mut ctx.accounts.desk;
    require!(rate > 0, FxError::InvalidRate);
    require!(published_at <= now, FxError::QuoteFromFuture);
    require!(published_at > desk.rate_published_at, FxError::StaleQuote);

    let message = quote_message(&desk.base_mint, &desk.quote_mint, rate, published_at);
    verify_signed_message(&ctx.accounts.instructions, &desk.publisher, &message)?;

    desk.rate = rate;
    desk.rate_published_at = published_at;
    emit!(RatePosted {
        desk: desk.key(),
        rate,
        published_at,
        submitted_by: ctx.accounts.submitter.key(),
    });
    Ok(())
}
