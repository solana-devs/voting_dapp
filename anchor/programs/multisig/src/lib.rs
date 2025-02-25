use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxqSWFEXvUfsicV7opJ2zG9JWxD");

#[program]
pub mod multisig {
    use super::*;

    pub fn create_multisig(ctx: Context<CreateMultisigContext>, seed: String, signers: Vec<Pubkey>, threshold: u8) -> Result<()> {
        let (pda, bump) = Pubkey::find_program_address(&[seed.as_bytes()], &id()); // Derive a PDA for the multisig and use it as the authority.
        require!(ctx.accounts.multisig.key() == pda, ErrorCode::InvalidPDA); 

        let multisig = &mut ctx.accounts.multisig;

        require!(signers.len() > 0 && (threshold as usize) <= signers.len(), ErrorCode::InvalidThreshold);

        multisig.signers = signers;
        multisig.threshold = threshold;
        multisig.bump = bump; // Store for PDA signing later

        Ok(())
    }

    /// Propose a new transaction
    pub fn propose_transaction(ctx: Context<ProposeTransaction>, target: Pubkey, data: Vec<u8>) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;
        tx.multisig = ctx.accounts.multisig.key();
        tx.target = target;
        tx.data = data;
        tx.approvals = vec![];

        Ok(())
    }

    /// Approve a transaction
    pub fn approve_transaction(ctx: Context<ApproveTransaction>) -> Result<()> {
        let tx = &mut ctx.accounts.transaction;
        let multisig = &ctx.accounts.multisig;
        
        // Ensure transaction has not been executed
        require!(!tx.executed, ErrorCode::AlreadyExecuted);
        
        // Ensure sender is a valid signer
        require!(multisig.signers.contains(ctx.accounts.signer.key), ErrorCode::Unauthorized);

        // Prevent duplicate approvals
        require!(!tx.approvals.contains(ctx.accounts.signer.key), ErrorCode::AlreadyApproved);

        tx.approvals.push(*ctx.accounts.signer.key);

        Ok(())
    }

    /// Execute transaction if threshold is met
    pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
        let tx = &ctx.accounts.transaction;
        let multisig = &ctx.accounts.multisig;

        require!(multisig.signers.contains(&ctx.accounts.authority.key()), ErrorCode::Unauthorized);
        require!(!tx.executed, ErrorCode::AlreadyExecuted);
        require!(tx.approvals.len() as u8 >= multisig.threshold, ErrorCode::NotEnoughApprovals);

        // Execute the transaction by calling the target program
        anchor_lang::solana_program::program::invoke(
            &anchor_lang::solana_program::instruction::Instruction {
                program_id: tx.target,
                accounts: vec![], 
                data: tx.data.clone(),
            },
            &[],
        )?;

        tx.executed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMultisigContext<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + Multisig::INIT_SPACE,
        seeds = [b"multisig"],
        bump
    )]
    pub multisig: Account<'info, Multisig>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProposeTransaction<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    #[account(init, payer = proposer, space = 8 + Transaction::INIT_SPACE)]
    pub transaction: Account<'info, Transaction>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveTransaction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
    
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(signer)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub transaction: Account<'info, Transaction>,
    #[account(mut)]
    pub multisig: Account<'info, Multisig>,
}

#[account]
#[derive(InitSpace)]
pub struct Multisig {
    #[max_len(10)] // Maximum of 10 signers
    pub signers: Vec<Pubkey>,
    pub threshold: u8,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Transaction {
    pub multisig: Pubkey,
    pub executed: bool,
    pub target: Pubkey, //This is the destination address where the transaction will be sent. It specifies which program (smart contract) will execute the transaction once approved.
    #[max_len(512)] // Maximum of 512 bytes for transaction data
    pub data: Vec<u8>, //This is the data that will be sent to the target program. It is the actual transaction data that will be executed
    #[max_len(7)]
    pub approvals: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid PDA")]
    InvalidPDA,
    #[msg("Invalid threshold value")]
    InvalidThreshold,
    #[msg("Signer not authorized")]
    Unauthorized,
    #[msg("Transaction already approved by this signer")]
    AlreadyApproved,
    #[msg("Transaction has already been executed")]
    AlreadyExecuted,
    #[msg("Not enough approvals to execute")]
    NotEnoughApprovals,
    #[msg("Invalid owner")]
    InvalidOwner,
}
