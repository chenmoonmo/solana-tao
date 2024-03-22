use anchor_lang::prelude::*;

#[account]
pub struct SubnetState {
    // 验证人质押总量
    pub total_stake: u64,
    // 验证人总数
    pub total_validators: u64,
    // 矿工总数
    pub total_miners: u64,
    // 子网属性
    // ID
    pub id: u8,
    // 权限拥有者
    pub owner: Pubkey,
    // 子网创建时抵押的代币数量
    pub create_fee: u64,
    // 子网的周期
    pub epoch: u64,
    // 上一次分红的区块高度
    pub last_reward_block: u64,
    // 子网待分配奖励
    pub distribute_reward: u64,
    // 子网中允许的最大验证人数量
    pub validata_amount: u64,
    // 子网中允许的最大矿工数量
    pub miner_amount: u64,
}

impl SubnetState {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8;
}
