use anchor_lang::prelude::*;

use crate::AuthState::AuthState;

#[derive(Accounts)]

pub struct CheckAuthentication<'info>{
      ///  user performing the authentication check
      #[account(mut)]
      pub user: Signer<'info>,

       /// The PDA storing the user's authentication state
       #[account(
        seeds = [b"auth_state", user.key().as_ref()],
        bump = auth_state.bump
       )]
       pub auth_state: Account<'info, AuthState>
}

pub fn handleri(ctx: Context<CheckAuthentication>) -> Result<bool> {
    let auth_state = &ctx.accounts.auth_state;

    if auth_state.is_authenticated {
        msg!("User is authenticated. Last auth time: {}", auth_state.last_auth_time);
        Ok(true)
    } else {
        msg!("User is not authenticated.");
        Ok(false)
    }
}
