use crate::{
    app::SolanaSOSApp,
    config::AppConfig,
    error::AppResult,
};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    #[arg(short, long)]
    verbose: bool,

    #[arg(long)]
    demo: bool,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = Args::parse();

    println!("🚨 Starting Solana SOS - Creating the phone you can't live without");

    // Load configuration
    let _config = AppConfig::load(&args.config)?;
    println!("Configuration loaded successfully");

    // Create and initialize the app
    let mut app = SolanaSOSApp::new().await?;
    app.initialize().await?;

    if args.demo {
        println!("Running in demo mode");
        // Demo functionality would go here
    } else {
        println!("Starting Solana SOS application");
        app.run().await?;
    }

    println!("Solana SOS application completed successfully");
    Ok(())
} 