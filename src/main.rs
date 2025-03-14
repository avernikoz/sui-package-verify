use anyhow::{anyhow, Result};
use clap::Parser;
use move_core_types::account_address::AccountAddress;
use std::path::PathBuf;
use std::str::FromStr;
use sui_move_build::BuildConfig;
use sui_sdk::SuiClientBuilder;
use sui_source_validation::{BytecodeSourceVerifier, ValidationMode};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[clap(
    name = "sui-verify",
    about = "Verify Move package source against on-chain bytecode"
)]
struct Args {
    /// Path to the package to verify
    #[clap(long)]
    package_path: PathBuf,

    /// Address of the package on-chain (if different from the address in the manifest)
    #[clap(long)]
    address: Option<String>,

    /// Network to verify against (mainnet, testnet, devnet, localnet)
    #[clap(long, default_value = "mainnet")]
    network: String,

    /// Enable verbose logging
    #[clap(long)]
    verbose: bool,

    #[clap(long)]
    rpc_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Setup logging
    let log_level = if args.verbose { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    // Determine RPC URL based on network or custom URL
    let rpc_url = if let Some(url) = &args.rpc_url {
        url.clone()
    } else {
        match args.network.to_lowercase().as_str() {
            "mainnet" => "https://fullnode.mainnet.sui.io:443".to_string(),
            "testnet" => "https://fullnode.testnet.sui.io:443".to_string(),
            "devnet" => "https://fullnode.devnet.sui.io:443".to_string(),
            "localnet" => "http://127.0.0.1:9000".to_string(),
            _ => return Err(anyhow!("Unknown network: {}", args.network)),
        }
    };
    
    info!("Verifying package at {} on {}", args.package_path.display(), 
          if args.rpc_url.is_some() { "custom RPC" } else { &args.network });
    
    // Build the package
    let build_config = BuildConfig::new_for_testing();
    info!("Building package...");
    let compiled_package = build_config.build(&args.package_path)?;
    
    // Create Sui client
    info!("Connecting to {} at {}", args.network, rpc_url);
    let sui_client = SuiClientBuilder::default().build(rpc_url).await?;
    let read_api = sui_client.read_api();
    
    // Create the verifier
    let verifier = BytecodeSourceVerifier::new(read_api);
    
    // Choose validation mode based on CLI args
    let mode = if let Some(addr_str) = args.address {
        let addr = AccountAddress::from_hex_literal(&addr_str)
            .or_else(|_| AccountAddress::from_str(&addr_str))
            .map_err(|e| anyhow!("Invalid address format: {}", e))?;
        
        info!("Verifying package at address {}", addr);
        ValidationMode::root_at(addr)
    } else {
        ValidationMode::root()
    };
    
    // Run verification
    info!("Starting verification...");
    match verifier.verify(&compiled_package, mode).await {
        Ok(()) => {
            println!("✅ Verification successful!");
            Ok(())
        }
        Err(e) => {
            println!("❌ Verification failed:");
            println!("  - {}", e);
            Err(anyhow!("Verification failed"))
        }
    }
}