mod consts;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::MAX_PERMITTED_DATA_INCREASE;
use consts::USER_INFO_SEED;

declare_id!("5ge4JgXdpwr87y3nDWX2e4Te8neCDwAX9X6LG7Rptez7");

#[program]
pub mod solana_memory_issue {

    use super::*;

    pub fn init_user(ctx: Context<InitUser>, age: u8) -> Result<()> {
        *ctx.accounts.user_info = UserInfo {
            age,
            address: ctx.accounts.signer.key(),
            friends: Vec::new(),
        };

        Ok(())
    }

    pub fn add_friend(ctx: Context<AddFriend>, address: Pubkey, age: u8) -> Result<()> {
        ctx.accounts.user_info.friends.push(Friend { age, address });
        msg!("Friends: {}", ctx.accounts.user_info.friends.len());
        Ok(())
    }

    pub fn increase_space(ctx: Context<IncreaseSpace>, _size: u16) -> Result<()> {
        let account = ctx.accounts.user_info.to_account_info();

        msg!("New Size: {}", account.data_len());
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Friend {
    age: u8,         // 1
    address: Pubkey, // 32
}

#[account]
pub struct UserInfo {
    pub age: u8,         // 1
    pub address: Pubkey, // 32
    pub friends: Vec<Friend /* 33 */>,
}

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init,payer = signer,space = MAX_PERMITTED_DATA_INCREASE,seeds = [USER_INFO_SEED,signer.key.as_ref()],bump)]
    pub user_info: Account<'info, UserInfo>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddFriend<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut,seeds = [USER_INFO_SEED,signer.key.as_ref()],bump)]
    pub user_info: Account<'info, UserInfo>,
}

#[derive(Accounts)]
#[instruction(len: u16)]
pub struct IncreaseSpace<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut,realloc=len as usize,realloc::payer=signer,realloc::zero=true ,seeds = [USER_INFO_SEED,signer.key.as_ref()],bump)]
    pub user_info: Account<'info, UserInfo>,

    pub system_program: Program<'info, System>,
}
