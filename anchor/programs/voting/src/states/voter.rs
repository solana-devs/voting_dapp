use anchor_lang::prelude::*;

#[account]
pub struct Voter {
    pub cid: u64, // candidate id
    pub poll_id: u64,
    pub has_voted: bool,
}