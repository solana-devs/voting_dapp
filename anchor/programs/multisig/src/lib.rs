use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

declare_id!("Fg6PaFpoGXkYsidMpWxqSWFEXvUfsicV7opJ2zG9JWxD");

#[program]
pub mod multisig_escrow {
    use super::*;

    /// Initialize multisig and escrow with admin and signers
    pub fn initialize(ctx: Context<InitializeContext>, signers: Vec<Pubkey>, threshold: u8, initial_balance: u64) -> Result<()> {
        let multisig = &mut ctx.accounts.multisig;
        require!(signers.len() > 0 && threshold as usize <= signers.len(), ErrorCode::InvalidThreshold);
        
        multisig.admin = *ctx.accounts.admin.key;
        multisig.signers = signers;
        multisig.threshold = threshold;
        multisig.nonce = 0;

        // Transfer SOL from admin to escrow
        let cpi_accounts = Transfer {
            from: ctx.accounts.admin.to_account_info(),
            to: ctx.accounts.escrow.to_account_info(),
        };
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, initial_balance)?;

        Ok(())
    }

    /// Propose a transfer transaction
    pub fn propose_transaction(
        ctx: Context<ProposeTransactionContext>, 
        target: Pubkey, 
        amount: u64, 
        nonce: u64, 
        is_auto_approve: bool,
    ) -> Result<()> {
        let multisig = &ctx.accounts.multisig;
        require!(multisig.signers.contains(&ctx.accounts.proposer.key()), ErrorCode::Unauthorized);

        let tx = &mut ctx.accounts.transaction;
        tx.multisig = multisig.key();
        tx.target = target;
        tx.amount = amount;
        tx.approvals = if is_auto_approve {
            vec![*ctx.accounts.proposer.key] // Auto-approve if true
        } else {
            vec![] // Empty if false—proposer must approve separately
        };
        tx.executed = false;
        tx.nonce = nonce;
        tx.transaction_type = TransactionType::Transfer;

        emit!(TransactionEvent {
            tx_key: tx.key(),
            action: "transfer_proposed".to_string(),
        });
        Ok(())
    }

    /// Propose a threshold change
    pub fn propose_threshold_change(ctx: Context<ProposeThresholdChangeContext>, new_threshold: u8, nonce: u64) -> Result<()> {
        let multisig = &ctx.accounts.multisig;
        require!(multisig.signers.contains(&ctx.accounts.proposer.key()), ErrorCode::Unauthorized);
        require!(new_threshold as usize <= multisig.signers.len(), ErrorCode::InvalidThreshold);

        let tx = &mut ctx.accounts.transaction;
        tx.multisig = multisig.key();
        tx.approvals = vec![*ctx.accounts.proposer.key]; // Auto-approve
        tx.executed = false;
        tx.nonce = nonce;
        tx.transaction_type = TransactionType::ThresholdChange(new_threshold);

        emit!(TransactionEvent {
            tx_key: tx.key(),
            action: "threshold_change_proposed".to_string(),
        });
        Ok(())
    }

    /// Admin or signer approves a transaction
    pub fn approve_transaction(ctx: Context<ApproveTransactionContext>) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;
        let multisig = &ctx.accounts.multisig;
        let signer_key = ctx.accounts.signer.key();

        require!(!tx.executed, ErrorCode::AlreadyExecuted);
        require!(
            multisig.signers.contains(&signer_key) || signer_key == multisig.admin,
            ErrorCode::Unauthorized
        );
        require!(!tx.approvals.contains(&signer_key), ErrorCode::AlreadyApproved);

        tx.approvals.push(signer_key);
        Ok(())
    }

    /// Admin deletes an approval
    pub fn delete_approval(ctx: Context<DeleteApprovalContext>, signer_to_remove: Pubkey) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;
        let multisig = &ctx.accounts.multisig;

        require!(ctx.accounts.admin.key() == multisig.admin, ErrorCode::Unauthorized);
        require!(!tx.executed, ErrorCode::AlreadyExecuted);

        let index = tx.approvals.iter().position(|&key| key == signer_to_remove)
            .ok_or(ErrorCode::ApprovalNotFound)?;
        tx.approvals.remove(index);
        Ok(())
    }

    /// Execute a transaction if threshold met
    pub fn execute_transaction(ctx: Context<ExecuteTransactionContext>) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;
        let multisig = &mut ctx.accounts.multisig;
    
        require!(multisig.signers.contains(&ctx.accounts.authority.key()), ErrorCode::Unauthorized);
        require!(!tx.executed, ErrorCode::AlreadyExecuted);
        require!(tx.nonce == multisig.nonce, ErrorCode::InvalidNonce);
        require!(tx.approvals.len() as u8 >= multisig.threshold, ErrorCode::NotEnoughApprovals);
    
        multisig.nonce += 1;
        match tx.transaction_type {
            TransactionType::Transfer => {
                // Transfer SOL from escrow to target
                let cpi_accounts = Transfer {
                    from: ctx.accounts.escrow.to_account_info(),
                    to: ctx.accounts.target.to_account_info(),
                };
                let cpi_program = ctx.accounts.system_program.to_account_info();
                let seeds: &[&[u8]] = &[b"escrow", &[ctx.accounts.escrow.bump]]; // escrow PDA’s seeds
                let signer_seeds = &[seeds]; // wraps it in an outer slice, making it &[&[&[u8]]]—a list with one signer’s seed set
                let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
                transfer(cpi_ctx, tx.amount)?;
            }
            TransactionType::ThresholdChange(new_threshold) => {
                require!(new_threshold as usize <= multisig.signers.len(), ErrorCode::InvalidThreshold);
                multisig.threshold = new_threshold;
            }
        }
    
        tx.executed = true;
        emit!(TransactionEvent {
            tx_key: tx.key(),
            action: "executed".to_string(),
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut, signer)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Multisig::INIT_SPACE,
    )]
    pub multisig: Account<'info, Multisig>,
    #[account(
        init,
        payer = admin,
        space = 8 + Escrow::INIT_SPACE, 
        seeds = [b"escrow"],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProposeTransactionContext<'info> {
    #[account(mut, signer)]
    pub proposer: Signer<'info>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(init, payer = proposer, space = 8 + Transaction::INIT_SPACE)]
    pub transaction: Account<'info, Transaction>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProposeThresholdChangeContext<'info> {
    #[account(mut, signer)]
    pub proposer: Signer<'info>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(init, payer = proposer, space = 8 + Transaction::INIT_SPACE)]
    pub transaction: Account<'info, Transaction>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveTransactionContext<'info> {
    #[account(mut, signer)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
}

#[derive(Accounts)]
pub struct DeleteApprovalContext<'info> {
    #[account(mut, signer)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
}

#[derive(Accounts)]
pub struct ExecuteTransactionContext<'info> {
    #[account(signer)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(mut, seeds = [b"escrow"], bump = escrow.bump)]
    pub escrow: Account<'info, Escrow>,
    #[account(mut)]
    pub target: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    pub admin: Pubkey,
    #[max_len(10)]
    pub signers: Vec<Pubkey>,
    pub threshold: u8,
    pub nonce: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub balance: u64,
    pub bump: u8,
}

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
    pub is_threshold_change: bool,
    pub new_threshold: u8,
    pub transaction_type: TransactionType,
}

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
}