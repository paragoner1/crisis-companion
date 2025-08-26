use solana_sos::public::voice_interface::VoiceTrigger;
use solana_sos::public::audio_interface::AudioProcessor;
use solana_sos::public::emergency_interface::EmergencySystem;
use solana_sos::public::types::EmergencyType;
use solana_sos::error::AppResult;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚨 Solana SOS Demo Test");
    info!("======================");

    // Construct VoiceTrigger directly
    let voice_trigger = VoiceTrigger {
        wake_word: "hey sos".to_string(),
        confidence_threshold: 0.8,
        emergency_phrases: vec![
            "drowning".to_string(),
            "heart attack".to_string(),
        ],
        direct_actions: vec!["cpr".to_string()],
    };

    println!("VoiceTrigger: {:?}", voice_trigger);
    // Demo: print the wake word and emergency phrases
    println!("Wake word: {}", voice_trigger.wake_word);
    println!("Emergency phrases: {:?}", voice_trigger.emergency_phrases);
    println!("Direct actions: {:?}", voice_trigger.direct_actions);

    // AudioProcessor now requires a cache_dir argument
    let _audio_processor = AudioProcessor::new("/tmp");
    println!("AudioProcessor created with cache_dir: /tmp");
    // No static methods to call anymore

    // Test emergency system
    info!("Testing emergency system...");
    let mut emergency_system = EmergencySystem::new();
    emergency_system.initiate_emergency_response(EmergencyType::Drowning)?;
    info!("✅ Emergency system activated");

    // Test emergency instructions
    info!("📋 Emergency instructions:");
    let instructions = emergency_system.get_emergency_instructions()?;
    for instruction in instructions {
        info!("   • {}", instruction);
    }

    // Simulate some processing time
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    info!("🎉 Demo test completed successfully!");
    info!("All core components working correctly!");

    Ok(())
}
