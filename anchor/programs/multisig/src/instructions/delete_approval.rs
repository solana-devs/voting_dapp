use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::*;

pub fn handle_delete_approval(ctx: Context<DeleteApprovalContext>, signer_to_remove: Pubkey) -> Result<()> {
    let tx = &mut ctx.accounts.transaction;
    let multisig = &ctx.accounts.multisig;

    require!(ctx.accounts.admin.key() == multisig.admin, ErrorCode::Unauthorized);
    require!(!tx.executed, ErrorCode::AlreadyExecuted);

    let index = tx.approvals.iter().position(|&key| key == signer_to_remove)
        .ok_or(ErrorCode::ApprovalNotFound)?;
    tx.approvals.remove(index);
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteApprovalContext<'info> {
    #[account(mut, signer)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut, seeds = [b"multisig"], bump = multisig.bump)]
    pub multisig: Account<'info, Multisig>,
}