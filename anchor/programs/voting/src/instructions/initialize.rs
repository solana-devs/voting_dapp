use crate::states::*;
use anchor_lang::prelude::*;

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.count = 0;

    let registerations = &mut ctx.accounts.registerations;
    registerations.count = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Counter::INIT_SPACE,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,

    #[account(
        init,
        payer = user,
        space = 8 + Registerations::INIT_SPACE,
        seeds = [b"registerations"],
        bump
    )]
    pub registerations: Account<'info, Registerations>,

    pub system_program: Program<'info, System>,
}