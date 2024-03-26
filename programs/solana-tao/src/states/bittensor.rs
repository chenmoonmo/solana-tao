use anchor_lang::prelude::*;

#[account]
pub struct BittensorState {
    pub owner: Pubkey,
    pub subnet_num: u8,
    pub subnets: Vec<SubnetInfo>,
    pub validators: Vec<BittensorValidatorInfo>,
}

impl BittensorState {
    pub const LEN: usize = 32 + 1 + 4 + SubnetInfo::LEN * 32 + 4 + BittensorValidatorInfo::LEN * 32;
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
        });
        ()
    }

    pub fn create_validator(&mut self, owner: Pubkey, stake: u64, bonds: u64, lockup: u64) -> () {
        self.validators.push(BittensorValidatorInfo {
            id: self.validators.len() as u64,
            owner,
            stake,
            bonds,
            lockup,
        });
        ()
    }
    // TODO: 分配奖励给子网
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
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
}

impl SubnetInfo {
    pub const LEN: usize = 8 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub struct BittensorValidatorInfo {
    pub id: u64,
    pub owner: Pubkey,
    // 质押数量
    pub stake: u64,
    // 工作量
    pub bonds: u64,
    // 保护期
    pub lockup: u64,
}

impl BittensorValidatorInfo {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 8;
}
