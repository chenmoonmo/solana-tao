use anchor_lang::prelude::*;
// 考虑 作为主网验证者或子网验证者

#[account]
pub struct MinerState {
    pub id: u8,
    pub owner: Pubkey,
    pub stake: u64,
    pub rpc_url: String,
    // 矿工的得分 [[验证者ID, 得分], [验证者ID, 得分]...]
    pub weights: Vec<Vec<[u8; 2]>>,
}

impl MinerState {
    pub const LEN: usize = 1 + 32 + 8 + (4 + 128) + 4 + ((4 + 2) * 32) * 10;
    // 设置分数 提供验证者 ID 和 得分, 如果矩阵的最后一行中 验证者 ID 不存在则添加, 如果存在则失败
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
}
