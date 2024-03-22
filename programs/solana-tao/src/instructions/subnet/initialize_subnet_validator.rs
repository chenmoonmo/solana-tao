use crate::states::*;
use anchor_lang::prelude::*;

pub fn initialize_subnet_validator(ctx: Context<InitializeSubnetValidator>) -> Result<()> {
    let subnet_state = &mut ctx.accounts.subnet_state;
    let validator_state = &mut ctx.accounts.validator_state;
    subnet_state.owner = ctx.accounts.owner.key();
    validator_state.owner = ctx.accounts.owner.key();
    subnet_state.create_validator(ctx.accounts.owner.key(), 0, 0, 0);

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSubnetValidator<'info> {
    #[account(
        mut,
        realloc = subnet_state.space(),
        realloc::zero = false,
        realloc::payer = owner,
        seeds = [b"subnet_state".as_ref()],
        bump,
    )]
    pub subnet_state: Box<Account<'info, SubnetState>>,

    #[account(
        init,
        space = ValidatorState::LEN,
        payer = owner,
        seeds = [b"validator_state".as_ref(),owner.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
