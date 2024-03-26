use crate::states::*;
use anchor_lang::prelude::*;

pub fn initialize_subnet_miner(ctx: Context<InitializeSubnetMiner>) -> Result<()> {
    // TODO:
    // 设置注册费用
    // 注册矿工时 燃烧代币
    // 矿工保护期初始化
    let subnet_state = &mut ctx.accounts.subnet_state;
    let miner_state = &mut ctx.accounts.miner_state;
    
    miner_state.owner = ctx.accounts.owner.key();
    subnet_state.create_miner(ctx.accounts.owner.key());

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnetMiner<'info> {
    #[account(
        mut,
        realloc = subnet_state.space(),
        realloc::zero = false,
        realloc::payer = owner,
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        init,
        space = 8 + MinerState::LEN,
        payer = owner,
        seeds = [b"miner_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
    pub miner_state: Account<'info, MinerState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
