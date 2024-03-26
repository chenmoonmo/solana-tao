use crate::states::*;
use anchor_lang::prelude::*;

pub fn initialize_bittensor_validator(ctx: Context<InitializeBittensorValidator>) -> Result<()> {

    let bittensor_state = &mut ctx.accounts.bittensor_state;

    // TODO: 
    // 1.设置注册为主网验证人的标准
    // 2.检查子网验证人的工作量是否达到标准
    
    bittensor_state.create_validator(ctx.accounts.owner.key(), 0, 0, 0);

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeBittensorValidator<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: Box<Account<'info, BittensorState>>,

    #[account(
        mut,
        realloc = subnet_state.space(),
        realloc::zero = false,
        realloc::payer = owner,
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        mut,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
