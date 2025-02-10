use anchor_lang::prelude::*;

declare_id!("FQSjkWA6Sb6PcEJm693XsX9GETY7zhArPcPkkuLwdGws");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod voting {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
    counter.count = 0;

    let registerations = &mut ctx.accounts.registerations;
    registerations.count = 0;

    Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + 8,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + 8,
        seeds = [b"registerations"],
        bump
    )]
    pub registerations: Account<'info, Registerations>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}

#[account]
pub struct Registerations {
    pub count: u64,
}