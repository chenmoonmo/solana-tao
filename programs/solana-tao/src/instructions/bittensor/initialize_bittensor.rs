use anchor_lang::prelude::*;

use crate::states::*;

pub fn initialize_bittensor(_ctx: Context<InitializeBittensor>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeBittensor<'info> {
    #[account(
        init,
        payer = owner,
        space = 10 * 1024 as usize,
        seeds = [b"system".as_ref()],
        bump
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
