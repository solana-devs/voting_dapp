use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::*;
use crate::utils::*;

pub fn handle_propose_transaction(
    ctx: Context<ProposeTransactionContext>, 
    target: Pubkey, 
    amount: u64, 
    nonce: u64, 
    is_auto_approve: bool,
) -> Result<()> {
    let multisig = &ctx.accounts.multisig;
    msg!("Approvals: {:?}", multisig.approvals);
    require!(multisig.approvals.contains(&ctx.accounts.proposer.key()), ErrorCode::Unauthorized);

    let tx = &mut ctx.accounts.transaction;
    tx.multisig = multisig.key();
    tx.target = target;
    tx.amount = amount;
    tx.approvals = if is_auto_approve {
        vec![*ctx.accounts.proposer.key] // Auto-approve if true
    } else {
        vec![] // Empty if false—proposer must approve separately
    };
    tx.executed = false;
    tx.nonce = nonce;
    tx.transaction_type = TransactionType::Transfer;
    // tx.bump = ctx.bumps.transaction;

    emit!(TransactionEvent {
        tx_key: tx.key(),
        action: "transfer_proposed".to_string(),
    });
    Ok(())
}

#[derive(Accounts)]
pub struct ProposeTransactionContext<'info> {
    #[account(mut, signer)]
    pub proposer: Signer<'info>,
    #[account(mut, seeds = [b"multisig"], bump)]
    pub multisig: Account<'info, Multisig>,
    #[account(
        init, 
        payer = proposer, 
        space = 8 + Transaction::INIT_SPACE, 
        seeds = [b"transaction"], 
        bump
    )]
    pub transaction: Account<'info, Transaction>,
    pub system_program: Program<'info, System>,
}