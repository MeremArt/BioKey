use anchor_lang::prelude::*;
use crate::state::UserAccountState;


#[derive(Accounts)]

pub struct FetchUserFingerprint<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"user", user.key().as_ref()],
        bump
    )]

    pub user_account: Account<'info, UserAccountState>,
}



pub fn handler(ctx: Context<FetchUserFingerprint>) -> Result<[u8; 32]> {
    let user_account = &ctx.accounts.user_account;

    // Return the hashed fingerprint
    Ok(user_account.hashed_fingerprint)
}