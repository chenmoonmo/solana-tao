use anchor_lang::prelude::*;

declare_id!("G4eLua3cJfGkAkuYramavo2QjEwU37SUk6RJeW8U8bSW");

#[program]
pub mod solana_tao {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
