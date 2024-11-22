use anchor_lang::prelude::*;


#[derive(Debug, Default, PartialEq)]
#[account]  
#[derive(InitSpace)]
pub struct AuthState {
    pub is_authenticated: bool, // True if the user is authenticated
    pub last_auth_time: u64,    // Timestamp of last authentication
}

