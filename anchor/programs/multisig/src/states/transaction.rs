use anchor_lang::prelude::*;
use crate::utils::TransactionType;

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub multisig: Pubkey,
    pub target: Pubkey,
    pub amount: u64,
    #[max_len(10)]
    pub approvals: Vec<Pubkey>,
    pub executed: bool,
    pub nonce: u64,
    // pub is_threshold_change: bool,
    // pub new_threshold: u8,
    pub transaction_type: TransactionType,
}