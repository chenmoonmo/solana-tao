use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

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

    // TODO: 最好和 子网 ID 相关
    #[account(
        init,
        payer = owner,
        space = SubnetState::LEN,
        seeds = [b"subnet_state",owner.key().as_ref()],
        bump,
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    // 系统代币
    #[account(
        mut,
        seeds = [b"tao", bittensor_state.key().as_ref()], 
        bump,
    )]
    pub tao_mint: Box<Account<'info, Mint>>,

    // 子网存储stake 的账户
    #[account(
        init,
        payer = owner,
        seeds=[b"tao_stake", subnet_state.key().as_ref()],
        bump,
        token::mint = tao_mint,
        token::authority = bittensor_state
    )]
    pub tao_stake: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
