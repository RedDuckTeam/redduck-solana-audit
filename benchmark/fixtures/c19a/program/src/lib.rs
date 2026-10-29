use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hashv;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("8KmXrw5wx1yocDY4A7mEPh2LRNdeH79spYZ5d3K1NRqL");

/// Domain prefixes keep leaf hashes and inner node hashes apart.
const LEAF_PREFIX: u8 = 0;
const NODE_PREFIX: u8 = 1;

#[program]
pub mod affiliate_rewards {
    use super::*;

    /// Creates a campaign that pays affiliates in `mint`. The signer publishes the epoch roots.
    pub fn create_campaign(ctx: Context<CreateCampaign>) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        campaign.authority = ctx.accounts.authority.key();
        campaign.mint = ctx.accounts.mint.key();
        campaign.vault = ctx.accounts.vault.key();
        campaign.epochs = 0;
        campaign.total_claimed = 0;
        campaign.bump = ctx.bumps.campaign;
        Ok(())
    }

    /// Adds reward tokens to the campaign vault. Anyone can top it up.
    pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()> {
        require!(amount > 0, RewardsError::ZeroAmount);
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.funder_token.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                    authority: ctx.accounts.funder.to_account_info(),
                },
            ),
            amount,
        )
    }

    /// Publishes the root for the next epoch. Each leaf holds an affiliate's total earnings
    /// since the campaign started.
    pub fn publish_epoch(ctx: Context<PublishEpoch>, root: [u8; 32]) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let epoch = &mut ctx.accounts.epoch;
        epoch.campaign = campaign.key();
        epoch.index = campaign.epochs;
        epoch.root = root;
        epoch.published_at = Clock::get()?.unix_timestamp;
        epoch.bump = ctx.bumps.epoch;
        campaign.epochs = campaign.epochs.checked_add(1).ok_or(RewardsError::MathOverflow)?;

        emit!(EpochPublished {
            campaign: campaign.key(),
            index: epoch.index,
            root,
        });
        Ok(())
    }

    pub fn register(ctx: Context<Register>) -> Result<()> {
        let affiliate = &mut ctx.accounts.affiliate;
        affiliate.campaign = ctx.accounts.campaign.key();
        affiliate.owner = ctx.accounts.owner.key();
        affiliate.claimed = 0;
        affiliate.bump = ctx.bumps.affiliate;
        Ok(())
    }

    /// Pays the part of `earned_total` that has not been paid yet.
    pub fn claim(ctx: Context<Claim>, earned_total: u64, proof: Vec<[u8; 32]>) -> Result<()> {
        let owner = ctx.accounts.owner.key();
        let leaf = hashv(&[
            &[LEAF_PREFIX],
            ctx.accounts.campaign.key().as_ref(),
            owner.as_ref(),
            &earned_total.to_le_bytes(),
        ])
        .to_bytes();
        require!(
            verify_proof(&proof, &ctx.accounts.epoch.root, leaf),
            RewardsError::InvalidProof
        );

        let affiliate = &mut ctx.accounts.affiliate;
        require!(earned_total > affiliate.claimed, RewardsError::NothingToClaim);
        let owed = earned_total - affiliate.claimed;
        affiliate.claimed = earned_total;

        if owed > 0 {
            let campaign = &ctx.accounts.campaign;
            let bump = [campaign.bump];
            let seeds: &[&[u8]] = &[
                b"campaign".as_ref(),
                campaign.authority.as_ref(),
                campaign.mint.as_ref(),
                &bump,
            ];
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.vault.to_account_info(),
                        to: ctx.accounts.owner_token.to_account_info(),
                        authority: campaign.to_account_info(),
                    },
                    &[seeds],
                ),
                owed,
            )?;
        }

        let campaign = &mut ctx.accounts.campaign;
        campaign.total_claimed = campaign
            .total_claimed
            .checked_add(owed)
            .ok_or(RewardsError::MathOverflow)?;

        emit!(RewardClaimed {
            campaign: campaign.key(),
            owner,
            epoch: ctx.accounts.epoch.index,
            earned_total,
            paid: owed,
        });
        Ok(())
    }
}

/// Sorted-pair Merkle proof check.
fn verify_proof(proof: &[[u8; 32]], root: &[u8; 32], leaf: [u8; 32]) -> bool {
    let mut node = leaf;
    for sibling in proof {
        node = if node <= *sibling {
            hashv(&[&[NODE_PREFIX], &node, sibling]).to_bytes()
        } else {
            hashv(&[&[NODE_PREFIX], sibling, &node]).to_bytes()
        };
    }
    node == *root
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Campaign::INIT_SPACE,
        seeds = [b"campaign", authority.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(
        init,
        payer = authority,
        seeds = [b"vault", campaign.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = campaign
    )]
    pub vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Fund<'info> {
    pub campaign: Account<'info, Campaign>,
    #[account(mut, address = campaign.vault)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = campaign.mint)]
    pub funder_token: Account<'info, TokenAccount>,
    pub funder: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct PublishEpoch<'info> {
    #[account(mut, has_one = authority)]
    pub campaign: Account<'info, Campaign>,
    #[account(
        init,
        payer = authority,
        space = 8 + EpochRoot::INIT_SPACE,
        seeds = [b"epoch", campaign.key().as_ref(), &campaign.epochs.to_le_bytes()],
        bump
    )]
    pub epoch: Account<'info, EpochRoot>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Register<'info> {
    pub campaign: Account<'info, Campaign>,
    #[account(
        init,
        payer = owner,
        space = 8 + Affiliate::INIT_SPACE,
        seeds = [b"affiliate", campaign.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    pub affiliate: Account<'info, Affiliate>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(has_one = campaign)]
    pub epoch: Account<'info, EpochRoot>,
    #[account(
        mut,
        has_one = campaign,
        has_one = owner,
        seeds = [b"affiliate", campaign.key().as_ref(), owner.key().as_ref()],
        bump = affiliate.bump
    )]
    pub affiliate: Account<'info, Affiliate>,
    #[account(mut, address = campaign.vault)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = campaign.mint)]
    pub owner_token: Account<'info, TokenAccount>,
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct Campaign {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub epochs: u64,
    pub total_claimed: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct EpochRoot {
    pub campaign: Pubkey,
    pub index: u64,
    pub root: [u8; 32],
    pub published_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Affiliate {
    pub campaign: Pubkey,
    pub owner: Pubkey,
    pub claimed: u64,
    pub bump: u8,
}

#[event]
pub struct EpochPublished {
    pub campaign: Pubkey,
    pub index: u64,
    pub root: [u8; 32],
}

#[event]
pub struct RewardClaimed {
    pub campaign: Pubkey,
    pub owner: Pubkey,
    pub epoch: u64,
    pub earned_total: u64,
    pub paid: u64,
}

#[error_code]
pub enum RewardsError {
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Merkle proof does not match the epoch root")]
    InvalidProof,
    #[msg("Nothing new to claim for this total")]
    NothingToClaim,
    #[msg("Arithmetic overflow")]
    MathOverflow,
}
