use crate::error::AppResult;
use crate::{AppError, types::*};
use crate::config::UIConfig;
use tracing::{info, warn, error, debug};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use uuid::Uuid;

/// UI manager for cross-platform user interface
#[derive(Clone, Debug)]
pub struct AppUI {
    config: UIConfig,
    ui_state: Arc<Mutex<UIState>>,
    ui_sender: mpsc::Sender<UICommand>,
    #[cfg(feature = "desktop")]
    egui_context: Option<eframe::egui::Context>,
}

/// UI state for managing interface state
#[derive(Debug, Clone)]
pub struct UIState {
    pub is_emergency_mode: bool,
    pub current_emergency: Option<EmergencyType>,
    pub current_instruction: Option<EmergencyInstruction>,
    pub instruction_step: u32,
    pub total_steps: u32,
    pub is_listening: bool,
    pub volume_level: f32,
    pub nearby_devices_count: u32,
    pub battery_level: f32,
    pub location: Option<(f64, f64)>,
}

/// UI commands for the UI manager
#[derive(Debug, Clone)]
pub enum UICommand {
    ShowEmergency(EmergencyType),
    ShowInstruction(EmergencyInstruction),
    UpdateProgress(u32, u32),
    SetListening(bool),
    SetVolume(f32),
    UpdateNearbyDevices(u32),
    UpdateBattery(f32),
    UpdateLocation(Option<(f64, f64)>),
    HideEmergency,
    ShowSettings,
    ShowStatistics,
}

impl AppUI {
    /// Create a new UI manager
    pub fn new(config: &UIConfig) -> AppResult<Self> {
        info!("Initializing UI manager");
        
        let (ui_sender, ui_receiver) = mpsc::channel(100);
        
        let ui_manager = Self {
            config: config.clone(),
            ui_state: Arc::new(Mutex::new(UIState {
                is_emergency_mode: false,
                current_emergency: None,
                current_instruction: None,
                instruction_step: 0,
                total_steps: 0,
                is_listening: false,
                volume_level: 0.5,
                nearby_devices_count: 0,
                battery_level: 0.85,
                location: Some((37.7749, -122.4194)), // Default location
            })),
            ui_sender,
            #[cfg(feature = "desktop")]
            egui_context: None,
        };
        
        // Start UI processing loop
        let config = config.clone();
        let ui_state = ui_manager.ui_state.clone();
        
        tokio::spawn(async move {
            if let Err(e) = Self::ui_processing_loop(
                config,
                ui_receiver,
                ui_state,
            ).await {
                error!("UI processing loop error: {}", e);
            }
        });
        
        info!("UI manager initialized successfully");
        Ok(ui_manager)
    }
    
    /// Show emergency interface
    pub async fn show_emergency(&self, emergency_type: EmergencyType) -> AppResult<()> {
        info!("Showing emergency interface for: {:?}", emergency_type);
        
        if let Err(e) = self.ui_sender.send(UICommand::ShowEmergency(emergency_type)).await {
            return Err(AppError::UI(format!("Failed to send show emergency command: {}", e)));
        }
        
        info!("Emergency interface command sent");
        Ok(())
    }
    
    /// Show instruction
    pub async fn show_instruction(&self, instruction: EmergencyInstruction) -> AppResult<()> {
        info!("Showing instruction: {}", instruction.title);
        
        if let Err(e) = self.ui_sender.send(UICommand::ShowInstruction(instruction)).await {
            return Err(AppError::UI(format!("Failed to send show instruction command: {}", e)));
        }
        
        info!("Instruction display command sent");
        Ok(())
    }
    
    /// Update progress
    pub async fn update_progress(&self, current_step: u32, total_steps: u32) -> AppResult<()> {
        debug!("Updating progress: {}/{}", current_step, total_steps);
        
        if let Err(e) = self.ui_sender.send(UICommand::UpdateProgress(current_step, total_steps)).await {
            return Err(AppError::UI(format!("Failed to send update progress command: {}", e)));
        }
        
        Ok(())
    }
    
    /// Set listening status
    pub async fn set_listening(&self, is_listening: bool) -> AppResult<()> {
        debug!("Setting listening status: {}", is_listening);
        
        if let Err(e) = self.ui_sender.send(UICommand::SetListening(is_listening)).await {
            return Err(AppError::UI(format!("Failed to send set listening command: {}", e)));
        }
        
        Ok(())
    }
    
    /// Set volume level
    pub async fn set_volume(&self, volume: f32) -> AppResult<()> {
        debug!("Setting volume level: {}", volume);
        
        if let Err(e) = self.ui_sender.send(UICommand::SetVolume(volume)).await {
            return Err(AppError::UI(format!("Failed to send set volume command: {}", e)));
        }
        
        Ok(())
    }
    
    /// Update nearby devices count
    pub async fn update_nearby_devices(&self, count: u32) -> AppResult<()> {
        debug!("Updating nearby devices count: {}", count);
        
        if let Err(e) = self.ui_sender.send(UICommand::UpdateNearbyDevices(count)).await {
            return Err(AppError::UI(format!("Failed to send update nearby devices command: {}", e)));
        }
        
        Ok(())
    }
    
    /// Update battery level
    pub async fn update_battery(&self, battery_level: f32) -> AppResult<()> {
        debug!("Updating battery level: {}", battery_level);
        
        if let Err(e) = self.ui_sender.send(UICommand::UpdateBattery(battery_level)).await {
            return Err(AppError::UI(format!("Failed to send update battery command: {}", e)));
        }
        
        Ok(())
    }
    
    /// Update location
    pub async fn update_location(&self, location: Option<(f64, f64)>) -> AppResult<()> {
        debug!("Updating location: {:?}", location);
        
        if let Err(e) = self.ui_sender.send(UICommand::UpdateLocation(location)).await {
            return Err(AppError::UI(format!("Failed to send update location command: {}", e)));
        }
        
        Ok(())
    }
    
    /// Hide emergency interface
    pub async fn hide_emergency(&self) -> AppResult<()> {
        info!("Hiding emergency interface");
        
        if let Err(e) = self.ui_sender.send(UICommand::HideEmergency).await {
            return Err(AppError::UI(format!("Failed to send hide emergency command: {}", e)));
        }
        
        info!("Hide emergency command sent");
        Ok(())
    }
    
    /// Show settings interface
    pub async fn show_settings(&self) -> AppResult<()> {
        info!("Showing settings interface");
        
        if let Err(e) = self.ui_sender.send(UICommand::ShowSettings).await {
            return Err(AppError::UI(format!("Failed to send show settings command: {}", e)));
        }
        
        info!("Show settings command sent");
        Ok(())
    }
    
    /// Show statistics interface
    pub async fn show_statistics(&self) -> AppResult<()> {
        info!("Showing statistics interface");
        
        if let Err(e) = self.ui_sender.send(UICommand::ShowStatistics).await {
            return Err(AppError::UI(format!("Failed to send show statistics command: {}", e)));
        }
        
        info!("Show statistics command sent");
        Ok(())
    }
    
    /// Get current UI state
    pub fn get_ui_state(&self) -> UIState {
        self.ui_state.lock().unwrap().clone()
    }
    
    /// Check if in emergency mode
    pub fn is_in_emergency_mode(&self) -> bool {
        self.ui_state.lock().unwrap().is_emergency_mode
    }
    
    /// Main UI processing loop
    async fn ui_processing_loop(
        config: UIConfig,
        mut receiver: mpsc::Receiver<UICommand>,
        ui_state: Arc<Mutex<UIState>>,
    ) -> AppResult<()> {
        info!("UI processing loop started");
        
        while let Some(command) = receiver.recv().await {
            match command {
                UICommand::ShowEmergency(emergency_type) => {
                    info!("Processing show emergency: {:?}", emergency_type);
                    
                    let mut state = ui_state.lock().unwrap();
                    state.is_emergency_mode = true;
                    state.current_emergency = Some(emergency_type);
                    state.instruction_step = 0;
                    state.total_steps = 0;
                }
                
                UICommand::ShowInstruction(instruction) => {
                    info!("Processing show instruction: {}", instruction.title);
                    
                    let mut state = ui_state.lock().unwrap();
                    state.current_instruction = Some(instruction);
                }
                
                UICommand::UpdateProgress(current_step, total_steps) => {
                    debug!("Processing update progress: {}/{}", current_step, total_steps);
                    
                    let mut state = ui_state.lock().unwrap();
                    state.instruction_step = current_step;
                    state.total_steps = total_steps;
                }
                
                UICommand::SetListening(is_listening) => {
                    debug!("Processing set listening: {}", is_listening);
                    
                    let mut state = ui_state.lock().unwrap();
                    state.is_listening = is_listening;
                }
                
                UICommand::SetVolume(volume) => {
                    debug!("Processing set volume: {}", volume);
                    
                    let mut state = ui_state.lock().unwrap();
                    state.volume_level = volume;
                }
                
                UICommand::UpdateNearbyDevices(count) => {
                    debug!("Processing update nearby devices: {}", count);
                    
                    let mut state = ui_state.lock().unwrap();
                    state.nearby_devices_count = count;
                }
                
                UICommand::UpdateBattery(battery_level) => {
                    debug!("Processing update battery: {}", battery_level);
                    
                    let mut state = ui_state.lock().unwrap();
                    state.battery_level = battery_level;
                }
                
                UICommand::UpdateLocation(location) => {
                    debug!("Processing update location: {:?}", location);
                    
                    let mut state = ui_state.lock().unwrap();
                    state.location = location;
                }
                
                UICommand::HideEmergency => {
                    info!("Processing hide emergency");
                    
                    let mut state = ui_state.lock().unwrap();
                    state.is_emergency_mode = false;
                    state.current_emergency = None;
                    state.current_instruction = None;
                    state.instruction_step = 0;
                    state.total_steps = 0;
                }
                
                UICommand::ShowSettings => {
                    info!("Processing show settings");
                    
                    // In a real implementation, this would show the settings interface
                }
                
                UICommand::ShowStatistics => {
                    info!("Processing show statistics");
                    
                    // In a real implementation, this would show the statistics interface
                }
            }
        }
        
        info!("UI processing loop stopped");
        Ok(())
    }
    
    /// Render desktop UI using egui
    #[cfg(feature = "desktop")]
    pub fn render_desktop_ui(&self, ctx: &eframe::egui::Context) -> AppResult<()> {
        let state = self.get_ui_state();
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Crisis Companion");
            
            if state.is_emergency_mode {
                self.render_emergency_ui(ui, &state);
            } else {
                self.render_main_ui(ui, &state);
            }
        });
        
        Ok(())
    }
    
    /// Render emergency UI
    #[cfg(feature = "desktop")]
    fn render_emergency_ui(&self, ui: &mut egui::Ui, state: &UIState) {
        ui.heading("EMERGENCY MODE");
        
        if let Some(emergency_type) = &state.current_emergency {
            ui.label(format!("Emergency Type: {:?}", emergency_type));
        }
        
        if let Some(instruction) = &state.current_instruction {
            ui.label(format!("Step {}: {}", state.instruction_step, instruction.title));
            ui.label(&instruction.description);
            
            if state.total_steps > 0 {
                let progress = state.instruction_step as f32 / state.total_steps as f32;
                ui.add(egui::widgets::ProgressBar::new(progress).text(format!("{}/{}", state.instruction_step, state.total_steps)));
            }
        }
        
        ui.label(format!("Volume: {:.0}%", state.volume_level * 100.0));
        ui.label(format!("Battery: {:.0}%", state.battery_level * 100.0));
        ui.label(format!("Nearby Devices: {}", state.nearby_devices_count));
        
        if ui.button("End Emergency").clicked() {
            // This would trigger the end emergency command
        }
    }
    
    /// Render main UI
    #[cfg(feature = "desktop")]
    fn render_main_ui(&self, ui: &mut egui::Ui, state: &UIState) {
        ui.label("Voice-activated emergency response system");
        
        ui.label(format!("Listening: {}", if state.is_listening { "Yes" } else { "No" }));
        ui.label(format!("Battery: {:.0}%", state.battery_level * 100.0));
        ui.label(format!("Nearby Devices: {}", state.nearby_devices_count));
        
        if let Some(location) = state.location {
            ui.label(format!("Location: {:.4}, {:.4}", location.0, location.1));
        }
        
        ui.horizontal(|ui| {
            if ui.button("Settings").clicked() {
                // This would trigger the show settings command
            }
            
            if ui.button("Statistics").clicked() {
                // This would trigger the show statistics command
            }
        });
    }
    
    /// Test UI functionality
    pub async fn test_ui(&self) -> AppResult<()> {
        info!("Testing UI functionality");
        
        // Test emergency display
        self.show_emergency(EmergencyType::Drowning).await?;
        assert!(self.is_in_emergency_mode());
        
        // Test instruction display
        let instruction = EmergencyInstruction {
            id: Uuid::new_v4(),
            emergency_type: EmergencyType::Drowning,
            step_number: 1,
            title: "Test Instruction".to_string(),
            description: "This is a test instruction".to_string(),
            audio_file: None,
            estimated_duration_seconds: 30,
        };
        self.show_instruction(instruction).await?;
        
        // Test progress update
        self.update_progress(1, 5).await?;
        
        // Test listening status
        self.set_listening(true).await?;
        
        // Test volume update
        self.set_volume(0.8).await?;
        
        // Test nearby devices update
        self.update_nearby_devices(3).await?;
        
        // Test battery update
        self.update_battery(0.75).await?;
        
        // Test location update
        self.update_location(Some((40.7128, -74.0060))).await?;
        
        // Test hide emergency
        self.hide_emergency().await?;
        assert!(!self.is_in_emergency_mode());
        
        info!("UI functionality test completed successfully");
        Ok(())
    }
}

impl Drop for AppUI {
    fn drop(&mut self) {
        info!("UI manager shutting down");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::UIConfig;
    
    #[tokio::test]
    async fn test_ui_creation() {
        let config = UIConfig::default();
        let ui = AppUI::new(&config);
        assert!(ui.is_ok());
    }
    
    #[tokio::test]
    async fn test_emergency_display() {
        let config = UIConfig::default();
        let ui = AppUI::new(&config).unwrap();
        
        // Initially not in emergency mode
        assert!(!ui.is_in_emergency_mode());
        
        // Show emergency
        ui.show_emergency(EmergencyType::Drowning).await.unwrap();
        assert!(ui.is_in_emergency_mode());
        
        // Hide emergency
        ui.hide_emergency().await.unwrap();
        assert!(!ui.is_in_emergency_mode());
    }
    
    #[tokio::test]
    async fn test_ui_updates() {
        let config = UIConfig::default();
        let ui = AppUI::new(&config).unwrap();
        
        // Test various UI updates
        ui.set_listening(true).await.unwrap();
        ui.set_volume(0.9).await.unwrap();
        ui.update_nearby_devices(5).await.unwrap();
        ui.update_battery(0.6).await.unwrap();
        ui.update_location(Some((34.0522, -118.2437))).await.unwrap();
        
        // Verify state updates
        let state = ui.get_ui_state();
        assert!(state.is_listening);
        assert_eq!(state.volume_level, 0.9);
        assert_eq!(state.nearby_devices_count, 5);
        assert_eq!(state.battery_level, 0.6);
        assert_eq!(state.location, Some((34.0522, -118.2437)));
    }
    
    #[tokio::test]
    async fn test_instruction_display() {
        let config = UIConfig::default();
        let ui = AppUI::new(&config).unwrap();
        
        let instruction = EmergencyInstruction {
            id: Uuid::new_v4(),
            emergency_type: EmergencyType::Fire,
            step_number: 2,
            title: "Test Title".to_string(),
            description: "Test Description".to_string(),
            audio_file: None,
            estimated_duration_seconds: 45,
        };
        
        ui.show_instruction(instruction).await.unwrap();
        
        // Test progress update
        ui.update_progress(2, 5).await.unwrap();
        
        let state = ui.get_ui_state();
        assert_eq!(state.instruction_step, 2);
        assert_eq!(state.total_steps, 5);
    }
} 