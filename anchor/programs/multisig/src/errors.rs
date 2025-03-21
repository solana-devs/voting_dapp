use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized signer")]
    Unauthorized,
    #[msg("Invalid threshold value")]
    InvalidThreshold,
    #[msg("Transaction already executed")]
    AlreadyExecuted,
    #[msg("Not enough approvals")]
    NotEnoughApprovals,
    #[msg("Invalid nonce")]
    InvalidNonce,
    #[msg("Approval already exists")]
    AlreadyApproved,
    #[msg("Approval not found")]
    ApprovalNotFound,
    #[msg("Amount should be greater than 0")]
    InvalidAmount,
    #[msg("Target must be set")]
    InvalidTarget,
    #[msg("Missing accounts")]
    MissingAccounts,
}