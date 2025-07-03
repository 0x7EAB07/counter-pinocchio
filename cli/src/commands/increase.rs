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

pub async fn increase_counter(keypair: Keypair, rpc_url: String, amount: u64) -> Result<()> {
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let program_id = get_program_id();

    let authority = keypair.pubkey();
    let (counter_pubkey, bump) = get_counter_address(&authority, &program_id);

    println!("PDA bump: {}", bump);
    println!("Authority: {}", authority);
    println!("Counter address: {}", counter_pubkey);
    println!("Increasing by: {}", amount);

    // Check if counter exists
    client.get_account(&counter_pubkey).map_err(|_| {
        anyhow!("Counter does not exist. Create it first with the 'create' command.")
    })?;

    // Prepare instruction data: discriminator (1) + amount (8 bytes)
    let mut instruction_data = vec![1]; // Increase discriminator
    instruction_data.extend_from_slice(&amount.to_le_bytes());

    // Create instruction
    let increase_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(counter_pubkey, false),
            AccountMeta::new_readonly(authority, false),
        ],
        data: instruction_data,
    };

    // Get recent blockhash
    let recent_blockhash = client.get_latest_blockhash()?;

    // Create and send transaction
    let transaction = Transaction::new_signed_with_payer(
        &[increase_instruction],
        Some(&authority),
        &[&keypair],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("✅ Counter increased successfully!");
    println!("Transaction signature: {}", signature);

    Ok(())
}
