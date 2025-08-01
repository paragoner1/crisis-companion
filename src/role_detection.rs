//! Role Detection Module for Solana SOS
//! 
//! This module handles determining whether the user is a victim (needing help)
//! or a bystander (helping someone else) during emergency situations.
//! 
//! # Features
//! - Multi-modal confirmation (UI tap + voice fallback)
//! - AI context inference from phrases and sensor data
//! - User profile-based role detection
//! - Adaptive learning from emergency outcomes
//! - Sensor fusion for improved accuracy

use crate::types::{
    UserRole, RoleDetectionMethod, RoleDetectionResult, RoleContext, SensorData,
    DeviceMovement, LocationContext, LocationType, AudioEnvironment, UserProfile,
    EmergencyType, VoiceModel, MedicalInfo,
};
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{info, warn, error};

/// Configuration for role detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDetectionConfig {
    /// Timeout for UI confirmation (milliseconds)
    pub ui_timeout_ms: u64,
    /// Timeout for voice confirmation (milliseconds)
    pub voice_timeout_ms: u64,
    /// Confidence threshold for AI inference
    pub ai_confidence_threshold: f32,
    /// Enable sensor fusion
    pub enable_sensor_fusion: bool,
    /// Enable adaptive learning
    pub enable_adaptive_learning: bool,
    /// User profile override allowed
    pub allow_profile_override: bool,
}

impl Default for RoleDetectionConfig {
    fn default() -> Self {
        Self {
            ui_timeout_ms: 3000, // 3 seconds
            voice_timeout_ms: 5000, // 5 seconds
            ai_confidence_threshold: 0.8,
            enable_sensor_fusion: true,
            enable_adaptive_learning: true,
            allow_profile_override: true,
        }
    }
}

/// Main role detection system
pub struct RoleDetector {
    config: RoleDetectionConfig,
    user_profile: Option<UserProfile>,
    ai_inference: AIInference,
    sensor_fusion: SensorFusion,
    adaptive_learning: AdaptiveLearning,
}

impl RoleDetector {
    /// Create a new role detector
    pub fn new(config: RoleDetectionConfig) -> Self {
        Self {
            config,
            user_profile: None,
            ai_inference: AIInference::new(),
            sensor_fusion: SensorFusion::new(),
            adaptive_learning: AdaptiveLearning::new(),
        }
    }

    /// Set user profile for role detection
    pub fn set_user_profile(&mut self, profile: UserProfile) {
        self.user_profile = Some(profile);
    }

    /// Detect user role using hybrid approach
    pub async fn detect_role(
        &mut self,
        emergency_context: &EmergencyContext,
    ) -> Result<RoleDetectionResult, AppError> {
        let start_time = Instant::now();

        // 1. Check user profile first (fastest)
        if let Some(profile) = &self.user_profile {
            if let Some(role) = self.check_user_profile(profile, emergency_context) {
                return Ok(RoleDetectionResult {
                    role,
                    method: RoleDetectionMethod::UserProfile,
                    confidence: 0.9,
                    detection_time_ms: start_time.elapsed().as_millis() as u64,
                    context_data: Some(RoleContext {
                        phrase_detected: emergency_context.phrase.clone(),
                        sensor_data: emergency_context.sensor_data.clone(),
                        user_profile: Some(profile.clone()),
                        emergency_type: emergency_context.emergency_type.clone(),
                    }),
                });
            }
        }

        // 2. AI context inference
        let ai_result = self.ai_inference.infer_role(emergency_context).await?;
        if ai_result.confidence >= self.config.ai_confidence_threshold {
            return Ok(RoleDetectionResult {
                role: ai_result.role,
                method: RoleDetectionMethod::AIInference,
                confidence: ai_result.confidence,
                detection_time_ms: start_time.elapsed().as_millis() as u64,
                context_data: Some(emergency_context.to_role_context()),
            });
        }

        // 3. Multi-modal confirmation (UI + voice fallback)
        let confirmation_result = self.multi_modal_confirmation(emergency_context).await?;
        
        // 4. Learn from this interaction
        if self.config.enable_adaptive_learning {
            self.adaptive_learning.learn_from_detection(
                emergency_context,
                &confirmation_result,
            ).await;
        }

        Ok(confirmation_result)
    }

    /// Check if user profile provides role information
    fn check_user_profile(
        &self,
        profile: &UserProfile,
        context: &EmergencyContext,
    ) -> Option<UserRole> {
        // Caregivers are typically bystanders
        if profile.is_caregiver {
            return Some(UserRole::Bystander);
        }

        // Check if profile override is allowed
        if self.config.allow_profile_override {
            return Some(profile.default_role.clone());
        }

        None
    }

    /// Multi-modal confirmation with UI tap and voice fallback
    async fn multi_modal_confirmation(
        &self,
        context: &EmergencyContext,
    ) -> Result<RoleDetectionResult, AppError> {
        let start_time = Instant::now();

        // Show UI confirmation dialog
        let ui_result = self.show_ui_confirmation(context).await;
        
        match ui_result {
            Ok(role) => {
                return Ok(RoleDetectionResult {
                    role,
                    method: RoleDetectionMethod::UITap,
                    confidence: 0.95,
                    detection_time_ms: start_time.elapsed().as_millis() as u64,
                    context_data: Some(context.to_role_context()),
                });
            }
            Err(_) => {
                // Fallback to voice confirmation
                let voice_result = self.voice_confirmation(context).await?;
                return Ok(RoleDetectionResult {
                    role: voice_result.role,
                    method: RoleDetectionMethod::VoiceConfirmation,
                    confidence: voice_result.confidence,
                    detection_time_ms: start_time.elapsed().as_millis() as u64,
                    context_data: Some(context.to_role_context()),
                });
            }
        }
    }

    /// Show UI confirmation dialog
    async fn show_ui_confirmation(
        &self,
        context: &EmergencyContext,
    ) -> Result<UserRole, AppError> {
        // This would integrate with the Android UI
        // For now, simulate UI interaction
        info!("Showing UI confirmation dialog for emergency: {:?}", context.emergency_type);
        
        // Simulate UI timeout
        tokio::time::sleep(Duration::from_millis(self.config.ui_timeout_ms)).await;
        
        // In real implementation, this would wait for user tap
        // For demo purposes, return error to trigger voice fallback
        Err(AppError::ConfirmationTimeout)
    }

    /// Voice confirmation fallback
    async fn voice_confirmation(
        &self,
        context: &EmergencyContext,
    ) -> Result<RoleDetectionResult, AppError> {
        info!("Starting voice confirmation for emergency: {:?}", context.emergency_type);
        
        // Play voice prompt
        self.play_voice_prompt(context).await?;
        
        // Listen for response
        let response = self.listen_for_voice_response().await?;
        
        // Parse response
        let role = self.parse_voice_response(&response)?;
        
        Ok(RoleDetectionResult {
            role,
            method: RoleDetectionMethod::VoiceConfirmation,
            confidence: 0.85,
            detection_time_ms: 0, // Will be set by caller
            context_data: Some(context.to_role_context()),
        })
    }

    /// Play voice prompt for role confirmation
    async fn play_voice_prompt(&self, context: &EmergencyContext) -> Result<(), AppError> {
        let prompt = match context.emergency_type {
            EmergencyType::Drowning => "Are you in danger or helping someone? Say 'me' for danger, 'other' for helping.",
            EmergencyType::Choking => "Are you choking or helping someone? Say 'me' for choking, 'other' for helping.",
            _ => "Are you in danger or helping someone? Say 'me' for danger, 'other' for helping.",
        };
        
        info!("Playing voice prompt: {}", prompt);
        // This would integrate with audio system
        Ok(())
    }

    /// Listen for voice response
    async fn listen_for_voice_response(&self) -> Result<String, AppError> {
        // This would integrate with voice recognition system
        // For demo purposes, simulate response
        tokio::time::sleep(Duration::from_millis(2000)).await;
        
        // Simulate voice response
        Ok("other".to_string())
    }

    /// Parse voice response
    fn parse_voice_response(&self, response: &str) -> Result<UserRole, AppError> {
        match response.to_lowercase().trim() {
            "me" | "danger" | "help" | "emergency" => Ok(UserRole::Victim),
            "other" | "helping" | "someone" | "bystander" => Ok(UserRole::Bystander),
            _ => Err(AppError::InvalidVoiceResponse),
        }
    }
}

/// AI inference for role detection
pub struct AIInference {
    phrase_patterns: Vec<PhrasePattern>,
    sensor_weights: SensorWeights,
}

impl AIInference {
    pub fn new() -> Self {
        Self {
            phrase_patterns: Self::load_phrase_patterns(),
            sensor_weights: SensorWeights::default(),
        }
    }

    /// Infer role from context
    pub async fn infer_role(
        &self,
        context: &EmergencyContext,
    ) -> Result<RoleInferenceResult, AppError> {
        let mut confidence = 0.0;
        let mut role = UserRole::Unknown;

        // Analyze phrase patterns
        if let Some(phrase_result) = self.analyze_phrase(&context.phrase) {
            confidence += phrase_result.confidence * 0.6; // 60% weight
            role = phrase_result.role;
        }

        // Analyze sensor data
        if let Some(sensor_data) = &context.sensor_data {
            if let Some(sensor_result) = self.analyze_sensor_data(sensor_data) {
                confidence += sensor_result.confidence * 0.4; // 40% weight
                
                // If sensor data strongly suggests different role, adjust
                if sensor_result.confidence > 0.8 {
                    role = sensor_result.role;
                }
            }
        }

        Ok(RoleInferenceResult {
            role,
            confidence,
            method: "AI Inference".to_string(),
        })
    }

    /// Analyze phrase patterns for role inference
    fn analyze_phrase(&self, phrase: &str) -> Option<PhraseAnalysis> {
        let phrase_lower = phrase.to_lowercase();
        
        // Victim patterns
        if phrase_lower.contains("i'm") || phrase_lower.contains("i am") {
            return Some(PhraseAnalysis {
                role: UserRole::Victim,
                confidence: 0.9,
                pattern: "First person distress".to_string(),
            });
        }
        
        if phrase_lower.contains("help me") || phrase_lower.contains("save me") {
            return Some(PhraseAnalysis {
                role: UserRole::Victim,
                confidence: 0.95,
                pattern: "Direct help request".to_string(),
            });
        }
        
        // Bystander patterns
        if phrase_lower.contains("my child") || phrase_lower.contains("my baby") {
            return Some(PhraseAnalysis {
                role: UserRole::Bystander,
                confidence: 0.9,
                pattern: "Caregiver reference".to_string(),
            });
        }
        
        if phrase_lower.contains("someone") || phrase_lower.contains("call 911") {
            return Some(PhraseAnalysis {
                role: UserRole::Bystander,
                confidence: 0.8,
                pattern: "Third party emergency".to_string(),
            });
        }
        
        None
    }

    /// Analyze sensor data for role inference
    fn analyze_sensor_data(&self, sensor_data: &SensorData) -> Option<SensorAnalysis> {
        let mut confidence = 0.0;
        let mut role = UserRole::Unknown;

        // Movement analysis
        match sensor_data.device_movement {
            DeviceMovement::Stationary => {
                // Stationary device suggests bystander (helping someone)
                confidence += 0.7;
                role = UserRole::Bystander;
            }
            DeviceMovement::Swimming | DeviceMovement::Falling => {
                // Swimming/falling suggests victim
                confidence += 0.8;
                role = UserRole::Victim;
            }
            DeviceMovement::Running => {
                // Running could be either - lower confidence
                confidence += 0.5;
                role = UserRole::Unknown;
            }
            _ => {}
        }

        // Location context analysis
        if let Some(location) = &sensor_data.location_context {
            match location.location_type {
                LocationType::Beach | LocationType::Pool => {
                    // Water locations suggest drowning scenarios
                    confidence += 0.3;
                }
                LocationType::Hospital => {
                    // Hospital suggests medical emergency
                    confidence += 0.2;
                }
                _ => {}
            }
        }

        if confidence > 0.0 {
            Some(SensorAnalysis {
                role,
                confidence,
                analysis: "Sensor fusion analysis".to_string(),
            })
        } else {
            None
        }
    }

    /// Load phrase patterns for analysis
    fn load_phrase_patterns() -> Vec<PhrasePattern> {
        vec![
            // Victim patterns
            PhrasePattern {
                pattern: r"i'm drowning",
                role: UserRole::Victim,
                confidence: 0.95,
            },
            PhrasePattern {
                pattern: r"help me",
                role: UserRole::Victim,
                confidence: 0.9,
            },
            PhrasePattern {
                pattern: r"i'm choking",
                role: UserRole::Victim,
                confidence: 0.95,
            },
            PhrasePattern {
                pattern: r"save me",
                role: UserRole::Victim,
                confidence: 0.9,
            },
            
            // Bystander patterns
            PhrasePattern {
                pattern: r"my child",
                role: UserRole::Bystander,
                confidence: 0.9,
            },
            PhrasePattern {
                pattern: r"my baby",
                role: UserRole::Bystander,
                confidence: 0.9,
            },
            PhrasePattern {
                pattern: r"someone is",
                role: UserRole::Bystander,
                confidence: 0.8,
            },
            PhrasePattern {
                pattern: r"call 911",
                role: UserRole::Bystander,
                confidence: 0.7,
            },
        ]
    }
}

/// Sensor fusion for role detection
pub struct SensorFusion {
    movement_analyzer: MovementAnalyzer,
    location_analyzer: LocationAnalyzer,
    audio_analyzer: AudioAnalyzer,
}

impl SensorFusion {
    pub fn new() -> Self {
        Self {
            movement_analyzer: MovementAnalyzer::new(),
            location_analyzer: LocationAnalyzer::new(),
            audio_analyzer: AudioAnalyzer::new(),
        }
    }

    /// Analyze sensor data for role inference
    pub async fn analyze_sensors(
        &self,
        sensor_data: &SensorData,
    ) -> Result<SensorAnalysis, AppError> {
        let movement_result = self.movement_analyzer.analyze(&sensor_data.device_movement);
        let location_result = self.location_analyzer.analyze(&sensor_data.location_context);
        let audio_result = self.audio_analyzer.analyze(&sensor_data.audio_environment);

        // Combine sensor results
        let combined_confidence = (movement_result.confidence + 
                                 location_result.confidence + 
                                 audio_result.confidence) / 3.0;

        let role = self.determine_role_from_sensors(
            &movement_result,
            &location_result,
            &audio_result,
        );

        Ok(SensorAnalysis {
            role,
            confidence: combined_confidence,
            analysis: "Multi-sensor fusion".to_string(),
        })
    }

    /// Determine role from sensor analysis
    fn determine_role_from_sensors(
        &self,
        movement: &SensorAnalysis,
        location: &SensorAnalysis,
        audio: &SensorAnalysis,
    ) -> UserRole {
        // Weight the sensor results
        let movement_weight = 0.5;
        let location_weight = 0.3;
        let audio_weight = 0.2;

        let mut victim_score = 0.0;
        let mut bystander_score = 0.0;

        // Movement analysis
        match movement.role {
            UserRole::Victim => victim_score += movement.confidence * movement_weight,
            UserRole::Bystander => bystander_score += movement.confidence * movement_weight,
            _ => {}
        }

        // Location analysis
        match location.role {
            UserRole::Victim => victim_score += location.confidence * location_weight,
            UserRole::Bystander => bystander_score += location.confidence * location_weight,
            _ => {}
        }

        // Audio analysis
        match audio.role {
            UserRole::Victim => victim_score += audio.confidence * audio_weight,
            UserRole::Bystander => bystander_score += audio.confidence * audio_weight,
            _ => {}
        }

        // Determine final role
        if victim_score > bystander_score && victim_score > 0.5 {
            UserRole::Victim
        } else if bystander_score > victim_score && bystander_score > 0.5 {
            UserRole::Bystander
        } else {
            UserRole::Unknown
        }
    }
}

/// Adaptive learning system
pub struct AdaptiveLearning {
    learning_rate: f32,
    historical_data: Vec<LearningExample>,
}

impl AdaptiveLearning {
    pub fn new() -> Self {
        Self {
            learning_rate: 0.1,
            historical_data: Vec::new(),
        }
    }

    /// Learn from role detection outcome
    pub async fn learn_from_detection(
        &mut self,
        context: &EmergencyContext,
        result: &RoleDetectionResult,
    ) {
        let example = LearningExample {
            context: context.clone(),
            result: result.clone(),
            timestamp: chrono::Utc::now(),
        };

        self.historical_data.push(example);
        
        // Keep only recent examples (last 100)
        if self.historical_data.len() > 100 {
            self.historical_data.remove(0);
        }

        info!("Learned from role detection: {:?} -> {:?}", 
              context.emergency_type, result.role);
    }

    /// Get learning insights
    pub fn get_insights(&self) -> LearningInsights {
        let total_examples = self.historical_data.len();
        let victim_count = self.historical_data.iter()
            .filter(|ex| ex.result.role == UserRole::Victim)
            .count();
        let bystander_count = self.historical_data.iter()
            .filter(|ex| ex.result.role == UserRole::Bystander)
            .count();

        LearningInsights {
            total_examples,
            victim_percentage: victim_count as f32 / total_examples as f32,
            bystander_percentage: bystander_count as f32 / total_examples as f32,
            average_confidence: self.historical_data.iter()
                .map(|ex| ex.result.confidence)
                .sum::<f32>() / total_examples as f32,
        }
    }
}

// Supporting types and structures

/// Emergency context for role detection
#[derive(Debug, Clone)]
pub struct EmergencyContext {
    pub phrase: String,
    pub emergency_type: EmergencyType,
    pub sensor_data: Option<SensorData>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl EmergencyContext {
    pub fn new(phrase: String, emergency_type: EmergencyType) -> Self {
        Self {
            phrase,
            emergency_type,
            sensor_data: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn with_sensor_data(mut self, sensor_data: SensorData) -> Self {
        self.sensor_data = Some(sensor_data);
        self
    }

    pub fn to_role_context(&self) -> RoleContext {
        RoleContext {
            phrase_detected: self.phrase.clone(),
            sensor_data: self.sensor_data.clone(),
            user_profile: None, // Will be set by caller
            emergency_type: self.emergency_type.clone(),
        }
    }
}

/// AI inference result
#[derive(Debug, Clone)]
pub struct RoleInferenceResult {
    pub role: UserRole,
    pub confidence: f32,
    pub method: String,
}

/// Phrase pattern for AI analysis
#[derive(Debug, Clone)]
pub struct PhrasePattern {
    pub pattern: &'static str,
    pub role: UserRole,
    pub confidence: f32,
}

/// Phrase analysis result
#[derive(Debug, Clone)]
pub struct PhraseAnalysis {
    pub role: UserRole,
    pub confidence: f32,
    pub pattern: String,
}

/// Sensor analysis result
#[derive(Debug, Clone)]
pub struct SensorAnalysis {
    pub role: UserRole,
    pub confidence: f32,
    pub analysis: String,
}

/// Sensor weights for AI inference
#[derive(Debug, Clone)]
pub struct SensorWeights {
    pub movement_weight: f32,
    pub location_weight: f32,
    pub audio_weight: f32,
}

impl Default for SensorWeights {
    fn default() -> Self {
        Self {
            movement_weight: 0.5,
            location_weight: 0.3,
            audio_weight: 0.2,
        }
    }
}

/// Learning example for adaptive learning
#[derive(Debug, Clone)]
pub struct LearningExample {
    pub context: EmergencyContext,
    pub result: RoleDetectionResult,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Learning insights
#[derive(Debug, Clone)]
pub struct LearningInsights {
    pub total_examples: usize,
    pub victim_percentage: f32,
    pub bystander_percentage: f32,
    pub average_confidence: f32,
}

// Placeholder implementations for sensor analyzers
pub struct MovementAnalyzer;
impl MovementAnalyzer {
    pub fn new() -> Self { Self }
    pub fn analyze(&self, _movement: &DeviceMovement) -> SensorAnalysis {
        SensorAnalysis {
            role: UserRole::Unknown,
            confidence: 0.0,
            analysis: "Movement analysis".to_string(),
        }
    }
}

pub struct LocationAnalyzer;
impl LocationAnalyzer {
    pub fn new() -> Self { Self }
    pub fn analyze(&self, _location: &Option<LocationContext>) -> SensorAnalysis {
        SensorAnalysis {
            role: UserRole::Unknown,
            confidence: 0.0,
            analysis: "Location analysis".to_string(),
        }
    }
}

pub struct AudioAnalyzer;
impl AudioAnalyzer {
    pub fn new() -> Self { Self }
    pub fn analyze(&self, _audio: &AudioEnvironment) -> SensorAnalysis {
        SensorAnalysis {
            role: UserRole::Unknown,
            confidence: 0.0,
            analysis: "Audio analysis".to_string(),
        }
    }
} 