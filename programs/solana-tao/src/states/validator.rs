use anchor_lang::prelude::*;


// 考虑 作为主网验证者或子网验证者 

#[account]
pub struct ValidatorState {
    pub id: u64,
    pub owner: Pubkey,
    // 质押数量
    pub stake: u64,
    // 工作量
    pub bonds: u64,
    // 保护期
    pub lockup: u64,
}

impl ValidatorState {
    pub const LEN: usize = 8+ 8 + 32 + 8 + 8 + 8;
}