use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::errors::ErrorCode;
use crate::states::*;
use crate::utils::*;

    pub fn handle_execute<'a, 'info>(ctx: Context<'a, 'a, 'a, 'info, ExecuteContext<'info>>, nonce: u64) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;
        let multisig = &mut ctx.accounts.multisig;
        let authority_key = ctx.accounts.authority.key();

    require!(multisig.approvals.contains(&ctx.accounts.authority.key()), ErrorCode::Unauthorized);
    require!(!tx.executed, ErrorCode::AlreadyExecuted);
    require!(tx.nonce == multisig.nonce, ErrorCode::InvalidNonce); // prevents replaying old transactions
    require!(tx.approvals.len() as u8 >= multisig.threshold, ErrorCode::NotEnoughApprovals);
    
    tx.executed = true; // Marked as executed to prevent reentrancy or double execution

    match tx.transaction_type {
        TransactionType::Transfer { target, amount } => {
            // Transfer SOL from escrow to target
            require!(ctx.remaining_accounts.len() >= 1, ErrorCode::MissingAccounts); // target
            let target_account = &ctx.remaining_accounts[0];
            require!(target_account.key() == target, ErrorCode::InvalidTarget);

            let cpi_accounts = Transfer {
                from: ctx.accounts.escrow.to_account_info(),
                to: target_account.to_account_info(), // target from TransactionType
            };
            let cpi_program = ctx.accounts.system_program.to_account_info();
            let signer_seeds: &[&[&[u8]]] = &[&[b"escrow", &[ctx.accounts.escrow.bump]]]; // escrow PDA’s seeds
           
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            transfer(cpi_ctx, amount)?;
        }
        TransactionType::ThresholdChange(new_threshold) => {
            require!(new_threshold as usize <= multisig.approvals.len(), ErrorCode::InvalidThreshold);
            multisig.threshold = new_threshold;
        }
    }

    multisig.nonce += 1;  // Increment nonce after successful execution to ensure uniqueness for future transactions

    emit!(TransactionEvent {
        tx_key: tx.key(),
        action: match tx.transaction_type {
            TransactionType::Transfer { .. } => "transfer_executed".to_string(),
            TransactionType::ThresholdChange(_) => "threshold_change_executed".to_string(),
        },
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct ExecuteContext<'info> {
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"tx", nonce.to_le_bytes().as_ref()], bump)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut, seeds = [b"multisig"], bump)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut, seeds = [b"escrow"], bump)]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}
