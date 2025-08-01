use tracing::{info, Level};
use crisis_companion::{
    app::CrisisCompanionApp,
    error::AppError,
};
use clap::Parser;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    
    /// Run in desktop mode (for testing)
    #[arg(long)]
    desktop: bool,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Initialize logging
    let log_level = if args.debug { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();
    
    info!("Starting Crisis Companion v{}", env!("CARGO_PKG_VERSION"));
    
    info!("Core components initialized successfully");

    // Create and run the main application
    let app = CrisisCompanionApp::new(&args.config).await?;
    
    // Start monitoring
    app.start_monitoring().await?;
    
    if args.desktop {
        info!("Running in desktop mode for testing");
        // Keep the app running for a while in desktop mode
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    } else {
        info!("Running in mobile mode");
        // Keep the app running for a while in mobile mode
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
    
    // Stop the application
    app.stop().await?;
    
    info!("Crisis Companion shutdown complete");
    Ok(())
} 