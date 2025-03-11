use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::errors::ErrorCode;
use crate::states::*;

pub fn handle_initialize(ctx: Context<InitializeContext>, signers: Vec<Pubkey>, threshold: u8, initial_balance: u64) -> Result<()> {
    let multisig = &mut ctx.accounts.multisig;
    require!(signers.len() > 0 && threshold as usize <= signers.len(), ErrorCode::InvalidThreshold);
    
    multisig.admin = *ctx.accounts.admin.key;
    multisig.signers = signers;
    multisig.threshold = threshold;
    multisig.nonce = 0;

    // Transfer SOL from admin to escrow
    let cpi_accounts = Transfer {
        from: ctx.accounts.admin.to_account_info(),
        to: ctx.accounts.escrow.to_account_info(),
    };
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer(cpi_ctx, initial_balance)?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Multisig::INIT_SPACE,
    )]
    pub multisig: Account<'info, Multisig>,
    #[account(
        init,
        payer = admin,
        space = 8 + Escrow::INIT_SPACE, 
        seeds = [b"escrow"],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}