use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Registerations {
    pub count: u64,
}