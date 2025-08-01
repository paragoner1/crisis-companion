use crisis_companion::{
    voice::VoiceTrigger,
    config::VoiceConfig,
    types::EmergencyType,
};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🎤 Voice Trigger System Test");
    info!("================================");

    // Create voice trigger with noise filtering
    let config = VoiceConfig::default();
    let mut voice_trigger = VoiceTrigger::new(&config)?;
    
    info!("✅ Voice trigger initialized with RNNoise filtering");

    // Test 1: Show available emergency phrases
    info!("📋 Available Emergency Phrases:");
    for phrase in voice_trigger.get_emergency_phrases() {
        info!("  • {}", phrase);
    }

    // Test 2: Test specific emergency detection
    info!("🎯 Testing Emergency Detection:");
    
    let test_phrases = [
        "drowning help",
        "fire help", 
        "choking help",
        "heart attack help",
    ];

    for phrase in test_phrases {
        info!("Testing phrase: '{}'", phrase);
        match voice_trigger.test_trigger(phrase).await {
            Ok(Some(trigger)) => {
                info!("✅ Triggered: {:?} (confidence: {:.2})", 
                      trigger.emergency_type, trigger.confidence);
            }
            Ok(None) => {
                info!("❌ No trigger for: {}", phrase);
            }
            Err(e) => {
                info!("⚠️ Error testing {}: {}", phrase, e);
            }
        }
    }

    // Test 3: Simulate continuous listening
    info!("🎧 Starting Continuous Listening (30 seconds)...");
    voice_trigger.start_listening().await?;
    
    // Let it run for 30 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    
    voice_trigger.stop_listening().await?;
    info!("✅ Listening stopped");

    info!("🎉 Voice trigger test completed!");
    info!("================================");

    Ok(())
} 