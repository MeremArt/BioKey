use anchor_lang::prelude::*;
use crate::state::UserAccountState;

#[derive(Accounts)]
pub struct ValidateFingerprint<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
   /// The PDA account storing the user's hashed fingerprint
    #[account(
        seeds= [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]

    pub user_account: Account<'info, UserAccountState>

}

pub fn handleriv(
    ctx: Context<ValidateFingerprint>,
    provided_hashed_fingerprint: [u8; 32],
)-> Result<bool>{
// access the user account

let user_account = &ctx.accounts.user_account;
 // Compare the stored and provided hashed fingerprints
if user_account.hashed_fingerprint == provided_hashed_fingerprint{
msg!("Fingerprint Validation successful");
Ok(true)
}else{
    msg!("Fingerprint Validation failed");
    
    Ok(false)
}
}