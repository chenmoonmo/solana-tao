use anchor_lang::prelude::*;

// #[account(zero_copy(unsafe))]
// #[repr(packed)]
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
    // 验证人列表
    pub validators: Vec<ValidatorInfo>,
    // 矿工列表
    pub miners: Vec<MinerInfo>,
}

impl SubnetState {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 24 + 24;

    pub fn space(&self) -> usize {
        let validator_num = self.validators.len();
        let miner_num = self.miners.len();

        8 + 8
            + 8
            + 8
            + 1
            + 32
            + 8
            + 8
            + 8
            + 8
            + 8
            + 8
            + 24
            + 24
            + (validator_num + miner_num + 1) * ValidatorInfo::LEN
    }

    pub fn create_validator(&mut self, owner: Pubkey, stake: u64, bonds: u64, lockup: u64) {
        // let len = self.validators.len();

        // if len >= self.validata_amount as usize {
        //     return;
        // }

        self.validators.push(ValidatorInfo {
            id: self.validators.len() as u64,
            owner,
            stake,
            bonds,
            lockup,
        });
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub struct ValidatorInfo {
    pub id: u64,
    pub owner: Pubkey,
    // 质押数量
    pub stake: u64,
    // 工作量
    pub bonds: u64,
    // 保护期
    pub lockup: u64,
}

impl ValidatorInfo {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub struct MinerInfo {
    pub id: u64,
    pub owner: Pubkey,
    // // 名称
    // pub name: String,
    // // 简介
    // pub desc: String,
    // // rpc
    // pub rpc: String,
}
