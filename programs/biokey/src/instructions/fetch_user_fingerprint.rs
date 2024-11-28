use anchor_lang::prelude::*;
use crate::{error::ErrorCode,state::UserAccountState};

#[derive(Accounts)]
pub struct FetchUserFingerprint<'info> {
    // The user who is fetching the fingerprint
    #[account(mut)]
    pub user: Signer<'info>,

    // User account PDA with seeds for validation
    #[account(
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccountState>,
}

pub fn handler(ctx: Context<FetchUserFingerprint>) -> Result<[u8; 32]> {
    let user_account = &ctx.accounts.user_account;
    if ctx.accounts.user.key() != user_account.public_key {
        return Err(ErrorCode::InvalidPublicKey.into());
    }

      // Save hashed fingerprint to a client-readable field
      let hashed_fingerprint = user_account.hashed_fingerprint;
      msg!("Hashed Fingerprint: {:?}", hashed_fingerprint);
    // Return the hashed fingerprint stored in the user account
    Ok(user_account.hashed_fingerprint)
}
