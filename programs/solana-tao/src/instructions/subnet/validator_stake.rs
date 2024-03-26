use anchor_lang::prelude::*;

use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::states::*;

pub fn validator_stake(ctx: Context<ValidatorStake>, amount: u64) -> Result<()> {
    
    let validator_stake = &mut *ctx.accounts.tao_stake;
    validator_stake.amount += amount;

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
pub struct ValidatorStake<'info> {
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

    #[account(
        mut,
        payer = owner,
        seeds = [b"validator_state",subnet_state.key().as_ref(),owner.key().as_ref()],
        bump
    )]
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
