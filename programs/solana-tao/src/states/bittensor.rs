use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[repr(packed)]
pub struct BittensorState {
    pub subnet_num: u8,
    pub subnets: [Option<SubnetInfo>; 32],
}

impl BittensorState {
    pub const LEN: usize = 8 + 1 + SubnetInfo::LEN * 32;
    pub const SEED: &'static [u8] = b"system";

    pub fn create_subnet(&mut self, owner: Pubkey) -> () {
        let subnets = self.subnets;

        let lowest_index = match subnets.iter().position(|x| x.is_none()) {
            Some(index) => index,
            None => {
                // TODO: handle error
                return ();
            }
        };

        msg!("create_subnet: lowest_index: {}", lowest_index);

        self.subnets[lowest_index] = Some(SubnetInfo {
            id: lowest_index as u8,
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
