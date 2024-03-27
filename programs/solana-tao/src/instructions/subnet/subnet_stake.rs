use anchor_lang::prelude::*;

use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::states::*;

pub fn subnet_stake(ctx: Context<SubnetStake>, amount: u64) -> Result<()> {
    // TODO: 修改 subnet 数据

    let subnet_state = &mut *ctx.accounts.subnet_state;
    subnet_state.stake += amount;

    let bittensor_state = &mut *ctx.accounts.bittensor_state;

    let subnets =  bittensor_state.subnets;
    // 遍历子网 找到对应的子网 修改 stake
    for subnet in subnets.iter() {
        if subnet.id == subnet_state.id {
            subnet.stake += amount;
        }
    }

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_tao_ata.to_account_info(),
                to: ctx.accounts.tao_stake.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}



#[derive(Accounts)]
pub struct SubnetStake<'info> {
    #[account(
        seeds = [b"bittensor"],
        bump
    )]
    pub bittensor_state: Box<Account<'info, BittensorState>>,

    // 子网state
    #[account(mut)]
    pub subnet_state: Box<Account<'info, SubnetState>>,
    // tao mint
    #[account(
            mut,
            seeds = [b"tao", bittensor_state.key().as_ref()], 
            bump,
        )]
    pub tao_mint: Box<Account<'info, Mint>>,
    // 子网 stake 账户
    #[account(
        mut,
        seeds=[b"tao_stake", subnet_state.key().as_ref()],
        bump,
    )]
    pub tao_stake: Box<Account<'info, TokenAccount>>,

    // tao ata 账户
    #[account(
        mut,
        constraint = user_tao_ata.mint == tao_mint.key(),
        has_one = owner,
    )]
    pub user_tao_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
