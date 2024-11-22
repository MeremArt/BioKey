use anchor_lang::prelude::*;

use crate::state::AuthState;

#[derive(Accounts)]

pub struct CreateOrUpdateAuthState<'info>{

    #[account(mut)]

    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer= user,
        space = AuthState::INIT_SPACE,
        seeds = [b"auth_state",user.key().as_ref()],
        bump
    )]
    pub auth_state: Account<'info, AuthState>,
    pub system_program: Program<'info, System>
}

impl <'info> CreateOrUpdateAuthState<'info>{

    pub fn create_or_update_auth_state(
        ctx: Context<CreateOrUpdateAuthState>,
        is_authenticated: bool,
    ) -> Result<()>{

        let auth_state = &mut ctx.accounts.auth_state;
          // Update the AuthState account with the provided values
          auth_state.is_authenticated = is_authenticated;
          auth_state.last_auth_time = Clock::get()?.unix_timestamp as u64; 
Ok(())
    }
}