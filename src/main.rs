use tracing::{info, Level};
use solana_sos::{
    app::SolanaSOSApp,
    config::AppConfig,
    error::AppResult,
};
use clap::Parser;
use tracing_subscriber;

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

    // Initialize logging
    if args.verbose {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .init();
    }

    info!("🚨 Starting Solana SOS - The phone you can't live without");

    // Load configuration
    let _config = AppConfig::load(&args.config)?;
    info!("Configuration loaded successfully");

    // Create and initialize the app
    let mut app = SolanaSOSApp::new().await?;
    app.initialize().await?;

    if args.demo {
        info!("Running in demo mode");
        // Demo functionality would go here
    } else {
        info!("Starting Solana SOS application");
        app.run().await?;
    }

    info!("Solana SOS application completed successfully");
    Ok(())
} 