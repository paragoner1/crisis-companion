//! Solana SOS - Main Application
//! 
//! Hybrid offline/online emergency response system with context-aware guidance.

use crate::{
    config::AppConfig,
    voice::VoiceTrigger,
    audio::AudioManager,
    database::EmergencyDatabase,
    coordination::DeviceCoordinator,
    emergency::EmergencyHandler,
    ui::AppUI,
    blockchain::BlockchainManager,
    context_analysis::context_analysis::ContextAnalyzer,
    types::{EmergencyType, EmergencyStage, ContextClues, UserRole, ConnectivityMode, ContextAnalysisResult, GuidanceMode, GuidanceResult},
    error::AppError,
};
use tracing::{info, warn};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main application for Solana SOS
pub struct CrisisCompanionApp {
    config: AppConfig,
    voice_trigger: VoiceTrigger,
    audio_manager: AudioManager,
    database: EmergencyDatabase,
    coordinator: DeviceCoordinator,
    emergency_handler: EmergencyHandler,
    ui: AppUI,
    blockchain_manager: BlockchainManager,
    context_analyzer: ContextAnalyzer,
    connectivity_mode: Arc<RwLock<ConnectivityMode>>,
    is_active: Arc<RwLock<bool>>,
}

impl CrisisCompanionApp {
    /// Create a new Solana SOS application
    pub async fn new(config_path: &str) -> Result<Self, AppError> {
        info!("🚨 Initializing Solana SOS - The phone you can't live without");
        
        // Load configuration
        let config = AppConfig::load(config_path)?;
        info!("✅ Configuration loaded from {}", config_path);

        // Initialize core components
        let voice_trigger = VoiceTrigger::new(&config.voice)?;
        let audio_manager = AudioManager::new(&config.audio)?;
        let database = EmergencyDatabase::new(&config.database)?;
        let coordinator = DeviceCoordinator::new(&config.coordination)?;
        let emergency_handler = EmergencyHandler::new(&config.emergency)?;
        let ui = AppUI::new(&config.ui)?;
        let blockchain_manager = BlockchainManager::new(&config.blockchain)?;
        let context_analyzer = ContextAnalyzer::new();

        // Determine initial connectivity mode
        let connectivity_mode = Arc::new(RwLock::new(
            Self::determine_connectivity_mode(&config).await
        ));

        let is_active = Arc::new(RwLock::new(true));

        info!("✅ All components initialized successfully");
        info!("🎯 Hybrid Architecture: {:?}", *connectivity_mode.read().await);

        Ok(Self {
            config,
            voice_trigger,
            audio_manager,
            database,
            coordinator,
            emergency_handler,
            ui,
            blockchain_manager,
            context_analyzer,
            connectivity_mode,
            is_active,
        })
    }

    /// Determine the appropriate connectivity mode based on configuration and environment
    async fn determine_connectivity_mode(config: &AppConfig) -> ConnectivityMode {
        // Check user preference first
        if let Some(preference) = &config.connectivity.user_preference {
            match preference.as_str() {
                "offline" => return ConnectivityMode::Offline,
                "online" => return ConnectivityMode::Online,
                "hybrid" => return ConnectivityMode::Hybrid,
                _ => {}
            }
        }

        // Check network connectivity
        if Self::check_network_connectivity().await {
            ConnectivityMode::Hybrid
        } else {
            ConnectivityMode::Offline
        }
    }

    /// Check if network connectivity is available
    async fn check_network_connectivity() -> bool {
        // Simple connectivity check - in production, this would be more sophisticated
        match tokio::time::timeout(Duration::from_secs(3), async {
            // Attempt to reach a reliable endpoint
            reqwest::get("https://httpbin.org/status/200").await.is_ok()
        }).await {
            Ok(true) => {
                info!("✅ Network connectivity available");
                true
            }
            _ => {
                warn!("⚠️ No network connectivity detected, using offline mode");
                false
            }
        }
    }

    /// Start the emergency monitoring system
    pub async fn start_monitoring(&self) -> Result<(), AppError> {
        info!("🚨 Starting Solana SOS emergency monitoring");
        info!("🎯 Mode: {:?}", *self.connectivity_mode.read().await);

        let is_active = Arc::clone(&self.is_active);
        let connectivity_mode = Arc::clone(&self.connectivity_mode);

        // Start background monitoring task
        tokio::spawn(async move {
            while *is_active.read().await {
                // Monitor for connectivity changes
                if let Ok(mode) = Self::check_connectivity_changes(&connectivity_mode).await {
                    let current_mode = connectivity_mode.read().await.clone();
                    if mode != current_mode {
                        *connectivity_mode.write().await = mode.clone();
                        info!("🔄 Connectivity mode changed to: {:?}", mode);
                    }
                }

                sleep(Duration::from_secs(30)).await;
            }
        });

        info!("✅ Emergency monitoring started successfully");
        Ok(())
    }

    /// Check for connectivity changes and update mode accordingly
    async fn check_connectivity_changes(
        connectivity_mode: &Arc<RwLock<ConnectivityMode>>
    ) -> Result<ConnectivityMode, AppError> {
        let current_mode = connectivity_mode.read().await.clone();
        
        match current_mode {
            ConnectivityMode::Offline => {
                // Check if we should switch to hybrid
                if Self::check_network_connectivity().await {
                    Ok(ConnectivityMode::Hybrid)
                } else {
                    Ok(ConnectivityMode::Offline)
                }
            }
            ConnectivityMode::Online => {
                // Check if we should switch to offline
                if !Self::check_network_connectivity().await {
                    Ok(ConnectivityMode::Offline)
                } else {
                    Ok(ConnectivityMode::Online)
                }
            }
            ConnectivityMode::Hybrid => {
                // Stay in hybrid mode, but could optimize based on conditions
                Ok(ConnectivityMode::Hybrid)
            }
        }
    }

    /// Handle emergency voice trigger with context-aware guidance
    pub async fn handle_emergency_trigger(
        &self,
        phrase: &str,
        emergency_type: EmergencyType,
        user_role: UserRole,
    ) -> Result<ContextAnalysisResult, AppError> {
        info!("🚨 Emergency trigger detected: {:?}", emergency_type);
        info!("📝 Phrase: '{}'", phrase);
        info!("👤 User role: {:?}", user_role);

        // Create context clues for analysis
        let context_clues = self.create_context_clues(phrase, emergency_type.clone(), user_role).await?;

        // Analyze emergency context
        let analysis = self.context_analyzer.analyze_emergency(
            emergency_type.clone(),
            &context_clues,
        ).await?;

        info!("🎯 Context analysis complete:");
        info!("   Stage: {:?}", analysis.stage);
        info!("   Confidence: {:.2}", analysis.confidence);
        info!("   Skip basic steps: {}", analysis.guidance.skip_basic_steps);

        // Determine guidance mode based on emergency type and connectivity
        let guidance_mode = match emergency_type {
            EmergencyType::Drowning | EmergencyType::Choking => {
                GuidanceMode::Offline // Simple, direct interventions
            }
            EmergencyType::HeartAttack | EmergencyType::Bleeding => {
                GuidanceMode::Hybrid // May need online enhancement
            }
            EmergencyType::Stroke | EmergencyType::Poisoning | EmergencyType::SevereBurns | EmergencyType::DiabeticEmergency => {
                GuidanceMode::Hybrid // Critical time-sensitive emergencies
            }
            EmergencyType::Unconscious | EmergencyType::AllergicReaction | EmergencyType::Trauma | EmergencyType::Seizure => {
                GuidanceMode::Hybrid // May need online enhancement
            }
        };

        info!("🎯 Guidance mode: {:?}", guidance_mode);

        // Generate appropriate guidance
        let guidance_result = match guidance_mode {
            GuidanceMode::Offline => {
                self.generate_offline_guidance(&analysis).await?
            }
            GuidanceMode::Online => {
                self.generate_online_guidance(&analysis).await?
            }
            GuidanceMode::Hybrid => {
                self.generate_hybrid_guidance(&analysis).await?
            }
        };

        // Execute emergency response
        self.execute_emergency_response(&analysis, &guidance_result).await?;

        Ok(ContextAnalysisResult {
            analysis: analysis.clone(),
            stage_detection_confidence: analysis.confidence,
            guidance_appropriateness: guidance_result.appropriateness,
            time_saved_seconds: guidance_result.time_saved,
            skipped_steps: guidance_result.skipped_steps,
        })
    }

    /// Create context clues for emergency analysis
    async fn create_context_clues(
        &self,
        phrase: &str,
        emergency_type: EmergencyType,
        user_role: UserRole,
    ) -> Result<ContextClues, AppError> {
        // Get current location context
        let location_context = self.get_location_context().await?;

        // Get actions already taken
        let actions_taken = self.get_actions_taken().await?;

        // Get victim status if available
        let victim_status = self.get_victim_status().await?;

        // Get environmental context
        let environment = self.get_environment_context().await?;

        Ok(ContextClues {
            user_phrase: phrase.to_string(),
            location_context,
            time_elapsed: None, // Will be set by emergency handler
            victim_status,
            environment,
            actions_taken,
        })
    }

    /// Determine the appropriate guidance mode
    async fn determine_guidance_mode(
        &self,
        connectivity_mode: ConnectivityMode,
        emergency_type: EmergencyType,
        analysis: &crate::types::EmergencyAnalysis,
    ) -> GuidanceMode {
        match connectivity_mode {
            ConnectivityMode::Offline => GuidanceMode::Offline,
            ConnectivityMode::Online => GuidanceMode::Online,
            ConnectivityMode::Hybrid => {
                // Smart routing logic
                match emergency_type {
                    EmergencyType::Drowning | EmergencyType::Choking => {
                        // Critical emergencies - use offline for speed
                        GuidanceMode::Offline
                    }
                    EmergencyType::HeartAttack | EmergencyType::Bleeding => {
                        // Complex scenarios - use online for intelligence
                        if analysis.confidence < 0.8 {
                            GuidanceMode::Online
                        } else {
                            GuidanceMode::Offline
                        }
                    }
                    _ => {
                        // Default to hybrid for other emergencies
                        GuidanceMode::Hybrid
                    }
                }
            }
        }
    }

    /// Generate offline guidance
    async fn generate_offline_guidance(
        &self,
        analysis: &crate::types::EmergencyAnalysis,
    ) -> Result<GuidanceResult, AppError> {
        info!("📱 Generating offline guidance for stage: {:?}", analysis.stage);

        let guidance = &analysis.guidance;
        let time_saved = if guidance.skip_basic_steps { 45 } else { 0 };

        Ok(GuidanceResult {
            mode: GuidanceMode::Offline,
            instructions: guidance.instructions.clone(),
            priority_actions: guidance.priority_actions.clone(),
            appropriateness: analysis.confidence,
            time_saved,
            skipped_steps: if guidance.skip_basic_steps {
                vec![
                    "Scene assessment".to_string(),
                    "Basic safety instructions".to_string(),
                    "Initial rescue preparation".to_string(),
                ]
            } else {
                vec![]
            },
        })
    }

    /// Generate online guidance
    async fn generate_online_guidance(
        &self,
        analysis: &crate::types::EmergencyAnalysis,
    ) -> Result<GuidanceResult, AppError> {
        info!("🤖 Generating online AI guidance for stage: {:?}", analysis.stage);

        // In Phase 2, this would integrate with LLM APIs
        // For now, we'll enhance the offline guidance with additional context
        let mut enhanced_instructions = analysis.guidance.instructions.clone();
        enhanced_instructions.push("AI: Consider calling emergency services if not already done".to_string());
        enhanced_instructions.push("AI: Monitor victim closely for any changes".to_string());

        Ok(GuidanceResult {
            mode: GuidanceMode::Online,
            instructions: enhanced_instructions,
            priority_actions: analysis.guidance.priority_actions.clone(),
            appropriateness: analysis.confidence + 0.1, // AI enhancement
            time_saved: if analysis.guidance.skip_basic_steps { 45 } else { 0 },
            skipped_steps: if analysis.guidance.skip_basic_steps {
                vec![
                    "Scene assessment".to_string(),
                    "Basic safety instructions".to_string(),
                    "Initial rescue preparation".to_string(),
                ]
            } else {
                vec![]
            },
        })
    }

    /// Generate hybrid guidance
    async fn generate_hybrid_guidance(
        &self,
        analysis: &crate::types::EmergencyAnalysis,
    ) -> Result<GuidanceResult, AppError> {
        info!("🔄 Generating hybrid guidance for stage: {:?}", analysis.stage);

        // Start with offline guidance
        let offline_result = self.generate_offline_guidance(analysis).await?;

        // Enhance with online capabilities if available
        if Self::check_network_connectivity().await {
            let online_result = self.generate_online_guidance(analysis).await?;
            
            // Combine both approaches
            let mut combined_instructions = offline_result.instructions;
            combined_instructions.extend(online_result.instructions);

            Ok(GuidanceResult {
                mode: GuidanceMode::Hybrid,
                instructions: combined_instructions,
                priority_actions: offline_result.priority_actions,
                appropriateness: (offline_result.appropriateness + online_result.appropriateness) / 2.0,
                time_saved: offline_result.time_saved,
                skipped_steps: offline_result.skipped_steps,
            })
        } else {
            // Fall back to offline if no connectivity
            Ok(offline_result)
        }
    }

    /// Execute emergency response
    async fn execute_emergency_response(
        &self,
        analysis: &crate::types::EmergencyAnalysis,
        guidance_result: &GuidanceResult,
    ) -> Result<(), AppError> {
        info!("🚨 Executing emergency response");

        // Set emergency volume
        self.audio_manager.set_emergency_volume().await?;

        // Display emergency UI - convert EmergencyStage to EmergencyType
        let emergency_type = match analysis.stage {
            EmergencyStage::InitialDetection => EmergencyType::Drowning, // Default for demo
            EmergencyStage::VictimExtracted => EmergencyType::Drowning,
            EmergencyStage::Unconscious => EmergencyType::Drowning,
            EmergencyStage::ConsciousButInjured => EmergencyType::Bleeding,
            EmergencyStage::BreathingButUnresponsive => EmergencyType::Drowning,
            EmergencyStage::ServicesEnRoute => EmergencyType::Drowning,
            EmergencyStage::PostEmergency => EmergencyType::Drowning,
        };
        
        let emergency_type_clone = emergency_type.clone();
        self.ui.show_emergency(emergency_type).await?;

        // Provide guidance instructions
        for instruction in &guidance_result.instructions {
            info!("📱 Displaying instruction: {}", instruction);
            // In a real implementation, this would display the instruction on the UI
            sleep(Duration::from_millis(2000)).await; // 2 second delay between instructions
        }

        // Store emergency data on blockchain
        self.blockchain_manager.store_audio_hash(
            &format!("emergency_{}", uuid::Uuid::new_v4()),
            &emergency_type_clone,
        ).await?;

        // Coordinate with nearby devices
        self.coordinator.coordinate_emergency_response(emergency_type_clone).await?;

        info!("✅ Emergency response executed successfully");
        Ok(())
    }

    /// Get current location context
    async fn get_location_context(&self) -> Result<Option<crate::types::LocationContext>, AppError> {
        // In production, this would use GPS or other location services
        Ok(Some(crate::types::LocationContext {
            location_type: crate::types::LocationType::Home,
            coordinates: Some((34.0522, -118.2437)), // Default coordinates
            nearby_landmarks: vec!["Home".to_string()],
        }))
    }

    /// Get actions already taken
    async fn get_actions_taken(&self) -> Result<Vec<crate::types::EmergencyAction>, AppError> {
        // In production, this would track actual actions taken
        Ok(vec![])
    }

    /// Get victim status
    async fn get_victim_status(&self) -> Result<Option<crate::types::VictimStatus>, AppError> {
        // In production, this would be determined through user input or sensors
        Ok(None)
    }

    /// Get environmental context
    async fn get_environment_context(&self) -> Result<crate::types::EnvironmentContext, AppError> {
        // In production, this would use sensors and environmental data
        Ok(crate::types::EnvironmentContext {
            weather_conditions: crate::types::WeatherConditions::Clear,
            crowd_present: false,
            professional_help_available: false,
            emergency_equipment_available: false,
            accessibility_issues: vec![],
        })
    }

    /// Stop the application
    pub async fn stop(&self) -> Result<(), AppError> {
        info!("🛑 Stopping Solana SOS");
        
        *self.is_active.write().await = false;
        
        // Cleanup resources
        self.audio_manager.set_volume(self.config.audio.default_volume).await?;
        self.ui.hide_emergency().await?;
        
        info!("✅ Solana SOS stopped successfully");
        Ok(())
    }

    /// Get current connectivity mode
    pub async fn get_connectivity_mode(&self) -> ConnectivityMode {
        self.connectivity_mode.read().await.clone()
    }

    /// Set connectivity mode
    pub async fn set_connectivity_mode(&self, mode: ConnectivityMode) -> Result<(), AppError> {
        info!("🔄 Setting connectivity mode to: {:?}", mode);
        *self.connectivity_mode.write().await = mode;
        Ok(())
    }
} 