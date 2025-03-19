use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::*;

pub fn handle_approve_transaction(ctx: Context<ApproveTransactionContext>) -> Result<()> {
    let tx = &mut ctx.accounts.transaction;
    let multisig = &ctx.accounts.multisig;
    let signer_key = ctx.accounts.signer.key();

    require!(!tx.executed, ErrorCode::AlreadyExecuted);
    require!(
        multisig.approvals.contains(&signer_key) || signer_key == multisig.admin,
        ErrorCode::Unauthorized
    );
    require!(!tx.approvals.contains(&signer_key), ErrorCode::AlreadyApproved);

    tx.approvals.push(signer_key);
    Ok(())
}

#[derive(Accounts)]
pub struct ApproveTransactionContext<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"approve tx"], bump)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut, seeds = [b"multisig"], bump = multisig.bump)]
    pub multisig: Account<'info, Multisig>,
}