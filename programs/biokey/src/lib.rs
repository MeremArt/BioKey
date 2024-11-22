use anchor_lang::prelude::*;

declare_id!("AeR9zRYNF7MGA35NbYUt1suVqNC3Uj4DaNhmGzEwNHAb");

mod state;
mod instructions;

pub use instruction::*;

#[program]
pub mod biokey {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }
}

