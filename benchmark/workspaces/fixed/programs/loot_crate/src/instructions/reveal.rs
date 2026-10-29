use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::slot_hashes;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::errors::CrateError;
use crate::events::Revealed;
use crate::state::{Commitment, House};

#[derive(Accounts)]
pub struct Reveal<'info> {
    pub caller: Signer<'info>,

    #[account(mut, seeds = [House::SEED, house.token_mint.as_ref()], bump = house.bump)]
    pub house: Account<'info, House>,

    #[account(
        mut,
        close = player,
        has_one = house,
        seeds = [Commitment::SEED, house.key().as_ref(), player.key().as_ref()],
        bump = commitment.bump,
    )]
    pub commitment: Account<'info, Commitment>,

    /// CHECK: receives the closed commitment's rent; pinned to the recorded player.
    #[account(mut, address = commitment.player)]
    pub player: UncheckedAccount<'info>,

    #[account(mut, address = house.vault)]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = house.token_mint,
        constraint = player_token.owner == commitment.player,
    )]
    pub player_token: Account<'info, TokenAccount>,

    /// CHECK: validated by address against the SlotHashes sysvar.
    #[account(address = slot_hashes::id())]
    pub slot_hashes: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

fn hash_for_slot(ai: &AccountInfo, target: u64) -> Result<Option<[u8; 32]>> {
    let data = ai.try_borrow_data()?;
    require!(data.len() >= 8, CrateError::BadSlotHashes);
    let count = u64::from_le_bytes(data[0..8].try_into().map_err(|_| CrateError::BadSlotHashes)?) as usize;
    for i in 0..count {
        let off = 8 + i * 40;
        if off + 40 > data.len() {
            break;
        }
        let slot = u64::from_le_bytes(data[off..off + 8].try_into().unwrap());
        if slot == target {
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&data[off + 8..off + 40]);
            return Ok(Some(hash));
        }
    }
    Ok(None)
}

pub fn reveal(ctx: Context<Reveal>) -> Result<()> {
    let clock = Clock::get()?;
    require!(clock.slot > ctx.accounts.commitment.reveal_slot, CrateError::TooEarly);

    let prize = ctx.accounts.commitment.prize;
    let odds = ctx.accounts.commitment.odds_bps;
    let mut won = false;

    // A hash for the committed slot resolves the crate; a missing one (a skipped slot,
    // or a slot too old to remain in the sysvar) settles it as a loss.
    if let Some(hash) = hash_for_slot(&ctx.accounts.slot_hashes, ctx.accounts.commitment.reveal_slot)? {
        let roll = u64::from_le_bytes(hash[0..8].try_into().unwrap()) % 10_000;
        if (roll as u16) < odds {
            won = true;
            let mint = ctx.accounts.house.token_mint;
            let seeds = ctx.accounts.house.signer_seeds(&mint);
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.vault.to_account_info(),
                        to: ctx.accounts.player_token.to_account_info(),
                        authority: ctx.accounts.house.to_account_info(),
                    },
                    &[&seeds],
                ),
                prize,
            )?;
        }
    }

    let player = ctx.accounts.commitment.player;
    let house = &mut ctx.accounts.house;
    house.reserved = house.reserved.saturating_sub(prize);
    emit!(Revealed {
        house: house.key(),
        player,
        won,
        payout: if won { prize } else { 0 },
    });
    Ok(())
}
