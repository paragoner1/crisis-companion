use solana_sos::{
    public::voice_interface::VoiceTrigger,
    public::audio_interface::AudioProcessor,
    public::emergency_interface::EmergencySystem,
    public::types::EmergencyType,
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

    info!("🚨 Solana SOS Demo Test");
    info!("=======================");

    // Test voice trigger
    info!("Testing voice trigger...");
    let mut voice_trigger = VoiceTrigger::new();
    voice_trigger.activate()?;
    info!("✅ Voice trigger activated");

    // Test audio processor
    info!("Testing audio processor...");
    let _audio_processor = AudioProcessor::new()?;
    info!("✅ Audio processor initialized");

    // Test emergency system
    info!("Testing emergency system...");
    let mut emergency_system = EmergencySystem::new();
    emergency_system.initiate_emergency_response(EmergencyType::Drowning)?;
    info!("✅ Emergency system activated");

    // Test emergency instructions
    let instructions = emergency_system.get_emergency_instructions()?;
    info!("📋 Emergency instructions:");
    for instruction in instructions {
        info!("   • {}", instruction);
    }

    sleep(Duration::from_secs(2)).await;

    info!("🎉 Demo test completed successfully!");
    info!("All core components working correctly!");

    Ok(())
}
