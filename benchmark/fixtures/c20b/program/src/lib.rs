use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("2THq8UQkjiYpdYDoCWxvsh3zeAShceAFzmb56SQ4ovyk");

/// A market still unresolved this long after staking closed refunds every stake.
pub const RESOLVE_WINDOW: i64 = 14 * 24 * 60 * 60;

#[program]
pub mod outcome_pools {
    use super::*;

    /// Registers the signer as a reporter. Market creators choose which reporter settles their market.
    pub fn register_reporter(ctx: Context<RegisterReporter>, name: [u8; 32]) -> Result<()> {
        let reporter = &mut ctx.accounts.reporter;
        reporter.authority = ctx.accounts.authority.key();
        reporter.name = name;
        reporter.reports = 0;
        reporter.bump = ctx.bumps.reporter;
        Ok(())
    }

    /// Publishes the result of a finished event. Each reporter posts an event once.
    pub fn post_report(ctx: Context<PostReport>, event_id: u64, result: Side) -> Result<()> {
        let report = &mut ctx.accounts.report;
        report.reporter = ctx.accounts.reporter.key();
        report.event_id = event_id;
        report.result = result;
        report.posted_at = Clock::get()?.unix_timestamp;
        report.bump = ctx.bumps.report;

        let reporter = &mut ctx.accounts.reporter;
        reporter.reports = reporter.reports.checked_add(1).ok_or(PoolError::MathOverflow)?;
        Ok(())
    }

    /// Opens a two-sided pool on an event. Staking closes at `closes_at`, before the event starts.
    pub fn create_market(ctx: Context<CreateMarket>, event_id: u64, closes_at: i64) -> Result<()> {
        require!(closes_at > Clock::get()?.unix_timestamp, PoolError::BadCloseTime);

        let market = &mut ctx.accounts.market;
        market.creator = ctx.accounts.creator.key();
        market.reporter = ctx.accounts.reporter.key();
        market.mint = ctx.accounts.mint.key();
        market.vault = ctx.accounts.vault.key();
        market.event_id = event_id;
        market.closes_at = closes_at;
        market.pools = [0; 2];
        market.winner = None;
        market.bump = ctx.bumps.market;
        Ok(())
    }

    pub fn place_stake(ctx: Context<PlaceStake>, side: Side, amount: u64) -> Result<()> {
        require!(amount > 0, PoolError::ZeroAmount);
        require!(Clock::get()?.unix_timestamp < ctx.accounts.market.closes_at, PoolError::MarketClosed);

        let accounts = Transfer {
            from: ctx.accounts.owner_token.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts), amount)?;

        let market = &mut ctx.accounts.market;
        let pool = &mut market.pools[side.index()];
        *pool = pool.checked_add(amount).ok_or(PoolError::MathOverflow)?;

        let ticket = &mut ctx.accounts.ticket;
        ticket.owner = ctx.accounts.owner.key();
        ticket.market = market.key();
        ticket.side = side;
        ticket.amount = amount;
        ticket.bump = ctx.bumps.ticket;
        Ok(())
    }

    /// Records the reported result as the winning side. Anyone can call this once staking has closed.
    pub fn resolve(ctx: Context<Resolve>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let now = Clock::get()?.unix_timestamp;
        require!(now >= market.closes_at, PoolError::MarketOpen);
        require!(now < market.closes_at.saturating_add(RESOLVE_WINDOW), PoolError::ResolveWindowOver);
        require!(market.winner.is_none(), PoolError::AlreadyResolved);
        require!(ctx.accounts.report.posted_at >= market.closes_at, PoolError::EarlyReport);

        let result = ctx.accounts.report.result;
        market.winner = Some(result);
        let report = ctx.accounts.report.key();
        emit!(MarketResolved { market: market.key(), event_id: market.event_id, winner: result, report });
        Ok(())
    }

    /// Pays a ticket out and closes it. Losing tickets are closed with no payout.
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let market = &ctx.accounts.market;
        let payout = match market.winner {
            Some(winner) => market.payout(winner, &ctx.accounts.ticket)?,
            None => {
                let refund_from = market.closes_at.saturating_add(RESOLVE_WINDOW);
                require!(Clock::get()?.unix_timestamp >= refund_from, PoolError::NotResolved);
                ctx.accounts.ticket.amount
            }
        };
        if payout > 0 {
            let event_id = market.event_id.to_le_bytes();
            let bump = [market.bump];
            let seeds: &[&[u8]] = &[b"market".as_ref(), market.creator.as_ref(), &event_id, &bump];
            let accounts = Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.owner_token.to_account_info(),
                authority: market.to_account_info(),
            };
            let program = ctx.accounts.token_program.to_account_info();
            token::transfer(CpiContext::new_with_signer(program, accounts, &[seeds]), payout)?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterReporter<'info> {
    #[account(init, payer = authority, space = 8 + Reporter::INIT_SPACE, seeds = [b"reporter", authority.key().as_ref()], bump)]
    pub reporter: Account<'info, Reporter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct PostReport<'info> {
    #[account(mut, has_one = authority)]
    pub reporter: Account<'info, Reporter>,
    #[account(init, payer = authority, space = 8 + Report::INIT_SPACE, seeds = [b"report", reporter.key().as_ref(), &event_id.to_le_bytes()], bump)]
    pub report: Account<'info, Report>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id: u64)]
pub struct CreateMarket<'info> {
    #[account(init, payer = creator, space = 8 + Market::INIT_SPACE, seeds = [b"market", creator.key().as_ref(), &event_id.to_le_bytes()], bump)]
    pub market: Account<'info, Market>,
    #[account(init, payer = creator, seeds = [b"vault", market.key().as_ref()], bump, token::mint = mint, token::authority = market)]
    pub vault: Account<'info, TokenAccount>,
    pub reporter: Account<'info, Reporter>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceStake<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    #[account(mut, address = market.vault)]
    pub vault: Account<'info, TokenAccount>,
    #[account(init, payer = owner, space = 8 + Ticket::INIT_SPACE, seeds = [b"ticket", market.key().as_ref(), owner.key().as_ref()], bump)]
    pub ticket: Account<'info, Ticket>,
    #[account(mut, token::mint = market.mint)]
    pub owner_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Resolve<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    pub reporter: Account<'info, Reporter>,
    #[account(has_one = reporter, constraint = report.event_id == market.event_id @ PoolError::WrongEvent)]
    pub report: Account<'info, Report>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    pub market: Account<'info, Market>,
    #[account(mut, address = market.vault)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, close = owner, has_one = owner, has_one = market)]
    pub ticket: Account<'info, Ticket>,
    #[account(mut, token::mint = market.mint)]
    pub owner_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum Side {
    Yes,
    No,
}

impl Side {
    pub fn index(self) -> usize {
        self as usize
    }
}

#[account]
#[derive(InitSpace)]
pub struct Reporter {
    pub authority: Pubkey,
    pub name: [u8; 32],
    pub reports: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Report {
    pub reporter: Pubkey,
    pub event_id: u64,
    pub result: Side,
    pub posted_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub creator: Pubkey,
    pub reporter: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub event_id: u64,
    pub closes_at: i64,
    pub pools: [u64; 2],
    pub winner: Option<Side>,
    pub bump: u8,
}

impl Market {
    /// Winners split both pools pro rata to their stake. If nobody backed the winner, stakes are refunded.
    pub fn payout(&self, winner: Side, ticket: &Ticket) -> Result<u64> {
        let winning = self.pools[winner.index()] as u128;
        if winning == 0 {
            return Ok(ticket.amount);
        }
        if ticket.side != winner {
            return Ok(0);
        }
        let total = self.pools[0] as u128 + self.pools[1] as u128;
        let share = (ticket.amount as u128).checked_mul(total).ok_or(PoolError::MathOverflow)? / winning;
        u64::try_from(share).map_err(|_| error!(PoolError::MathOverflow))
    }
}

#[account]
#[derive(InitSpace)]
pub struct Ticket {
    pub owner: Pubkey,
    pub market: Pubkey,
    pub side: Side,
    pub amount: u64,
    pub bump: u8,
}

#[event]
pub struct MarketResolved {
    pub market: Pubkey,
    pub event_id: u64,
    pub winner: Side,
    pub report: Pubkey,
}

#[error_code]
pub enum PoolError {
    #[msg("Close time must be in the future")]
    BadCloseTime,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Staking for this market has closed")]
    MarketClosed,
    #[msg("Staking for this market is still open")]
    MarketOpen,
    #[msg("Market is already resolved")]
    AlreadyResolved,
    #[msg("Market can no longer be resolved")]
    ResolveWindowOver,
    #[msg("Report was posted before staking closed")]
    EarlyReport,
    #[msg("Market is not resolved yet")]
    NotResolved,
    #[msg("Report is for a different event")]
    WrongEvent,
    #[msg("Arithmetic overflow")]
    MathOverflow,
}
