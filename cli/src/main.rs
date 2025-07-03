use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod helpers;

use commands::{create::create_counter, fetch::fetch_counter, increase::increase_counter};
use helpers::keypair::load_keypair;

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
