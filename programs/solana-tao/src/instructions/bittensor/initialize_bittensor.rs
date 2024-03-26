use anchor_lang::prelude::*;

use crate::states::*;

pub fn initialize_bittensor(ctx: Context<InitializeBittensor>) -> Result<()> {
    let bittensor_state = &mut ctx.accounts.bittensor_state;
    bittensor_state.owner = ctx.accounts.owner.key();
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeBittensor<'info> {
    #[account(
        init,
        payer = owner,
        // TODO: 设置初始大小 并在添加子网时 动态分配大小
        space = 8 + BittensorState::LEN,
        seeds = [b"system"],
        bump
    )]
    pub bittensor_state: Box<Account<'info, BittensorState>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
