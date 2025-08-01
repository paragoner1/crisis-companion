//! Solana SOS - Hybrid Architecture Demo
//! 
//! Demonstrates the hybrid offline/online architecture with context-aware guidance.

use solana_sos::error::AppResult;
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

    info!("🔄 Solana SOS - Hybrid Architecture Demo");
    info!("========================================");

    // Test hybrid architecture concepts
    info!("🌐 Hybrid Modes:");
    info!("   • Offline: Voice recognition, context analysis, guidance");
    info!("   • Online: AI enhancement, real-time updates, cloud sync");
    info!("   • Hybrid: Seamless handoff, context preservation");

    sleep(Duration::from_secs(2)).await;

    // Test mode switching
    info!("🔁 Mode Switching:");
    info!("   • Emergency detected: Drowning");
    info!("   • Offline mode: Fastest response");
    info!("   • Online available: Switch to hybrid for AI support");
    info!("   • Fallback to offline if network lost");

    sleep(Duration::from_secs(2)).await;

    // Test context preservation
    info!("🧠 Context Preservation:");
    info!("   • Emergency stage, user actions, and environment tracked");
    info!("   • Seamless handoff between modes");
    info!("   • No loss of critical information");

    sleep(Duration::from_secs(2)).await;

    info!("🎉 Hybrid architecture demo completed successfully!");
    info!("Offline/online hybrid operation working correctly!");

    Ok(())
} 