use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::errors::ErrorCode;
use crate::states::*;
use crate::utils::*;

pub fn handle_execute_transaction(ctx: Context<ExecuteTransactionContext>) -> Result<()> {
    let tx = &mut ctx.accounts.transaction;
    let multisig = &mut ctx.accounts.multisig;

    require!(multisig.signers.contains(&ctx.accounts.authority.key()), ErrorCode::Unauthorized);
    require!(!tx.executed, ErrorCode::AlreadyExecuted);
    require!(tx.nonce == multisig.nonce, ErrorCode::InvalidNonce);
    require!(tx.approvals.len() as u8 >= multisig.threshold, ErrorCode::NotEnoughApprovals);

    multisig.nonce += 1;
    match tx.transaction_type {
        TransactionType::Transfer => {
            // Transfer SOL from escrow to target
            let cpi_accounts = Transfer {
                from: ctx.accounts.escrow.to_account_info(),
                to: ctx.accounts.target.to_account_info(),
            };
            let cpi_program = ctx.accounts.system_program.to_account_info();
            let seeds: &[&[u8]] = &[b"escrow", &[ctx.accounts.escrow.bump]]; // escrow PDA’s seeds
            let signer_seeds = &[seeds]; // wraps it in an outer slice, making it &[&[&[u8]]]—a list with one signer’s seed set
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            transfer(cpi_ctx, tx.amount)?;
        }
        TransactionType::ThresholdChange(new_threshold) => {
            require!(new_threshold as usize <= multisig.signers.len(), ErrorCode::InvalidThreshold);
            multisig.threshold = new_threshold;
        }
    }

    tx.executed = true;
    emit!(TransactionEvent {
        tx_key: tx.key(),
        action: "executed".to_string(),
    });
    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteTransactionContext<'info> {
    #[account(signer)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut, seeds = [b"escrow"], bump = escrow.bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub target: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}