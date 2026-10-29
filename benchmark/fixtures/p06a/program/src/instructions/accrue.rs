use anchor_lang::prelude::*;

use crate::{Config, RewardsError, User};

/// The admin credits a user with a reward.
#[derive(Accounts)]
pub struct Accrue<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(seeds = [b"config"], bump = config.bump, has_one = admin)]
    pub config: Account<'info, Config>,
    /// CHECK: the user the reward is credited to; only their key is needed.
    pub owner: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", owner.key().as_ref()],
        bump,
    )]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

pub fn accrue(ctx: Context<Accrue>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.owner = ctx.accounts.owner.key();
    user.bump = ctx.bumps.user;
    user.pending = user
        .pending
        .checked_add(amount)
        .ok_or(RewardsError::MathOverflow)?;
    Ok(())
}
