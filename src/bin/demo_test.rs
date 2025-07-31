use crisis_companion::{
    config::AppConfig,
    voice::VoiceTrigger,
    audio::AudioManager,
    database::EmergencyDatabase,
    coordination::DeviceCoordinator,
    emergency::EmergencyHandler,
    ui::AppUI,
    blockchain::BlockchainManager,
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
    
    info!("🚨 Crisis Companion Demo Test");
    info!("================================");
    
    // Load configuration
    let config = AppConfig::load("config.toml")?;
    info!("✅ Configuration loaded");
    
    // Initialize core components
    let voice_trigger = VoiceTrigger::new(&config.voice)?;
    let audio_manager = AudioManager::new(&config.audio)?;
    let database = EmergencyDatabase::new(&config.database)?;
    let coordinator = DeviceCoordinator::new(&config.coordination)?;
    let emergency_handler = EmergencyHandler::new(&config.emergency)?;
    let ui = AppUI::new(&config.ui)?;
    let blockchain_manager = BlockchainManager::new(&config.blockchain)?;
    
    info!("✅ All components initialized");
    
    // Test 1: Voice Trigger
    info!("🎤 Testing Voice Trigger...");
    let test_trigger = voice_trigger.test_trigger("drowning help").await?;
    if let Some(trigger) = test_trigger {
        info!("✅ Voice trigger detected: {:?}", trigger.emergency_type);
    }
    
    // Test 2: Emergency Response
    info!("🚨 Testing Emergency Response...");
    let response = emergency_handler.start_emergency_response(EmergencyType::Drowning).await?;
    info!("✅ Emergency response started: {:?}", response.emergency_type);
    
    // Test 3: Audio Management
    info!("🔊 Testing Audio Management...");
    audio_manager.set_emergency_volume().await?;
    info!("✅ Emergency volume set");
    
    // Test 4: Database
    info!("💾 Testing Database...");
    let instructions = database.get_emergency_instructions(&EmergencyType::Drowning)?;
    info!("✅ Retrieved {} emergency instructions", instructions.len());
    
    // Test 5: UI
    info!("📱 Testing UI...");
    ui.show_emergency(EmergencyType::Drowning).await?;
    info!("✅ UI emergency display activated");
    
    // Test 6: Blockchain
    info!("⛓️ Testing Blockchain...");
    blockchain_manager.store_audio_hash("demo_hash_123", &EmergencyType::Drowning).await?;
    info!("✅ Audio hash stored on blockchain");
    
    // Test 7: Coordination
    info!("📡 Testing Device Coordination...");
    coordinator.coordinate_emergency_response(EmergencyType::Drowning).await?;
    info!("✅ Device coordination activated");
    
    // Complete emergency response
    emergency_handler.complete_emergency_response().await?;
    info!("✅ Emergency response completed");
    
    info!("🎉 All tests completed successfully!");
    info!("================================");
    info!("Demo ready for hackathon submission!");
    
    Ok(())
}
