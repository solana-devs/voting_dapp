use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::*;
use crate::utils::*;

pub fn handle_propose_threshold_change(ctx: Context<ProposeThresholdChangeContext>, new_threshold: u8, nonce: u64) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    require!(multisig.approvals.contains(&ctx.accounts.proposer.key()), ErrorCode::Unauthorized);
    require!(new_threshold as usize <= multisig.approvals.len(), ErrorCode::InvalidThreshold);

    let tx = &mut ctx.accounts.transaction;
    tx.multisig = multisig.key();
    tx.approvals = vec![*ctx.accounts.proposer.key]; // Auto-approve
    tx.executed = false;
    tx.nonce = nonce;
    tx.transaction_type = TransactionType::ThresholdChange(new_threshold);

    emit!(TransactionEvent {
        tx_key: tx.key(),
        action: "threshold_change_proposed".to_string(),
    });
    Ok(())
}

#[derive(Accounts)]
pub struct ProposeThresholdChangeContext<'info> {
    #[account(mut, signer)]
    pub proposer: Signer<'info>,
    #[account(mut, seeds = [b"multisig"], bump = multisig.bump)]
    pub multisig: Account<'info, Multisig>,
    #[account(init, payer = proposer, space = 8 + Transaction::INIT_SPACE)]
    pub transaction: Account<'info, Transaction>,
    pub system_program: Program<'info, System>,
}