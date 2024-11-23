use anchor_lang::prelude::*;
use crate::state::UserAccountState;

#[derive(Accounts)]
#[instruction(hashed_fingerprint: [u8; 32])]
pub struct CreateUserAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        init,
        payer = user,
        space = 8 + UserAccountState::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccountState>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateUserAccount>,
    hashed_fingerprint: [u8; 32],
) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    // Initialize fields in the UserAccountState
    user_account.hashed_fingerprint = hashed_fingerprint;
    user_account.public_key = ctx.accounts.user.key();
    user_account.created_at = Clock::get()?.unix_timestamp;
    user_account.bump = ctx.bumps.user_account;
    
    Ok(())
}
