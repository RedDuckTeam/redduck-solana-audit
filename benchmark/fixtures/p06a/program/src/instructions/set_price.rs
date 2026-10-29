use anchor_lang::prelude::*;

use crate::Config;

/// The team's bot writes the current price.
#[derive(Accounts)]
pub struct SetPrice<'info> {
    pub bot: Signer<'info>,
    #[account(mut, seeds = [b"config"], bump = config.bump, has_one = bot)]
    pub config: Account<'info, Config>,
}

pub fn set_price(ctx: Context<SetPrice>, price: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.price = price;
    config.price_time = Clock::get()?.unix_timestamp;
    Ok(())
}
