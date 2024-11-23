use anchor_lang::prelude::*;

#[derive(Debug, Default, PartialEq)]
#[account]
#[derive(InitSpace)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub last_auth_time: u64,
    pub bump: u8,
}