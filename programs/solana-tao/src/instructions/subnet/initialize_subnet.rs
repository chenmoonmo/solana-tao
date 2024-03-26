use crate::states::*;
use anchor_lang::prelude::*;

pub fn initialize_subnet(ctx: Context<InitializeSubnet>) -> Result<()> {
    // TODO:
    // 设置注册费用
    // 将注册费用转到 stake 账户
    // 子网周期初始化
    let subnet_state = &mut ctx.accounts.subnet_state;
    subnet_state.owner = ctx.accounts.owner.key();

    ctx.accounts.bittensor_state.subnet_num += 1;

    ctx.accounts
        .bittensor_state
        .create_subnet(ctx.accounts.owner.key());

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnet<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump,
    )]
    pub bittensor_state: Box<Account<'info, BittensorState>>,

    #[account(
        init,
        payer = owner,
        space = SubnetState::LEN,
        seeds = [b"subnet_state",owner.key().as_ref()],
        bump,
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
