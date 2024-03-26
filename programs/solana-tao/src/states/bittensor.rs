use anchor_lang::prelude::*;

#[account]
pub struct BittensorState {
    pub owner: Pubkey,
    pub subnet_num: u8,
    pub subnets: Vec<SubnetInfo>,
}

impl BittensorState {
    pub const LEN: usize = 32 + 1 + 4 + SubnetInfo::LEN * 32;
    pub const SEED: &'static [u8] = b"system";

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
}

impl SubnetInfo {
    pub const LEN: usize = 8 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8;
}
