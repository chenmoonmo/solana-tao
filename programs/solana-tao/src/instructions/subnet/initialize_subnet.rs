use crate::states::*;
use anchor_lang::prelude::*;

pub fn initialize_subnet(ctx: Context<InitializeSubnet>) -> Result<()> {
    let epoch = Clock::get().unwrap().epoch;
    msg!("Current Epoch: {}", epoch);

    let subnet_state = &mut *ctx.accounts.subnet_state;
    subnet_state.owner = ctx.accounts.owner.key();

    ctx.accounts.bittensor_state.load_mut()?.subnet_num += 1;

    ctx.accounts
        .bittensor_state
        .load_mut()?
        .create_subnet(ctx.accounts.owner.key());

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnet<'info> {
    #[account(
        mut,
        seeds = [b"system".as_ref()],
        bump,
    )]
    pub bittensor_state: AccountLoader<'info, BittensorState>,

    #[account(
        init,
        payer = owner,
        space = SubnetState::LEN,
        seeds = [b"subnet_state".as_ref()],
        bump,
    )]
    pub subnet_state: Account<'info, SubnetState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
