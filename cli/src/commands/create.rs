use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};

use crate::helpers::counter::{get_counter_address, get_program_id};

pub async fn create_counter(keypair: Keypair, rpc_url: String) -> Result<()> {
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let program_id = get_program_id();

    let authority = keypair.pubkey();
    let (counter_pubkey, _) = get_counter_address(&authority, &program_id);

    println!("Authority: {}", authority);
    println!("Counter address: {}", counter_pubkey);

    // Check if counter already exists
    if let Ok(_account) = client.get_account(&counter_pubkey) {
        return Err(anyhow!("Counter already exists for this authority"));
    }

    // Create instruction
    let create_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(counter_pubkey, false),
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ],
        data: vec![0], // Create discriminator
    };

    // Get recent blockhash
    let recent_blockhash = client.get_latest_blockhash()?;

    // Create and send transaction
    let transaction = Transaction::new_signed_with_payer(
        &[create_instruction],
        Some(&authority),
        &[&keypair],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("✅ Counter created successfully!");
    println!("Transaction signature: {}", signature);

    Ok(())
}
