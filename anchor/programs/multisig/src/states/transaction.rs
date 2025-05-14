use anchor_lang::prelude::*;
use crate::utils::TransactionType;

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub multisig: Pubkey,
    #[max_len(10)]
    pub approvals: Vec<Pubkey>,
    pub executed: bool,
    pub nonce: u64,
    pub transaction_type: TransactionType,
    pub bump: u8,
}

