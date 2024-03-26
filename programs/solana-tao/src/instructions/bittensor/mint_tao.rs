use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Mint, MintTo, Token, TokenAccount},
};

use crate::states::*;

pub fn mint_tao(ctx: Context<MintTao>) -> Result<()> {
    let bump = ctx.bumps.bittensor_state;
    let pda_sign: &[&[u8]; 2] = &[b"bittensor", &[bump]];

    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.tao_mint.to_account_info(),
                to: ctx.accounts.user_tao_ata.to_account_info(),
                authority: ctx.accounts.bittensor_state.to_account_info(),
            },
        )
        .with_signer(&[pda_sign]),
        10 * 10 * 10_000_000_000,
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct MintTao<'info> {
    #[account(
        mut,
        seeds = [b"bittensor"],
        bump
    )]
    pub bittensor_state: Box<Account<'info, BittensorState>>,

    #[account(
        mut,
        constraint = user_tao_ata.mint == tao_mint.key(),
        seeds = [b"tao", bittensor_state.key().as_ref()], 
        bump
    )]
    pub tao_mint: Account<'info, Mint>,


    #[account(
        mut,
        constraint = user_tao_ata.mint == tao_mint.key(),
        has_one = owner,
    )]
    pub user_tao_ata: Box<Account<'info, TokenAccount>>,

    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
