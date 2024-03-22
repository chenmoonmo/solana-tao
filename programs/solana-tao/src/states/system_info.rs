use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct BittensorState {
    pub subnet_num: u8,
    pub subnets: [Option<SubnetInfo>; 32],
}

impl BittensorState {
    pub const LEN: usize = 8 + SubnetInfo::LEN * 32;
    pub const SEED: &'static [u8] = b"system";
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
    pub const LEN: usize = 1 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8;
}
