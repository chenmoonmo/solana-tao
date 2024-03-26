use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

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
        seeds = [b"bittensor"],
        bump
    )]
    pub bittensor_state: Box<Account<'info, BittensorState>>,

    // 系统奖励代币
    #[account(
        init,
        payer = owner,
        seeds = [b"tao", bittensor_state.key().as_ref()], 
        bump,
        mint::decimals = 9,
        mint::authority = bittensor_state
    )]
    pub tao_mint: Box<Account<'info, Mint>>,
    // 质押代币存储账户
    #[account(
        init,
        payer = owner,
        seeds=[b"tao_stake", bittensor_state.key().as_ref()],
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
