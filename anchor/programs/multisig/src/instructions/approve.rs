use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::*;
use crate::utils::*;

pub fn handle_approve(ctx: Context<ApproveContext>, nonce: u64) -> Result<()> {
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

    emit!(TransactionEvent {
        tx_key: tx.key(),
        action: "approved".to_string(),
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct ApproveContext<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,
    #[account(mut, seeds = [b"tx", nonce.to_le_bytes().as_ref()], bump)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut, seeds = [b"multisig"], bump)]
    pub multisig: Account<'info, Multisig>,
}