use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("FQSjkWA6Sb6PcEJm693XsX9GETY7zhArPcPkkuLwdGws");

#[program]
pub mod voting {

    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        instructions::initialize::handle_initialize(ctx)
    }

    pub fn create_poll(
        ctx: Context<CreatePollContext>,
        description: String,
        start: u64,
        end: u64,
    ) -> Result<()> {
        instructions::handle_create_poll(ctx, description, start, end)
    }

    pub fn register_candidate(
        ctx: Context<RegisterCandidateContext>,
        poll_id: u64,
        name: String,
    ) -> Result<()> {
        instructions::handle_register_candidate(ctx, poll_id, name)
    }

    pub fn vote(ctx: Context<VoteContext>, poll_id: u64, cid: u64) -> Result<()> {
        instructions::handle_vote(ctx, poll_id, cid)
    }

}
