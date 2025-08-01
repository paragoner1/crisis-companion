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
pub mod context_analysis;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod types {
    use super::*;

    /// Emergency types that can be detected
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum EmergencyType {
        Drowning,
        HeartAttack,
        Stroke,
        Choking,
        Bleeding,
        Unconscious,
        Seizure,
        Poisoning,
        SevereBurns,
        DiabeticEmergency,
        AllergicReaction,
        Trauma,
    }

    /// Emergency stage detection
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum EmergencyStage {
        /// Emergency just detected, initial response needed
        InitialDetection,
        /// Victim extracted from danger, needs immediate care
        VictimExtracted,
        /// Victim conscious but needs medical attention
        ConsciousButInjured,
        /// Victim unconscious, needs CPR
        Unconscious,
        /// Victim breathing but unresponsive
        BreathingButUnresponsive,
        /// Emergency services en route
        ServicesEnRoute,
        /// Post-emergency care
        PostEmergency,
    }

    /// Context clues for stage detection
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ContextClues {
        /// What the user said (key phrases)
        pub user_phrase: String,
        /// Current location context
        pub location_context: Option<LocationContext>,
        /// Time since emergency started
        pub time_elapsed: Option<std::time::Duration>,
        /// Victim status (if known)
        pub victim_status: Option<VictimStatus>,
        /// Environmental conditions
        pub environment: EnvironmentContext,
        /// Bystander actions already taken
        pub actions_taken: Vec<EmergencyAction>,
    }

    /// Victim status information
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VictimStatus {
        pub is_conscious: bool,
        pub is_breathing: bool,
        pub has_pulse: bool,
        pub visible_injuries: Vec<String>,
        pub estimated_age: Option<u8>,
        pub estimated_condition: VictimCondition,
    }

    /// Victim condition assessment
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum VictimCondition {
        Stable,
        Critical,
        Unconscious,
        InShock,
        Hypothermic,
        Unknown,
    }

    /// Environmental context
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EnvironmentContext {
        pub weather_conditions: WeatherConditions,
        pub crowd_present: bool,
        pub professional_help_available: bool,
        pub emergency_equipment_available: bool,
        pub accessibility_issues: Vec<String>,
    }

    /// Weather conditions
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum WeatherConditions {
        Clear,
        Rainy,
        Windy,
        Cold,
        Hot,
        Stormy,
    }

    /// Emergency actions that may have been taken
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum EmergencyAction {
        Called911,
        ExtractedVictim,
        StartedCPR,
        AppliedFirstAid,
        MovedToSafety,
        AlertedBystanders,
        LocatedEquipment,
        AssessedInjuries,
    }

    /// Emergency guidance for a specific stage
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmergencyGuidance {
        pub stage: EmergencyStage,
        pub instructions: Vec<String>,
        pub priority_actions: Vec<String>,
        pub skip_basic_steps: bool,
        pub focus_on_medical_care: bool,
    }

    /// Emergency analysis result
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EmergencyAnalysis {
        pub stage: EmergencyStage,
        pub guidance: EmergencyGuidance,
        pub confidence: f32,
        pub context_clues: ContextClues,
    }

    /// Connectivity modes for the hybrid architecture
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ConnectivityMode {
        /// Offline-only mode - fast, reliable, privacy-focused
        Offline,
        /// Online-only mode - AI-enhanced, conversational, personalized
        Online,
        /// Hybrid mode - smart routing between offline and online
        Hybrid,
    }

    /// Guidance modes for emergency response
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum GuidanceMode {
        /// Offline guidance - context-aware, fast response
        Offline,
        /// Online guidance - AI-enhanced, conversational
        Online,
        /// Hybrid guidance - combines offline and online approaches
        Hybrid,
    }

    /// Result of guidance generation
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GuidanceResult {
        pub mode: GuidanceMode,
        pub instructions: Vec<String>,
        pub priority_actions: Vec<String>,
        pub appropriateness: f32,
        pub time_saved: u32,
        pub skipped_steps: Vec<String>,
    }

    /// Context analysis result
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ContextAnalysisResult {
        pub analysis: EmergencyAnalysis,
        pub stage_detection_confidence: f32,
        pub guidance_appropriateness: f32,
        pub time_saved_seconds: u32,
        pub skipped_steps: Vec<String>,
    }

    /// Voice trigger detection result
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VoiceTriggerResult {
        pub emergency_type: EmergencyType,
        pub confidence: f32,
        pub timestamp: DateTime<Utc>,
        pub audio_hash: String,
    }

    /// Emergency instruction for a specific emergency type
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

    /// Coordination message between devices
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

    /// Emergency response tracking
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
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum ResponseStatus {
        Active,
        Completed,
        Cancelled,
        Failed,
    }

    /// User role in emergency
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum UserRole {
        /// User is in danger and needs help
        Victim,
        /// User is helping someone else
        Bystander,
        /// Role not yet determined
        Unknown,
    }

    /// Role detection methods
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
        Remote,
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

// Re-export error types
pub use error::AppError;
