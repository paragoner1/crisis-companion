use tracing::{info, Level};
use crisis_companion::{
    app::CrisisCompanionApp,
    config::AppConfig,
    error::AppError,
    voice::VoiceTrigger,
    audio::AudioManager,
    database::EmergencyDatabase,
    coordination::DeviceCoordinator,
    emergency::EmergencyHandler,
    ui::AppUI,
    blockchain::BlockchainManager,
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
    
    // Load configuration
    let config = AppConfig::load(&args.config)?;
    info!("Configuration loaded from {}", args.config);
    
    // Initialize core components
    let voice_trigger = VoiceTrigger::new(&config.voice)?;
    let audio_manager = AudioManager::new(&config.audio)?;
    let database = EmergencyDatabase::new(&config.database)?;
    let coordinator = DeviceCoordinator::new(&config.coordination)?;
    let emergency_handler = EmergencyHandler::new(&config.emergency)?;
    let ui = AppUI::new(&config.ui)?;
    let blockchain_manager = BlockchainManager::new(&config.blockchain)?;

    info!("Core components initialized successfully");

    // Create and run the main application
    let mut app = CrisisCompanionApp::new(
        config,
        voice_trigger,
        audio_manager,
        database,
        coordinator,
        emergency_handler,
        ui,
        blockchain_manager,
    )?;
    
    if args.desktop {
        info!("Running in desktop mode for testing");
        app.run_desktop().await?;
    } else {
        info!("Running in mobile mode");
        app.run_mobile().await?;
    }
    
    info!("Crisis Companion shutdown complete");
    Ok(())
} 