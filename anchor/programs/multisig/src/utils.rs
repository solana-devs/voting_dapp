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