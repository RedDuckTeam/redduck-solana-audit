use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{Pool, PoolError, Staker};

/// Funds a reward and credits it to the stakers in proportion to their stake.
#[derive(Accounts)]
pub struct AddRewards<'info> {
    pub funder: Signer<'info>,
    #[account(seeds = [b"pool"], bump = pool.bump)]
    pub pool: Account<'info, Pool>,
    #[account(address = pool.mint)]
    pub mint: Account<'info, Mint>,
    #[account(mut, token::mint = mint, token::authority = funder)]
    pub funder_tokens: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn add_rewards<'info>(
    ctx: Context<'_, '_, 'info, 'info, AddRewards<'info>>,
    amount: u64,
) -> Result<()> {
    let total_staked = ctx.accounts.pool.total_staked;
    require!(total_staked > 0, PoolError::NothingStaked);
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.funder_tokens.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.funder.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    for account in ctx.remaining_accounts {
        require_keys_eq!(*account.owner, crate::ID, PoolError::NotAStaker);
        let mut data = account.try_borrow_mut_data()?;
        let mut staker = Staker::try_deserialize(&mut &data[..])?;
        let share = (amount as u128)
            .checked_mul(staker.stake as u128)
            .ok_or(PoolError::MathOverflow)?
            / total_staked as u128;
        staker.pending = staker
            .pending
            .checked_add(u64::try_from(share).map_err(|_| PoolError::MathOverflow)?)
            .ok_or(PoolError::MathOverflow)?;
        let mut cursor: &mut [u8] = &mut data;
        staker.try_serialize(&mut cursor)?;
    }
    Ok(())
}
