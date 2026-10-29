use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::CrateError;
use crate::state::House;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct HouseArgs {
    pub cost: u64,
    pub prize: u64,
    pub odds_bps: u16,
}

#[derive(Accounts)]
pub struct InitHouse<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        space = House::SPACE,
        seeds = [House::SEED, token_mint.key().as_ref()],
        bump
    )]
    pub house: Account<'info, House>,

    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = house,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn init_house(ctx: Context<InitHouse>, args: HouseArgs) -> Result<()> {
    require!(args.cost > 0 && args.prize > 0, CrateError::ZeroAmount);
    require!(args.odds_bps > 0 && args.odds_bps <= 10_000, CrateError::BadOdds);
    require!(args.prize >= args.cost, CrateError::BadPrize);

    let house = &mut ctx.accounts.house;
    house.authority = ctx.accounts.authority.key();
    house.token_mint = ctx.accounts.token_mint.key();
    house.vault = ctx.accounts.vault.key();
    house.cost = args.cost;
    house.prize = args.prize;
    house.reserved = 0;
    house.odds_bps = args.odds_bps;
    house.bump = ctx.bumps.house;
    Ok(())
}
