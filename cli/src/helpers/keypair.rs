use anyhow::{anyhow, Result};
use solana_sdk::signature::Keypair;
use std::fs;

pub fn load_keypair(path: &str) -> Result<Keypair> {
    let keypair_data = fs::read_to_string(path)
        .map_err(|e| anyhow!("Failed to read keypair file '{}': {}", path, e))?;

    // Try parsing as JSON array (standard Solana format)
    if let Ok(bytes) = serde_json::from_str::<Vec<u8>>(&keypair_data) {
        return Keypair::try_from(&bytes[..]).map_err(|e| anyhow!("Invalid keypair format: {}", e));
    }

    Err(anyhow!("Keypair file must be in JSON array format"))
}
