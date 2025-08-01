//! Crisis Companion - Voice-activated emergency response app for Solana Mobile Seeker
//! 
//! This library provides the core functionality for detecting emergency voice triggers,
//! coordinating with nearby devices, and providing emergency instructions offline.
//! 
//! # Core Features
//! - Voice trigger detection using Vosk (offline speech recognition)
//! - Automatic volume control and audio playback
//! - Multi-device coordination via Bluetooth Low Energy
//! - Offline emergency instructions database
//! - Solana blockchain integration for security and payments
//! - Emergency response features (911 dialing, location sharing, audio recording)

pub mod app;
pub mod audio;
pub mod blockchain;
pub mod config;
pub mod coordination;
pub mod database;
pub mod emergency;
pub mod error;
pub mod ui;
pub mod voice;
pub mod noise_filter;
pub mod adaptive_training;
pub mod role_detection;

// Re-export main types for convenience
pub use app::CrisisCompanionApp;
pub use audio::AudioManager;
pub use blockchain::BlockchainManager;
pub use config::AppConfig;
pub use coordination::DeviceCoordinator;
pub use database::EmergencyDatabase;
pub use emergency::EmergencyHandler;
pub use error::AppError;
pub use ui::AppUI;
pub use voice::VoiceTrigger;
pub use adaptive_training::AdaptiveTrainer;
pub use role_detection::{RoleDetector, RoleDetectionConfig, EmergencyContext};

// Common types used across modules
pub mod types {
    use serde::{Deserialize, Serialize};
    use chrono::{DateTime, Utc};
    use uuid::Uuid;

    /// Emergency types that can be detected
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum EmergencyType {
        Drowning,
        Fire,
        HeartAttack,
        Choking,
        Bleeding,
        Unconscious,
        Seizure,
        AllergicReaction,
        Poisoning,
        Trauma,
    }

    /// Voice trigger detection result
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VoiceTriggerResult {
        pub emergency_type: EmergencyType,
        pub confidence: f32,
        pub timestamp: DateTime<Utc>,
        pub audio_hash: String,
    }

    /// Emergency instruction step
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmergencyInstruction {
        pub id: Uuid,
        pub emergency_type: EmergencyType,
        pub step_number: u32,
        pub title: String,
        pub description: String,
        pub audio_file: Option<String>,
        pub estimated_duration_seconds: u32,
    }

    /// Device coordination message
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CoordinationMessage {
        pub device_id: Uuid,
        pub emergency_type: EmergencyType,
        pub timestamp: DateTime<Utc>,
        pub battery_level: f32,
        pub location: Option<(f64, f64)>, // (latitude, longitude)
        pub action: CoordinationAction,
    }

    /// Coordination actions between devices
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum CoordinationAction {
        /// This device will provide voice instructions
        ProvideInstructions,
        /// This device will dial 911
        Dial911,
        /// This device will record audio
        RecordAudio,
        /// This device will share location
        ShareLocation,
        /// This device will display silent instructions
        DisplayInstructions,
    }

    /// Emergency response status
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmergencyResponse {
        pub id: Uuid,
        pub emergency_type: EmergencyType,
        pub trigger_timestamp: DateTime<Utc>,
        pub response_start: DateTime<Utc>,
        pub response_end: Option<DateTime<Utc>>,
        pub status: ResponseStatus,
        pub instructions_provided: Vec<Uuid>,
        pub audio_recorded: bool,
        pub location_shared: bool,
        pub emergency_called: bool,
    }

    /// Response status
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    pub enum ResponseStatus {
        Active,
        Completed,
        Cancelled,
        Failed,
    }

    /// User role in emergency situation
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum UserRole {
        /// User is in danger and needs help
        Victim,
        /// User is helping someone else
        Bystander,
        /// Role not yet determined
        Unknown,
    }

    /// Role detection method used
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum RoleDetectionMethod {
        /// Detected via UI button tap
        UITap,
        /// Detected via voice confirmation
        VoiceConfirmation,
        /// Detected via AI context inference
        AIInference,
        /// Detected via user profile setting
        UserProfile,
        /// Detected via sensor fusion
        SensorFusion,
    }

    /// Role detection result
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RoleDetectionResult {
        pub role: UserRole,
        pub method: RoleDetectionMethod,
        pub confidence: f32,
        pub detection_time_ms: u64,
        pub context_data: Option<RoleContext>,
    }

    /// Context data for role detection
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RoleContext {
        pub phrase_detected: String,
        pub sensor_data: Option<SensorData>,
        pub user_profile: Option<UserProfile>,
        pub emergency_type: EmergencyType,
    }

    /// Sensor data for role inference
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SensorData {
        pub device_movement: DeviceMovement,
        pub location_context: Option<LocationContext>,
        pub audio_environment: AudioEnvironment,
        pub battery_level: f32,
    }

    /// Device movement patterns
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum DeviceMovement {
        Stationary,
        Walking,
        Running,
        Swimming,
        Falling,
        Unknown,
    }

    /// Location context for role inference
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LocationContext {
        pub location_type: LocationType,
        pub coordinates: Option<(f64, f64)>,
        pub nearby_landmarks: Vec<String>,
    }

    /// Types of locations
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum LocationType {
        Beach,
        Pool,
        Hospital,
        Home,
        Public,
        Vehicle,
        Unknown,
    }

    /// Audio environment analysis
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AudioEnvironment {
        pub noise_level: f32,
        pub crowd_density: f32,
        pub wind_level: f32,
        pub background_sounds: Vec<String>,
    }

    /// User profile for role detection
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserProfile {
        pub default_role: UserRole,
        pub is_caregiver: bool,
        pub emergency_contacts: Vec<String>,
        pub medical_info: Option<MedicalInfo>,
        pub voice_model: Option<VoiceModel>,
    }

    /// Medical information (optional)
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MedicalInfo {
        pub allergies: Vec<String>,
        pub conditions: Vec<String>,
        pub medications: Vec<String>,
        pub emergency_notes: Option<String>,
    }

    /// Voice model for adaptive training
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VoiceModel {
        pub user_id: String,
        pub adaptation_score: f32,
        pub accent_type: Option<String>,
        pub speech_patterns: Vec<String>,
        pub last_updated: DateTime<Utc>,
    }
}

// Re-export common types
pub use types::*; // BONK and SKR token integration planned for Q1 2026
