use anchor_lang::prelude::*;

pub const PROPORTION: f64 = 0.5;
pub const MINER_REWARD_PROPORTION: f64 = 0.5;
pub const VALIDATOR_REWARD_PROPORTION: f64 = 0.5;

#[account]
pub struct SubnetState {
    // 验证人质押总量
    pub total_stake: u64,
    // 验证人总数
    pub total_validators: u64,
    // 矿工总数
    pub total_miners: u64,
    // 子网属性
    // 子网的质押数量
    pub stake: u64,
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
    // 子网的得分 [[验证者ID, 得分], [验证者ID, 得分]...]
    pub weights: Vec<Vec<[u8; 2]>>,
}

impl SubnetState {
    pub const LEN: usize =
        8 + 8 + 8 + 8 + 8 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 24 + 24 + 4 + ((4 + 2) * 32) * 10;

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
        // TODO: 限制验证人数量
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
            reward: 0,
        });
    }

    pub fn create_miner(&mut self, owner: Pubkey) {
        self.miners.push(MinerInfo {
            id: self.miners.len() as u8,
            owner,
        });
    }

    pub fn set_weight(&mut self, validator_id: u8, weight: u8) -> Result<()> {
        // 取出矩阵最后一行
        let last_row = self.weights.last_mut().unwrap();
        // 遍历矩阵最后一行 如果验证者 ID 存在则失败
        for i in 0..last_row.len() {
            if last_row[i][0] == validator_id {
                // TODO: 抛出错误
                // return Err(ErrorCode::ValidatorIdExists.into());
            }
        }

        last_row.push([validator_id, weight]);
        Ok(())
    }
    // TODO: 分配奖励给验证人和矿工
    pub fn distribute_reward(&mut self) {
        // 给验证人分配奖励
        let validators = self.validators;
        let reward_base: f64 = 0;
        let validator_reward = validators.iter().map(|validator| {
            let bonds = validator.bonds;
            let stake = validator.stake;
            // 瓜分比
            let validator_reward = (bonds as f64) * PROPORTION + (stake as f64) * (1 - PROPORTION);
            reward_base += validator_reward;
            (validator, validator_reward)
        });

        for (validator, validator_reward) in validator_reward {
            // 瓜分比例
            let proportion = validator_reward / reward_base;
            validator.bonds = validator_reward;
            // 瓜分奖励
            let reward = (self.distribute_reward * VALIDATOR_REWARD_PROPORTION * proportion) as u64;
            validator.reward += reward;
        }

        // 给矿工分配奖励
        let miners = self.miners;
        

    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub struct ValidatorInfo {
    pub id: u8,
    pub owner: Pubkey,
    // 质押数量
    pub stake: u64,
    // 工作量
    pub bonds: u64,
    // 保护期
    pub lockup: u64,
    // 待提取奖励
    pub reward: u64,
}

impl ValidatorInfo {
    pub const LEN: usize = 1 + 32 + 8 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub struct MinerInfo {
    pub id: u8,
    pub owner: Pubkey,
    // // 名称
    // pub name: String,
    // // 简介
    // pub desc: String,
    // // rpc
    // pub rpc: String,
}

impl MinerInfo {
    pub const LEN: usize = 1 + 32;
}
