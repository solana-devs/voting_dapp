use anchor_lang::prelude::*;

#[derive(Clone, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub enum TransactionType {
    Transfer { target: Pubkey, amount: u64 },
    ThresholdChange(u8),
}

#[event]
pub struct TransactionEvent {
    pub tx_key: Pubkey,
    pub action: String,
}

#[event]
pub struct TransactionApprovedEvent {
    pub tx_key: Pubkey,
    pub signer: Pubkey,
    pub transaction_type: TransactionType,
}
