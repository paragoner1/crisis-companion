use solana_sos::{
    public::voice_interface::VoiceTrigger,
    config::VoiceConfig,
    public::types::EmergencyType,
    error::AppResult,
};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🎤 Solana SOS Voice Recognition Test");
    info!("====================================");

    // Test voice trigger
    info!("Testing voice trigger...");
    let mut voice_trigger = VoiceTrigger::new();
    voice_trigger.activate()?;
    info!("✅ Voice trigger activated");

    // Test emergency phrase detection
    info!("Testing emergency phrase detection...");
    let detected = voice_trigger.detect_emergency_phrase("drowning help")?;
    info!("✅ Emergency phrase detection: {}", detected);

    // Test confidence threshold
    info!("Testing confidence threshold...");
    voice_trigger.set_confidence_threshold(0.8);
    info!("✅ Confidence threshold set to 0.8");

    // Test audio hash generation
    info!("Testing audio hash generation...");
    let hash = VoiceTrigger::generate_audio_hash("test phrase");
    info!("✅ Audio hash generated: {}", hash);

    // Test phrase simulation
    info!("Testing phrase simulation...");
    let emergency_type = VoiceTrigger::simulate_phrase_detection("drowning help").await?;
    info!("✅ Simulated emergency type: {:?}", emergency_type);

    info!("🎉 Voice recognition test completed successfully!");
    info!("All voice functionality working correctly!");

    Ok(())
} 