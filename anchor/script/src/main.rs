// an RPC-based integration test that successfully sets up the multisig and escrow on a live Solana cluster
// and proposes a transaction to transfer 0.02 SOL to the admin's address.


use std::rc::Rc;
use std::str::FromStr;

// use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_client::rpc_config::RpcSendTransactionConfig;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::{Client, Cluster};
// use anchor_client::solana_sdk::transaction::Transaction;
use anchor_lang::solana_program;
use anyhow::Result;
use bs58;
// use dotenv::dotenv;

// use spl_associated_token_account::get_associated_token_address;
// use multisig::accounts::InitializeContext;
// use multisig::instructions::initialize;
// use multisig::instructions::initialize::InitializeContext;
// use multisig::program::Multisig;


fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let env_signer_private_key = std::env::var("SIGNER_PRIV_KEY").expect("SIGNER_PRIV_KEY must be set.");
    let env_solana_rpc = std::env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set.");
    let env_program_id = std::env::var("MULTISIG_PROGRAM_ID").expect("MULTISIG_PROGRAM_ID must be set.");

    let private_key_bytes = bs58::decode(env_signer_private_key).into_vec()?;
    let admin = Rc::new(Keypair::from_bytes(&private_key_bytes)?);
    let program_id = Pubkey::from_str(&env_program_id).unwrap();



    println!("Admin pubkey: {}", admin.pubkey());
    println!("program_id: {}", program_id);

    let client = Client::new_with_options(
        Cluster::from_str(&env_solana_rpc).unwrap(),
        admin.clone(),
        CommitmentConfig::processed(),
    );
    let program = client.program(program_id)?;

    let (escrow_pda, _) = Pubkey::find_program_address(&[b"escrow"], &program_id); //bump??
    let (multisig_pda, _) = Pubkey::find_program_address(&[b"multisig"], &program_id);

    let approval_list = vec![admin.pubkey(), admin.pubkey(), admin.pubkey(), admin.pubkey()]; // Example signer list

    let tx = program
        .request()
        .accounts(multisig::accounts::InitializeContext {
            admin: admin.pubkey(),
            multisig: multisig_pda,
            escrow: escrow_pda,
            system_program: solana_program::system_program::ID,
        })
        .args(multisig::instruction::Initialize {
            approval_list,
            threshold: 2,
            initial_balance: 100_000_000, // 0.1 SOL
        })
        .payer(admin.clone())
        .signer(&*admin)
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;

    println!("Initialized multisig - Signature: {}", tx);


    let (tx_pda, _) = Pubkey::find_program_address(&[b"transaction"], &program_id);
    // let transaction = Keypair::new();

        let tx = program
        .request()
        .accounts(multisig::accounts::ProposeTransactionContext {
            proposer: admin.pubkey(),
            multisig: multisig_pda,
            transaction: tx_pda,
            system_program: solana_program::system_program::ID,
        })
        .args(multisig::instruction::ProposeTransaction {
            target: admin.pubkey(),
            amount: 20_000_000, // 0.02 SOL
            nonce: 0,
            is_auto_approve: true,
        })
        .payer(admin.clone())
        .signer(&*admin)
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: true, //"true" hides detailed errors. Turned it off for better logs -> had "RPC response error -32002: Transaction simulation failed: Blockhash not found", so set to true again
            max_retries: Some(5),
            ..Default::default()
        })?;
        
    println!("Propose a multisig tx - Signature: {}", tx);


    let (threshold_change_tx_pda, _) = Pubkey::find_program_address(&[b"threshold change tx"], &program_id);

        let tx = program
        .request()
        .accounts(multisig::accounts::ProposeThresholdChangeContext {
            proposer: admin.pubkey(),
            multisig: multisig_pda,
            transaction: threshold_change_tx_pda,
            system_program: solana_program::system_program::ID,
        })
        .args(multisig::instruction::ProposeThresholdChange {
            new_threshold: 1,
            nonce: 0,
        })
        .payer(admin.clone())
        .signer(&*admin)
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: true, 
            ..Default::default()
        })?;
        
    println!("Propose a threshold change tx - Signature: {}", tx);


        let tx = program
        .request()
        .accounts(multisig::accounts::ApproveThresholdChangeContext {
            signer: admin.pubkey(),
            transaction: threshold_change_tx_pda,
            multisig: multisig_pda,
        })
        .args(multisig::instruction::ApproveThresholdChange {})
        .payer(admin.clone())
        .signer(&*admin)
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: true, 
            ..Default::default()
        })?;
        
    println!("Approved threshold change - Signature: {}", tx);
    Ok(())
}