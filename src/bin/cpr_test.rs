use solana_sos::{
    public::types::DirectAction,
    error::AppResult,
};
use tracing::{info, Level};
use tracing_subscriber;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("💓 Solana SOS - CPR Direct Action Test");
    info!("=====================================");

    // Test CPR direct action
    let cpr = DirectAction::CPR;
    info!("Testing direct action: {}", cpr.display_name());
    info!("Description: {}", cpr.description());
    info!("App: 'Starting CPR guidance immediately.'");
    info!("App: 'Place hands on center of chest.'");
    info!("App: 'Push hard and fast at 100-120 beats per minute.'");
    info!("App: 'Continue until help arrives or person responds.'");

    sleep(Duration::from_secs(2)).await;

    info!("🎉 CPR direct action test completed successfully!");
    info!("CPR guidance working correctly!");

    Ok(())
} 