use anchor_lang::prelude::*;

use crate::states::*;

#[derive(Accounts)]
pub struct ValidatorCredit<'info> {
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
    pub miner_state: Account<'info, MinerState>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn validata_credit(ctx: Context<ValidatorCredit>, score: u8) -> Result<()> {
    let validator_state = &mut ctx.accounts.validator_state;
    let miner_state = &mut ctx.accounts.miner_state;

    miner_state.set_score(validator_state.id, score);

    Ok(())
}
