use anchor_lang::prelude::*;

use crate::Device;

#[derive(Accounts)]
pub struct RegisterDevice<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + Device::INIT_SPACE,
        seeds = [b"device", owner.key().as_ref()],
        bump,
    )]
    pub device: Account<'info, Device>,
    pub system_program: Program<'info, System>,
}

pub fn register_device(ctx: Context<RegisterDevice>) -> Result<()> {
    ctx.accounts.device.set_inner(Device {
        owner: ctx.accounts.owner.key(),
        nonce: 0,
        bump: ctx.bumps.device,
    });
    Ok(())
}
