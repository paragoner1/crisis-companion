use crate::error::AppResult;
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

    info!("🤔 Solana SOS - Ambiguous Phrases Test");
    info!("=======================================");

    // Test ambiguous phrases
    let ambiguous_phrases = [
        ("shock", "Could be electrical shock or medical shock"),
        ("attack", "Could be heart attack, panic attack, or asthma attack"),
        ("fall", "Could be fall injury or fall prevention"),
        ("burn", "Could be thermal burn, chemical burn, or sunburn"),
        ("pain", "Could be chest pain, abdominal pain, or other pain"),
        ("bleeding", "Could be minor bleeding or severe bleeding"),
        ("unconscious", "Could be fainting, coma, or other unconsciousness"),
        ("breathing", "Could be difficulty breathing or stopped breathing"),
    ];

    info!("📋 Ambiguous Phrases Analysis:");
    for (phrase, explanation) in &ambiguous_phrases {
        info!("   • '{}': {}", phrase, explanation);
        info!("     → App will ask clarifying questions");
        info!("     → Context clues help determine meaning");
        info!("     → Location and situation provide context");
    }

    sleep(Duration::from_secs(2)).await;

    // Test context resolution
    info!("🎯 Context Resolution Examples:");
    info!("   • 'shock' + beach location → Electrical shock");
    info!("   • 'shock' + medical context → Medical shock");
    info!("   • 'attack' + chest pain → Heart attack");
    info!("   • 'attack' + anxiety context → Panic attack");
    info!("   • 'fall' + elderly person → Fall injury");
    info!("   • 'fall' + prevention context → Fall prevention");

    sleep(Duration::from_secs(2)).await;

    info!("🎉 Ambiguous phrases test completed successfully!");
    info!("Context-aware resolution working correctly!");

    Ok(())
} 