use anchor_lang::prelude::*;

use crate::states::*;

pub fn validator_credit_for_subnet(
    ctx: Context<ValidatorCreditForSubnet>,
    weight: u8,
) -> Result<()> {
    let bittensor_state = &mut ctx.accounts.bittensor_state;
    let subnet_state = &ctx.accounts.subnet_state;

    subnet_state.set_weight(ctx.accounts.validator_state.id, weight);

    Ok(())
}

#[derive(Accounts)]
pub struct ValidatorCreditForSubnet<'info> {
    #[account(
        seeds = [b"bittensor"],
        bump
    )]
    pub bittensor_state: Box<Account<'info, BittensorState>>,

    // 子网state
    #[account(mut)]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    #[account(mut)]
    pub owner: Signer<'info>,
}
