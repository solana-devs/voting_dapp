use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Candidate {
    pub cid: u64, // candidate id
    pub poll_id: u64,
    #[max_len(40)]
    pub name: String,
    pub votes: u64,
    pub has_registered: bool,
}