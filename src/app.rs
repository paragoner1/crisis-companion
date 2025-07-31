#![allow(unused_imports, unused_variables, dead_code)]
use crate::error::AppResult;
use crate::{
    types::*,
    config::AppConfig,
    voice::VoiceTrigger,
    audio::AudioManager,
    database::EmergencyDatabase,
    coordination::DeviceCoordinator,
    emergency::EmergencyHandler,
    ui::AppUI,
    blockchain::BlockchainManager,
};
use tracing::{info, warn, error, debug};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use chrono::Utc;

/// Main application coordinator
pub struct CrisisCompanionApp {
    config: AppConfig,
    voice_trigger: VoiceTrigger,
    audio_manager: AudioManager,
    database: EmergencyDatabase,
    coordinator: DeviceCoordinator,
    emergency_handler: EmergencyHandler,
    ui: AppUI,
    blockchain_manager: BlockchainManager,
    is_running: Arc<Mutex<bool>>,
    app_sender: mpsc::Sender<AppCommand>,
}

/// Application commands for the main app
#[derive(Debug, Clone)]
pub enum AppCommand {
    VoiceTriggerDetected(VoiceTriggerResult),
    EmergencyResponseComplete,
    DeviceCoordinationUpdate,
    UIUpdate,
    Shutdown,
}

impl CrisisCompanionApp {
    /// Create a new Crisis Companion application
    pub fn new(
        config: AppConfig,
        voice_trigger: VoiceTrigger,
        audio_manager: AudioManager,
        database: EmergencyDatabase,
        coordinator: DeviceCoordinator,
        emergency_handler: EmergencyHandler,
        ui: AppUI,
        blockchain_manager: BlockchainManager,
    ) -> AppResult<Self> {
        info!("Initializing Crisis Companion application");
        
        let (app_sender, _app_receiver) = mpsc::channel(100);
        let _is_running = Arc::new(Mutex::new(false));
        
        let app = Self {
            config,
            voice_trigger,
            audio_manager,
            database,
            coordinator,
            emergency_handler,
            ui,
            blockchain_manager,
            is_running: _is_running,
            app_sender,
        };
        
        info!("Crisis Companion application initialized successfully");
        Ok(app)
    }
    
    /// Start the application
    pub async fn start(&mut self) -> AppResult<()> {
        info!("Starting Crisis Companion application");
        
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            warn!("Application is already running");
            return Ok(());
        }
        *is_running = true;
        drop(is_running);
        
        // Initialize blockchain connection
        self.blockchain_manager.initialize_solana().await?;
        
        // Start voice trigger listening
        self.voice_trigger.start_listening().await?;
        
        // Start device coordination
        self.coordinator.start_advertising().await?;
        self.coordinator.start_scanning().await?;
        
        // Update UI to show listening status
        self.ui.set_listening(true).await?;
        
        info!("Crisis Companion application started successfully");
        Ok(())
    }
    
    /// Stop the application
    pub async fn stop(&mut self) -> AppResult<()> {
        info!("Stopping Crisis Companion application");
        
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        
        // Stop voice trigger
        self.voice_trigger.stop_listening().await?;
        
        // Stop device coordination
        self.coordinator.stop_advertising()?;
        self.coordinator.stop_scanning()?;
        
        // Update UI
        self.ui.set_listening(false).await?;
        
        // End any active emergency response
        if self.emergency_handler.is_emergency_active() {
            self.emergency_handler.complete_emergency_response().await?;
        }
        
        info!("Crisis Companion application stopped successfully");
        Ok(())
    }
    
    /// Handle voice trigger detection
    async fn handle_voice_trigger(&mut self, trigger: VoiceTriggerResult) -> AppResult<()> {
        info!("Handling voice trigger: {:?}", trigger.emergency_type);
        
        // Set emergency volume
        self.audio_manager.set_emergency_volume().await?;
        
        // Start emergency response
        self.emergency_handler.start_emergency_response(trigger.emergency_type.clone()).await?;
        
        // Update UI
        self.ui.show_emergency(trigger.emergency_type.clone()).await?;
        
        // Coordinate with other devices
        self.coordinator.coordinate_emergency_response(trigger.emergency_type.clone()).await?;
        
        // Get emergency instructions
        let instructions = self.database.get_emergency_instructions(&trigger.emergency_type)?;
        
        // Play first instruction
        if let Some(first_instruction) = instructions.first() {
            let tts_text = format!("{}: {}", first_instruction.title, first_instruction.description);
            self.audio_manager.play_tts(tts_text).await?;
        }
        
        // Store audio hash on blockchain
        self.blockchain_manager.store_audio_hash(&trigger.audio_hash, &trigger.emergency_type).await?;
        
        info!("Voice trigger handled successfully");
        Ok(())
    }
    
    /// Handle emergency response completion
    async fn handle_emergency_completion(&mut self) -> AppResult<()> {
        info!("Handling emergency response completion");
        
        // Complete emergency response
        self.emergency_handler.complete_emergency_response().await?;
        
        // Reset volume
        self.audio_manager.set_volume(self.config.audio.default_volume).await?;
        
        // Update UI
        self.ui.hide_emergency().await?;
        
        info!("Emergency response completion handled");
        Ok(())
    }

    /// Handle device coordination updates
    async fn handle_coordination_update(&mut self) -> AppResult<()> {
        info!("Handling device coordination update");
        
        // Update coordination status
        self.coordinator.test_coordination().await?;
        
        info!("Device coordination update handled");
        Ok(())
    }

    /// Handle UI updates
    async fn handle_ui_update(&mut self) -> AppResult<()> {
        info!("Handling UI update");
        
        // Update UI state
        self.ui.show_statistics().await?;
        
        info!("UI update handled");
        Ok(())
    }

    /// Test the application functionality
    pub async fn test_functionality(&mut self) -> AppResult<()> {
        info!("Testing application functionality");
        
        // Test voice trigger
        let test_trigger = self.voice_trigger.test_trigger("drowning help").await?;
        if let Some(trigger) = test_trigger {
            self.handle_voice_trigger(trigger).await?;
        }
        
        // Test emergency response
        let response = self.emergency_handler.start_emergency_response(EmergencyType::Drowning).await?;
        assert_eq!(response.emergency_type, EmergencyType::Drowning);
        assert_eq!(response.status, ResponseStatus::Active);
        assert!(self.emergency_handler.is_emergency_active());
        
        // Test 911 dialing
        self.emergency_handler.call_911().await?;
        
        // Test location sharing
        self.emergency_handler.share_location().await?;
        
        // Test audio recording
        self.emergency_handler.start_recording().await?;
        self.emergency_handler.stop_recording().await?;
        
        // Complete emergency response
        self.emergency_handler.complete_emergency_response().await?;
        assert!(!self.emergency_handler.is_emergency_active());
        
        info!("Application functionality test completed successfully");
        Ok(())
    }

    /// Test audio functionality
    pub async fn test_audio(&self) -> AppResult<()> {
        info!("Testing audio functionality");
        
        // Test volume control
        self.audio_manager.set_emergency_volume().await?;
        self.audio_manager.set_volume(self.config.audio.default_volume).await?;
        
        // Test TTS playback
        self.audio_manager.play_tts("Test emergency message".to_string()).await?;
        
        // Test stop playback
        self.audio_manager.stop_playback().await?;
        
        // Test audio file playback (this would fail without an actual file)
        // self.audio_manager.play_audio_file("test.mp3".to_string()).await?;
        
        // Test stop playback
        self.audio_manager.stop_playback().await?;
        
        info!("Audio functionality test completed successfully");
        Ok(())
    }
    
    /// Run the application in desktop mode (for testing)
    pub async fn run_desktop(&mut self) -> AppResult<()> {
        info!("Running Crisis Companion in desktop mode");
        
        // Start the application
        self.start().await?;
        
        // Run desktop UI loop
        #[cfg(feature = "desktop")]
        {
            use eframe::egui;
            
            let options = eframe::NativeOptions::default();
            eframe::run_native(
                "Crisis Companion",
                options,
                Box::new(|_cc| {
                    let app = self.clone();
                    Box::new(app)
                }),
            )?;
        }
        
        // Stop the application
        self.stop().await?;
        
        info!("Desktop mode completed");
        Ok(())
    }
    
    /// Run the application in mobile mode
    pub async fn run_mobile(&mut self) -> AppResult<()> {
        info!("Running Crisis Companion in mobile mode");
        
        // Start the application
        self.start().await?;
        
        // Keep the application running
        while *self.is_running.lock().unwrap() {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        
        // Stop the application
        self.stop().await?;
        
        info!("Mobile mode completed");
        Ok(())
    }
    
    /// Test the application functionality
    pub async fn test_app(&mut self) -> AppResult<()> {
        info!("Testing Crisis Companion application");
        
        // Test voice trigger
        let test_trigger = self.voice_trigger.test_trigger("drowning help").await?;
        if let Some(trigger) = test_trigger {
            self.handle_voice_trigger(trigger).await?;
        }
        
        // Test emergency response
        let response = self.emergency_handler.start_emergency_response(EmergencyType::Drowning).await?;
        assert_eq!(response.emergency_type, EmergencyType::Drowning);
        assert_eq!(response.status, ResponseStatus::Active);
        assert!(self.emergency_handler.is_emergency_active());
        
        // Test 911 dialing
        self.emergency_handler.call_911().await?;
        
        // Test location sharing
        self.emergency_handler.share_location().await?;
        
        // Test audio recording
        self.emergency_handler.start_recording().await?;
        self.emergency_handler.stop_recording().await?;
        
        // Complete emergency response
        self.emergency_handler.complete_emergency_response().await?;
        assert!(!self.emergency_handler.is_emergency_active());
        
        info!("Application test completed successfully");
        Ok(())
    }
    
    /// Main application processing loop
    async fn app_processing_loop(
        mut receiver: mpsc::Receiver<AppCommand>,
        is_running: Arc<Mutex<bool>>,
    ) -> AppResult<()> {
        info!("Application processing loop started");
        
        while let Some(command) = receiver.recv().await {
            match command {
                AppCommand::VoiceTriggerDetected(trigger) => {
                    info!("Processing voice trigger: {:?}", trigger.emergency_type);
                    
                    // In a real implementation, this would:
                    // 1. Handle the voice trigger
                    // 2. Coordinate with other components
                    // 3. Update UI and audio
                }
                
                AppCommand::EmergencyResponseComplete => {
                    info!("Processing emergency response complete");
                    
                    // In a real implementation, this would:
                    // 1. Complete the emergency response
                    // 2. Reset all systems
                    // 3. Update UI
                }
                
                AppCommand::DeviceCoordinationUpdate => {
                    debug!("Processing device coordination update");
                    
                    // In a real implementation, this would:
                    // 1. Update coordination status
                    // 2. Adjust device roles
                    // 3. Update UI
                }
                
                AppCommand::UIUpdate => {
                    debug!("Processing UI update");
                    
                    // In a real implementation, this would:
                    // 1. Update UI state
                    // 2. Refresh display
                }
                
                AppCommand::Shutdown => {
                    info!("Processing shutdown command");
                    break;
                }
            }
        }
        
        info!("Application processing loop stopped");
        Ok(())
    }

    pub async fn shutdown(&mut self) -> AppResult<()> {
        info!("Shutting down Crisis Companion application");
        
        // Stop voice listening
        self.voice_trigger.stop_listening().await?;
        
        // Complete any active emergency response
        if self.emergency_handler.is_emergency_active() {
            self.emergency_handler.complete_emergency_response().await?;
        }
        
        // Stop audio playback
        self.audio_manager.stop_playback().await?;
        
        info!("Crisis Companion shutdown complete");
        Ok(())
    }
}

impl Drop for CrisisCompanionApp {
    fn drop(&mut self) {
        info!("Crisis Companion application shutting down");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    
    #[tokio::test]
    async fn test_app_creation() {
        let config = AppConfig::default();
        let voice_trigger = VoiceTrigger::new(&config.voice).unwrap();
        let audio_manager = AudioManager::new(&config.audio).unwrap();
        let database = EmergencyDatabase::new(&config.database).unwrap();
        let coordinator = DeviceCoordinator::new(&config.coordination).unwrap();
        let emergency_handler = EmergencyHandler::new(&config.emergency).unwrap();
        let ui = AppUI::new(&config.ui).unwrap();
        let blockchain_manager = BlockchainManager::new(&config.blockchain).unwrap();
        
        let app = CrisisCompanionApp::new(
            config,
            voice_trigger,
            audio_manager,
            database,
            coordinator,
            emergency_handler,
            ui,
            blockchain_manager,
        );
        assert!(app.is_ok());
    }
    
    #[tokio::test]
    async fn test_app_lifecycle() {
        let config = AppConfig::default();
        let voice_trigger = VoiceTrigger::new(&config.voice).unwrap();
        let audio_manager = AudioManager::new(&config.audio).unwrap();
        let database = EmergencyDatabase::new(&config.database).unwrap();
        let coordinator = DeviceCoordinator::new(&config.coordination).unwrap();
        let emergency_handler = EmergencyHandler::new(&config.emergency).unwrap();
        let ui = AppUI::new(&config.ui).unwrap();
        let blockchain_manager = BlockchainManager::new(&config.blockchain).unwrap();
        
        let mut app = CrisisCompanionApp::new(
            config,
            voice_trigger,
            audio_manager,
            database,
            coordinator,
            emergency_handler,
            ui,
            blockchain_manager,
        ).unwrap();
        
        // Test start
        app.start().await.unwrap();
        
        // Test stop
        app.stop().await.unwrap();
    }
} 