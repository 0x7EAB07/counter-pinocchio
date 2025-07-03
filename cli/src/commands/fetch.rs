use anyhow::{anyhow, Result};
use counter_pinocchio::state::Counter;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Keypair, signer::Signer,
};

use crate::helpers::counter::{get_counter_address, get_program_id};

pub async fn fetch_counter(keypair: Keypair, rpc_url: String) -> Result<()> {
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let program_id = get_program_id();

    let authority = keypair.pubkey();
    let (counter_pubkey, _bump) = get_counter_address(&authority, &program_id);

    println!("Authority: {}", authority);
    println!("Counter address: {}", counter_pubkey);

    // Fetch account data
    let account = client.get_account(&counter_pubkey).map_err(|_| {
        anyhow!("Counter does not exist. Create it first with the 'create' command.")
    })?;

    // Deserialize counter data using the program's load method
    let counter = Counter::load(&account.data)
        .map_err(|e| anyhow!("Failed to deserialize counter data: {:?}", e))?;

    let authority_pubkey = Pubkey::new_from_array(counter.authority);

    println!("📊 Counter Information:");
    println!("  Authority: {}", authority_pubkey);
    println!("  Value: {}", counter.value);
    println!("  Bump: {}", counter.bump);

    Ok(())
}
