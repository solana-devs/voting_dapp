// Program Flow

// Initialize:
// Admin creates a multisig with signers, a threshold, and an escrow account.
// Stores admin, signers, threshold, and escrow bump for signing.

// Propose:
// Admin or signer proposes a Transfer (with target and amount) or ThresholdChange (new threshold).
// Uses a unique nonce to generate a transaction PDA ([b"tx", nonce.to_le_bytes()]).
// Optionally auto-approves for the proposer.

// Approve:
// Admin or signers add their approval to a transaction.
// Checks prevent duplicate or unauthorized approvals.

// Delete Approval:
// Admin can remove an approval from a transaction, allowing corrections or resets.

// Execute:
// If approvals meet the threshold, admin or signer executes:
// Transfer: Moves SOL from escrow to the target using CPI.
// ThresholdChange: Updates the multisig threshold.
// Increments nonce to prevent replays and marks the transaction as executed.

use anchor_lang::prelude::*;

pub mod utils;
mod errors;
mod states;
pub mod instructions;

use instructions::*;
use crate::utils::TransactionType;

declare_id!("AcaCRmiqCsJafJqcSBgjSYbZ6gk2445kxZChkkbp9FH9");

#[program]
pub mod multisig {

    use super::*;

    /// Initialize multisig and escrow with admin and signers
    pub fn initialize(ctx: Context<InitializeContext>, approval_list: Vec<Pubkey>, threshold: u8, initial_balance: u64) -> Result<()> {
        handle_initialize(ctx, approval_list, threshold, initial_balance)
    }

    /// Propose a transfer transaction or threshold change
    pub fn propose(
        ctx: Context<ProposeContext>, 
        nonce: u64, 
        tx_type: TransactionType,
        is_auto_approve: bool,
    ) -> Result<()> {
        handle_propose(ctx, nonce, tx_type, is_auto_approve)
    }

    /// Admin or signer approves a transaction or threshold change
    pub fn approve(ctx: Context<ApproveContext>, nonce: u64) -> Result<()> {
        handle_approve(ctx, nonce)
    }

    /// Admin deletes approval
    pub fn delete_approval(ctx: Context<DeleteApprovalContext>, nonce: u64, signer_to_remove: Pubkey) -> Result<()> {
        handle_delete_approval(ctx, nonce, signer_to_remove)
    }

    /// Execute a transaction if threshold met
    pub fn execute<'a, 'info>(ctx: Context<'a, 'a, 'a, 'info, ExecuteContext<'info>>, nonce: u64) -> Result<()> {
        handle_execute(ctx, nonce)
    }
}