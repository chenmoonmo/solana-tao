use anchor_lang::prelude::*;

pub const EPOCH_REWARD: u64 = 100_000_000_000;

#[account]
pub struct BittensorState {
    pub owner: Pubkey,
    pub subnet_num: u8,
    pub total_stake: u64,
    pub subnets: Vec<SubnetInfo>,
    pub validators: Vec<BittensorValidatorInfo>,
}

impl BittensorState {
    pub const LEN: usize =
        32 + 1 + 8 + 4 + SubnetInfo::LEN * 32 + 4 + BittensorValidatorInfo::LEN * 32;
    pub const SEED: &'static [u8] = b"bittensor";

    pub fn create_subnet(&mut self, owner: Pubkey) -> () {
        self.subnets.push(SubnetInfo {
            id: self.subnets.len() as u8,
            owner,
            create_fee: 0,
            epoch: 0,
            last_reward_block: 0,
            distribute_reward: 0,
            validata_amount: 0,
            miner_amount: 0,
            stake: 0,
            weights: vec![],
        });
        ()
    }

    pub fn create_validator(&mut self, owner: Pubkey, stake: u64, bonds: u64, lockup: u64) -> () {
        self.validators.push(BittensorValidatorInfo {
            id: self.validators.len() as u8,
            owner,
            stake,
            bonds,
            lockup,
        });
        ()
    }
    // TODO: 分配奖励给子网
    pub fn distribute_reward(&mut self) -> () {
        // TODO: 判断周期
        let total_stake = self.total_stake;
        for subnet in self.subnets.iter_mut() {
            let mut subnet_total_weight = 0;
            // 权重
            let weights = subnet.weights;

            for weight in weights.iter() {
                let validator_id = weight[0][0] as usize;
                let validator = self.validators.get(validator_id).unwrap();
                subnet_total_weight += validator.stake * weight;
            }

            // 分配奖励
            let reward = subnet_total_weight
                .checked_mul(EPOCH_REWARD)
                .unwrap()?
                .checked_div(total_stake)
                .unwrap();

            subnet.distribute_reward = reward;
        }
        ()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct SubnetInfo {
    pub id: u8,
    pub owner: Pubkey,
    pub create_fee: u64,
    pub epoch: u64,
    pub last_reward_block: u64,
    pub distribute_reward: u64,
    pub validata_amount: u64,
    pub miner_amount: u64,
    pub stake: u64,
    pub weights: Vec<Vec<[u8; 2]>>,
}

impl SubnetInfo {
    pub const LEN: usize = 8 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub struct BittensorValidatorInfo {
    pub id: u8,
    pub owner: Pubkey,
    // 质押数量
    pub stake: u64,
    // 工作量
    pub bonds: u64,
    // 保护期
    pub lockup: u64,
}

impl BittensorValidatorInfo {
    pub const LEN: usize = 1 + 32 + 8 + 8 + 8;
}
