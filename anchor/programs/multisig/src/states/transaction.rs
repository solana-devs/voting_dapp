use anchor_lang::prelude::*;
use crate::utils::TransactionType;

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub multisig: Pubkey,
    // pub target: Pubkey,
    // pub amount: u64, //handled by TransactionType enum
    // pub program_id: Pubkey,
    // #[max_len(20)]
    // pub accounts: Vec<TransactionAccount>,
    // #[max_len(30)]
    // pub data: Vec<u8>,
    #[max_len(10)]
    pub approvals: Vec<Pubkey>,
    pub executed: bool,
    pub nonce: u64,
    // pub is_threshold_change: bool,
    // pub new_threshold: u8,
    pub transaction_type: TransactionType,
    pub bump: u8,
}

