use anchor_lang::prelude::*;

declare_id!("HXEPyKzidcNAbnNWY1DMu433eBcN4nP1m381QdVKQe2S");

pub mod state;
mod error;
pub mod instructions;

use instructions::*;
use state::*;

#[program]
pub mod biokey {
    use super::*;
  

    pub fn create_or_update_auth_state(
        ctx: Context<CreateOrUpdateAuthState>,
        is_authenticated: bool,
    ) -> Result<()> {
        instructions::create_or_update_auth_state::handlerii(ctx, is_authenticated)
    }

    pub fn create_user_account(
        ctx: Context<CreateUserAccount>,
        hashed_fingerprint: [u8; 32],
    ) -> Result<()> {
        instructions::create_user_account::handleriii(ctx, hashed_fingerprint)

    }

    pub fn validate_fingerprint(
        ctx: Context<ValidateFingerprint>,
        provided_hashed_fingerprint: [u8; 32],
    ) -> Result<bool> {
        instructions::validate_fingerprint::handleriv(ctx, provided_hashed_fingerprint)
    }
    pub fn check_authentication(
        ctx: Context<CheckAuthentication>,

    )-> Result<bool>{
        instructions::check_authentication::handleri(ctx)
    }

    pub fn fetch_user_fingerprint(ctx: Context<FetchUserFingerprint>) -> Result<[u8; 32]> {
        instructions::fetch_user_fingerprint::handler(ctx)
    }
    
}