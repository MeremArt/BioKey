use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccountState{
 /// Hashed fingerprint of the user (32 bytes, e.g., SHA-256)
pub hashed_fingerprint: [u8;32],
// The public key of the user owning this account 
pub public_key: Pubkey,
 /// Timestamp when the account was created
 
 pub created_at: i64,

 //bump seed for the pda 

 pub bump: u8

}
