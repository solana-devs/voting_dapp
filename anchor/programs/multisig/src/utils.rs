use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum TransactionType {
    Transfer,
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