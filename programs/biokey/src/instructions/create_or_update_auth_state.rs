use anchor_lang::prelude::*;
use crate::AuthState::AuthState;

#[derive(Accounts)]
#[instruction(is_authenticated: bool)]
pub struct CreateOrUpdateAuthState<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + AuthState::INIT_SPACE,
        seeds = [b"auth_state", user.key().as_ref()],
        bump
    )]
    pub auth_state: Account<'info, AuthState>,
    pub system_program: Program<'info, System>,
}

pub fn handlerii(
    ctx: Context<CreateOrUpdateAuthState>,
    is_authenticated: bool,
) -> Result<()> {
    let auth_state = &mut ctx.accounts.auth_state;
    
    auth_state.is_authenticated = is_authenticated;
    auth_state.last_auth_time = Clock::get()?.unix_timestamp as u64;
    auth_state.bump = ctx.bumps.auth_state;  // Fixed: using auth_state instead of user_account
    
    Ok(())
}