use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::states::*;
use crate::utils::*;

pub fn handle_propose(
    ctx: Context<ProposeContext>,  
    nonce: u64,
    tx_type: TransactionType, 
    is_auto_approve: bool,
) -> Result<()> {
    let tx = &mut ctx.accounts.transaction;
    let multisig = &ctx.accounts.multisig;

    require!(multisig.approvals.contains(&ctx.accounts.proposer.key()), ErrorCode::Unauthorized);

    tx.multisig = multisig.key();
    tx.transaction_type = tx_type.clone();
    tx.approvals = if is_auto_approve {
        vec![*ctx.accounts.proposer.key] // Auto-approve if true
    } else {
        vec![] // Empty if false—proposer must approve separately
    };
    tx.executed = false;
    tx.nonce = nonce;

    match tx_type {
        TransactionType::Transfer { target, amount } => {
            require!(amount > 0, ErrorCode::InvalidAmount); 
            require!(target != Pubkey::default(), ErrorCode::InvalidTarget); 
            tx.transaction_type = TransactionType::Transfer { target, amount };
        }
        TransactionType::ThresholdChange(new_threshold) => {
            require!(new_threshold as usize <= multisig.approvals.len(), ErrorCode::InvalidThreshold);
            require!(new_threshold > 0, ErrorCode::InvalidThreshold); // Optional: ensure non-zero
            tx.transaction_type = TransactionType::ThresholdChange(new_threshold);
        }
    }

    emit!(TransactionEvent {
        tx_key: tx.key(),
        action: "proposed".to_string(),
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct ProposeContext<'info> {
    #[account(mut, signer)]
    pub proposer: Signer<'info>,
    #[account(mut, seeds = [b"multisig"], bump)]
    pub multisig: Account<'info, Multisig>,
    #[account(
        init, 
        payer = proposer, 
        space = 8 + Transaction::INIT_SPACE, 
        seeds = [b"tx", nonce.to_le_bytes().as_ref()],
        bump
    )]
    pub transaction: Account<'info, Transaction>,
    pub system_program: Program<'info, System>,
}