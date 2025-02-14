use anchor_lang::prelude::*;
use crate::errors::ErrorCode::*;
use crate::states::*;

pub fn handle_vote(ctx: Context<Vote>, poll_id: u64, cid: u64) -> Result<()> {
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64, cid: u64)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>, // Poll to be voted in

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), cid.to_le_bytes().as_ref()],
        bump
    )]
    pub candidate: Account<'info, Candidate>, // Candidate to receive the vote

    #[account(
        init, // Create the voter account if it doesn't exist
        payer = user,
        space = 8 + 25, // Account size
        seeds = [b"voter", poll_id.to_le_bytes().as_ref(), user.key().as_ref()],
        bump
    )]
    pub voter: Account<'info, Voter>, // Unique per poll and user

    #[account(mut)]
    pub user: Signer<'info>, // Voter's signer account

    pub system_program: Program<'info, System>,
}
