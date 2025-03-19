use anchor_lang::prelude::*;
// use anchor_lang::system_program::{transfer, Transfer};
use crate::errors::ErrorCode;
use crate::states::*;
use crate::utils::*;

pub fn handle_execute_threshold_change(ctx: Context<ExecuteThresholdChangeContext>) -> Result<()> {
    let tx = &mut ctx.accounts.transaction;
    let multisig = &mut ctx.accounts.multisig;

    require!(multisig.approvals.contains(&ctx.accounts.authority.key()), ErrorCode::Unauthorized);
    require!(!tx.executed, ErrorCode::AlreadyExecuted);
    require!(tx.nonce == multisig.nonce, ErrorCode::InvalidNonce);
    require!(tx.approvals.len() as u8 >= multisig.threshold, ErrorCode::NotEnoughApprovals);
   
    require!(tx.new_threshold as usize <= multisig.approvals.len(), ErrorCode::InvalidThreshold);
    multisig.nonce += 1;
    multisig.threshold = tx.new_threshold;

    tx.executed = true;
    emit!(TransactionEvent {
        tx_key: tx.key(),
        action: "threshold_change_executed".to_string(),
    });
    Ok(())
}

#[derive(Accounts)]
pub struct ExecuteThresholdChangeContext<'info> {
    #[account(signer)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"threshold change tx"], bump)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut, seeds = [b"multisig"], bump)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut, seeds = [b"escrow"], bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub target: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}