use anchor_lang::prelude::*;

declare_id!("AeR9zRYNF7MGA35NbYUt1suVqNC3Uj4DaNhmGzEwNHAb");

pub mod state;
pub use state::*;
pub mod instructions;

use instructions::*;

#[program]
pub mod biokey {
    use super::*;

    pub fn create_or_update_auth_state(
        ctx: Context<CreateOrUpdateAuthState>,
        is_authenticated: bool,
    ) -> Result<()> {
        instructions::create_or_update_auth_state::handler(ctx, is_authenticated)
    }
}