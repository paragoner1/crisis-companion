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
}

// Re-export common types
pub use types::*; // BONK and SKR token integration planned for Q1 2026
