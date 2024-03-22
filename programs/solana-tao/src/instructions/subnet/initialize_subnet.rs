use crate::states::*;
use anchor_lang::prelude::*;

pub fn initialize_subnet(ctx: Context<InitializeSubnet>,) -> Result<()> {
    let epoch = Clock::get().unwrap().epoch;
    msg!("Current Epoch: {}", epoch);
    let subnet_state = &mut *ctx.accounts.subnet_state;
    subnet_state.owner = ctx.accounts.owner.key();

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnet<'info> {
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
