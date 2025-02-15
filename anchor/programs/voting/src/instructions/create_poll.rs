use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::*;

pub fn handle_create_poll(
    ctx: Context<CreatePollContext>,
    description: String,
    start: u64,
    end: u64,
) -> Result<()> {
    if start >= end {
        return Err(ErrorCode::InvalidDates.into());
    }

    let counter = &mut ctx.accounts.counter;
    counter.count += 1;

    let poll = &mut ctx.accounts.poll;
    poll.id = counter.count;
    poll.description = description;
    poll.start = start;
    poll.end = end;
    poll.candidates = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct CreatePollContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Poll::INIT_SPACE,
        seeds = [(counter.count + 1).to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        mut,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}
