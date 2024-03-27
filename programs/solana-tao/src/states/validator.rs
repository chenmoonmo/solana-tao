use anchor_lang::prelude::*;
// 考虑 作为主网验证者或子网验证者

#[account]
pub struct ValidatorState {
    pub id: u8,
    pub owner: Pubkey,
    // 质押数量
    pub stake: u64,
    // 工作量
    pub bonds: u64,
    // 保护期
    pub lockup: u64,
    // 打分记录 [[矿工ID，得分], [矿工ID，得分]...]
    pub weights: Vec<[u8; 2]>,
}

impl ValidatorState {
    pub const LEN: usize = 8 + 1 + 32 + 8 + 8 + 8 + 4 + 2;
}
