use anchor_lang::prelude::*;
// 考虑 作为主网验证者或子网验证者

#[account]
pub struct MinerState {
    pub id: u64,
    pub owner: Pubkey,
}

impl MinerState {
    pub const LEN: usize = 8 + 32;
}
