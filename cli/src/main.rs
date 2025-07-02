use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use counter_pinocchio::{state::Counter, ID as PROGRAM_ID};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use std::fs;

fn get_program_id() -> Pubkey {
    Pubkey::new_from_array(PROGRAM_ID)
}

#[derive(Parser)]
#[command(name = "counter")]
#[command(about = "A CLI tool to interact with the counter Solana program")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new counter account
    Create {
        /// Path to the keypair file
        #[arg(short, long)]
        keypair: String,
        /// RPC URL for the Solana cluster
        #[arg(short, long, default_value = "https://api.devnet.solana.com")]
        rpc_url: String,
    },
    /// Increase the counter value
    Increase {
        /// Path to the keypair file
        #[arg(short, long)]
        keypair: String,
        /// RPC URL for the Solana cluster
        #[arg(short, long, default_value = "https://api.devnet.solana.com")]
        rpc_url: String,
        /// Amount to increase by (default: 1)
        #[arg(short = 'm', long, default_value = "1")]
        amount: u64,
    },
    /// Fetch the current counter value
    Fetch {
        /// Path to the keypair file (needed to derive counter address)
        #[arg(short, long)]
        keypair: String,
        /// RPC URL for the Solana cluster
        #[arg(short, long, default_value = "https://api.devnet.solana.com")]
        rpc_url: String,
    },
}

fn load_keypair(path: &str) -> Result<Keypair> {
    let keypair_data = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read keypair file '{}': {}", path, e))?;

    // Try parsing as JSON array (standard Solana format)
    if let Ok(bytes) = serde_json::from_str::<Vec<u8>>(&keypair_data) {
        return Keypair::try_from(&bytes[..]).map_err(|e| anyhow!("Invalid keypair format: {}", e));
    }

    Err(anyhow!("Keypair file must be in JSON array format"))
}

fn get_counter_address(authority: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"counter", authority.as_ref()], program_id)
}

async fn create_counter(keypair: Keypair, rpc_url: String) -> Result<()> {
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

async fn increase_counter(keypair: Keypair, rpc_url: String, amount: u64) -> Result<()> {
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

async fn fetch_counter(keypair: Keypair, rpc_url: String) -> Result<()> {
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { keypair, rpc_url } => {
            println!("Loading keypair from: {}", keypair);
            println!("Using RPC URL: {}", rpc_url);
            println!();

            let kp = load_keypair(&keypair)?;
            create_counter(kp, rpc_url).await?;
        }
        Commands::Increase {
            keypair,
            rpc_url,
            amount,
        } => {
            println!("Loading keypair from: {}", keypair);
            println!("Using RPC URL: {}", rpc_url);
            println!();

            let kp = load_keypair(&keypair)?;
            increase_counter(kp, rpc_url, amount).await?;
        }
        Commands::Fetch { keypair, rpc_url } => {
            println!("Loading keypair from: {}", keypair);
            println!("Using RPC URL: {}", rpc_url);
            println!();

            let kp = load_keypair(&keypair)?;
            fetch_counter(kp, rpc_url).await?;
        }
    }

    Ok(())
}
