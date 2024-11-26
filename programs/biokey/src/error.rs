use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided public key does not match the expected owner.")]
    InvalidPublicKey,
}
