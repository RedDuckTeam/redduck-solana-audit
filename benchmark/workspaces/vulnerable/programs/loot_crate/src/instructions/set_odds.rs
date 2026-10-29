use anchor_lang::prelude::*;

use crate::errors::CrateError;
use crate::state::House;

#[derive(Accounts)]
pub struct SetOdds<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [House::SEED, house.token_mint.as_ref()],
        bump = house.bump,
        has_one = authority @ CrateError::NotAuthority,
    )]
    pub house: Account<'info, House>,
}

pub fn set_odds(ctx: Context<SetOdds>, odds_bps: u16, prize: u64) -> Result<()> {
    require!(odds_bps > 0 && odds_bps <= 10_000, CrateError::BadOdds);
    require!(prize > 0, CrateError::ZeroAmount);
    require!(prize >= ctx.accounts.house.cost, CrateError::BadPrize);
    let house = &mut ctx.accounts.house;
    house.odds_bps = odds_bps;
    house.prize = prize;
    Ok(())
}
