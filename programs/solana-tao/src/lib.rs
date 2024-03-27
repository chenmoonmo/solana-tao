pub mod instructions;
pub mod states;

use anchor_lang::prelude::*;

use crate::instructions::*;

declare_id!("G4eLua3cJfGkAkuYramavo2QjEwU37SUk6RJeW8U8bSW");
#[program]
pub mod solana_tao {
    use super::*;
    // 初始化主网
    pub fn initialize_bittensor(ctx: Context<InitializeBittensor>) -> Result<()> {
        instructions::initialize_bittensor(ctx)
    }

    // 注册子网
    pub fn initialize_subnet(ctx: Context<InitializeSubnet>) -> Result<()> {
        instructions::initialize_subnet(ctx)
    }

    // 注册子网验证人
    pub fn initialize_subnet_validator(ctx: Context<InitializeSubnetValidator>) -> Result<()> {
        instructions::initialize_subnet_validator(ctx)
    }

    // 注册子网矿工
    pub fn initialize_subnet_miner(ctx: Context<InitializeSubnetMiner>) -> Result<()> {
        instructions::initialize_subnet_miner(ctx)
    }

    // 从子网验证人注册主网验证人
    pub fn initialize_bittensor_validator(
        ctx: Context<InitializeBittensorValidator>,
    ) -> Result<()> {
        instructions::initialize_bittensor_validator(ctx)
    }

    // 临时使用 mint tao
    pub fn mint_tao(ctx: Context<MintTao>) -> Result<()> {
        instructions::mint_tao(ctx)
    }

    // 验证人添加质押 done
    pub fn validator_stake(ctx: Context<ValidatorStake>, amount: u64) -> Result<()> {
        instructions::validator_stake(ctx, amount)
    }
    // 验证人提取质押

    // 矿工添加质押 done
    pub fn miner_stake(ctx: Context<MinerStake>, amount: u64) -> Result<()> {
        instructions::miner_stake(ctx, amount)
    }
    // 矿工提取质押

    // 子网验证人给矿工打分 done
    pub fn validator_credit(ctx: Context<ValidatorCredit>, weight: u8) -> Result<()> {
        instructions::validator_credit(ctx, weight)
    }
    // 主网验证人给子网打分
    pub fn validator_credit_for_subnet(
        ctx: Context<ValidatorCreditForSubnet>,
        weight: u8,
    ) -> Result<()> {
        instructions::validator_credit_for_subnet(ctx, weight)
    }

    //TODO:
    // 1.每次操作后检测主网是否到达周期，给子网分配奖励
    // 2.每次操作后检测子网是否到达周期，给验证人和矿工分配奖励
}
