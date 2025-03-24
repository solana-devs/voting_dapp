use anchor_lang::prelude::*;

pub mod utils;
mod errors;
mod states;
pub mod instructions;

use instructions::*;
use crate::utils::TransactionType;

declare_id!("3aJ9rFBEoDuDMj2gxHNvV3hRGhmUtmJmMJXgR8QDBjZE");

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