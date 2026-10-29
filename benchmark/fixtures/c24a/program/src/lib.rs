use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("EU3C8W9eoQbFYWHvFiubw6A4Crka9v2tsrkxDcxqZWtn");

pub const MIN_EPOCH_LENGTH: i64 = 60 * 60;
pub const MAX_EPOCH_LENGTH: i64 = 30 * 24 * 60 * 60;

#[program]
pub mod points_staking {
    use super::*;

    pub fn create_pool(ctx: Context<CreatePool>, epoch_length: i64) -> Result<()> {
        require!(
            (MIN_EPOCH_LENGTH..=MAX_EPOCH_LENGTH).contains(&epoch_length),
            StakingError::BadEpochLength
        );
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.stake_mint = ctx.accounts.stake_mint.key();
        pool.vault = ctx.accounts.vault.key();
        pool.epoch = 0;
        pool.epoch_started_at = Clock::get()?.unix_timestamp;
        pool.epoch_length = epoch_length;
        pool.total_staked = 0;
        pool.bump = ctx.bumps.pool;
        Ok(())
    }

    pub fn open_account(ctx: Context<OpenAccount>) -> Result<()> {
        let staker = &mut ctx.accounts.staker;
        staker.pool = ctx.accounts.pool.key();
        staker.owner = ctx.accounts.owner.key();
        staker.staked = 0;
        staker.bump = ctx.bumps.staker;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        require!(amount > 0, StakingError::ZeroAmount);
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.owner_token.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;

        let staker = &mut ctx.accounts.staker;
        staker.staked = staker.staked.checked_add(amount).ok_or(StakingError::MathOverflow)?;
        let pool = &mut ctx.accounts.pool;
        pool.total_staked = pool.total_staked.checked_add(amount).ok_or(StakingError::MathOverflow)?;
        Ok(())
    }

    /// Moves `amount` out of the active stake. The tokens can be withdrawn after the
    /// current epoch has been closed.
    pub fn request_unstake(ctx: Context<RequestUnstake>, amount: u64) -> Result<()> {
        require!(amount > 0, StakingError::ZeroAmount);
        let staker = &mut ctx.accounts.staker;
        staker.staked = staker.staked.checked_sub(amount).ok_or(StakingError::InsufficientStake)?;
        let pool = &mut ctx.accounts.pool;
        pool.total_staked = pool.total_staked.checked_sub(amount).ok_or(StakingError::MathOverflow)?;

        let request = &mut ctx.accounts.request;
        request.pool = pool.key();
        request.owner = ctx.accounts.owner.key();
        request.amount = amount;
        request.epoch = pool.epoch;
        request.bump = ctx.bumps.request;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let pool = &ctx.accounts.pool;
        let request = &ctx.accounts.request;
        require!(pool.epoch > request.epoch, StakingError::EpochNotClosed);

        let bump = [pool.bump];
        let seeds: &[&[u8]] = &[
            b"pool".as_ref(),
            pool.authority.as_ref(),
            pool.stake_mint.as_ref(),
            &bump,
        ];
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.owner_token.to_account_info(),
                    authority: pool.to_account_info(),
                },
                &[seeds],
            ),
            request.amount,
        )
    }

    /// Closes the current epoch once its length has passed and emits the totals used by
    /// the points engine.
    pub fn close_epoch(ctx: Context<CloseEpoch>) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let pool = &mut ctx.accounts.pool;
        let ends_at = pool
            .epoch_started_at
            .checked_add(pool.epoch_length)
            .ok_or(StakingError::MathOverflow)?;
        require!(now >= ends_at, StakingError::EpochNotOver);

        emit!(EpochClosed {
            pool: pool.key(),
            epoch: pool.epoch,
            total_staked: pool.total_staked,
            closed_at: now,
        });
        pool.epoch = pool.epoch.checked_add(1).ok_or(StakingError::MathOverflow)?;
        pool.epoch_started_at = ends_at;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = authority, space = 8 + Pool::INIT_SPACE, seeds = [b"pool", authority.key().as_ref(), stake_mint.key().as_ref()], bump)]
    pub pool: Account<'info, Pool>,
    #[account(init, payer = authority, seeds = [b"vault", pool.key().as_ref()], bump, token::mint = stake_mint, token::authority = pool)]
    pub vault: Account<'info, TokenAccount>,
    pub stake_mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenAccount<'info> {
    pub pool: Account<'info, Pool>,
    #[account(init, payer = owner, space = 8 + Staker::INIT_SPACE, seeds = [b"staker", pool.key().as_ref(), owner.key().as_ref()], bump)]
    pub staker: Account<'info, Staker>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut, has_one = pool, has_one = owner)]
    pub staker: Account<'info, Staker>,
    #[account(mut, address = pool.vault)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = pool.stake_mint)]
    pub owner_token: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RequestUnstake<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut, has_one = pool, has_one = owner)]
    pub staker: Account<'info, Staker>,
    #[account(init, payer = owner, space = 8 + Unbonding::INIT_SPACE, seeds = [b"unbonding", pool.key().as_ref(), owner.key().as_ref()], bump)]
    pub request: Account<'info, Unbonding>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub pool: Account<'info, Pool>,
    #[account(mut, close = owner, has_one = pool, has_one = owner)]
    pub request: Account<'info, Unbonding>,
    #[account(mut, address = pool.vault)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = pool.stake_mint)]
    pub owner_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CloseEpoch<'info> {
    #[account(mut, has_one = authority)]
    pub pool: Account<'info, Pool>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub authority: Pubkey,
    pub stake_mint: Pubkey,
    pub vault: Pubkey,
    pub epoch: u64,
    pub epoch_started_at: i64,
    pub epoch_length: i64,
    pub total_staked: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Staker {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub staked: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Unbonding {
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub epoch: u64,
    pub bump: u8,
}

#[event]
pub struct EpochClosed {
    pub pool: Pubkey,
    pub epoch: u64,
    pub total_staked: u64,
    pub closed_at: i64,
}

#[error_code]
pub enum StakingError {
    #[msg("Epoch length is out of range")]
    BadEpochLength,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Not enough active stake")]
    InsufficientStake,
    #[msg("The epoch of this request has not been closed yet")]
    EpochNotClosed,
    #[msg("The current epoch has not ended")]
    EpochNotOver,
    #[msg("Arithmetic overflow")]
    MathOverflow,
}
