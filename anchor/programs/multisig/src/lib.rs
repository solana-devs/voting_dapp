use anchor_lang::prelude::*;

mod utils;
mod errors;
mod states;
pub mod instructions;

use instructions::*;

declare_id!("6VEFARTwWDVyJYmaxAxvGnGWYaRPzVobuvfr92GuFcGT");

#[program]
pub mod multisig {
    use super::*;

    /// Initialize multisig and escrow with admin and signers
    pub fn initialize(ctx: Context<InitializeContext>, approval_list: Vec<Pubkey>, threshold: u8, initial_balance: u64) -> Result<()> {
        handle_initialize(ctx, approval_list, threshold, initial_balance)
    }

    /// Propose a transfer transaction
    pub fn propose_transaction(
        ctx: Context<ProposeTransactionContext>, 
        target: Pubkey, 
        amount: u64, 
        nonce: u64, 
        is_auto_approve: bool,
    ) -> Result<()> {
        handle_propose_transaction(ctx, target, amount, nonce, is_auto_approve)
    }

    /// Propose a threshold change
    pub fn propose_threshold_change(ctx: Context<ProposeThresholdChangeContext>, new_threshold: u8, nonce: u64) -> Result<()> {
        handle_propose_threshold_change(ctx, new_threshold, nonce)
    }

    /// Admin or signer approves a transaction
    pub fn approve_transaction(ctx: Context<ApproveTransactionContext>) -> Result<()> {
        handle_approve_transaction(ctx)
    }

    /// Admin deletes an approval
    pub fn delete_approval(ctx: Context<DeleteApprovalContext>, signer_to_remove: Pubkey) -> Result<()> {
        handle_delete_approval(ctx, signer_to_remove)
    }

    /// Execute a transaction if threshold met
    pub fn execute_transaction(ctx: Context<ExecuteTransactionContext>) -> Result<()> {
        handle_execute_transaction(ctx)
    }
}