pub mod states;
pub mod instructions;

use anchor_lang::prelude::*;

use crate::instructions::*;

declare_id!("G4eLua3cJfGkAkuYramavo2QjEwU37SUk6RJeW8U8bSW");
#[program]
pub mod solana_tao {
    use super::*;

    pub fn initialize_system(ctx:Context<InitializeBittensor>) -> Result<()> {
        instructions::initialize_bittensor(ctx)
    }
}
