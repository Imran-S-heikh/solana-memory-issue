use anchor_lang::prelude::*;

declare_id!("7ADHtPbzx2msHbiKu1fEBwP8u2eePGRU6WCz4f55cVpg");

#[program]
pub mod solana_memory_issue {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
