use anchor_lang::prelude::*;
use anchor_client::{Client, Cluster};
use multisig::{self, InitializeContext};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::rc::Rc;

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let env_signer_private_key = std::env::var("SIGNER_PRIV_KEY").expect("SIGNER_PRIV_KEY must be set.");
    let env_solana_rpc = std::env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set.");
    let env_program_id = std::env::var("MULTISIG_PROGRAM_ID").expect("MULTISIG_PROGRAM_ID must be set.");

    let private_key_bytes = bs58::decode(env_signer_private_key).into_vec()?;
    let admin = Rc::new(Keypair::from_bytes(&private_key_bytes)?);
    let program_id = Pubkey::from_str(&env_program_id).unwrap();

    let client = Client::new_with_options(
        Cluster::from_str(&env_solana_rpc).unwrap(),
        admin.clone(),
        CommitmentConfig::processed(),
    );
    let program = client.program(program_id)?;

    let multisig = Keypair::new();
    let escrow = Keypair::new();
    let (escrow_pda, _) = Pubkey::find_program_address(&[b"escrow"], &program_id);
    let signers = vec![admin.pubkey()]; // Example signer list

    let tx = program
        .request()
        .accounts(multisig::InitializeContext {
            admin: admin.pubkey(),
            multisig: multisig.pubkey(),
            escrow: escrow_pda,
            system_program: solana_program::system_program::ID,
        })
        .args(multisig::Initialize {
            signers,
            threshold: 1,
            initial_balance: 100_000_000, // 0.1 SOL
        })
        .payer(admin.clone())
        .signer(&*admin)
        .signer(&multisig)
        .signer(&escrow)
        .send_with_spinner_and_config(RpcSendTransactionConfig {
            skip_preflight: true,
            ..Default::default()
        })?;

    println!("Initialized multisig - Signature: {}", tx);
    Ok(())
}