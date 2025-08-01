use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::AppResult;
use std::collections::HashMap;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub voice: VoiceConfig,
    pub audio: AudioConfig,
    pub database: DatabaseConfig,
    pub coordination: CoordinationConfig,
    pub emergency: EmergencyConfig,
    pub ui: UIConfig,
    pub blockchain: BlockchainConfig,
    pub connectivity: ConnectivityConfig,
}

/// Voice recognition configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    /// Path to Vosk model directory
    pub model_path: String,
    /// Minimum confidence threshold for trigger detection (0.0-1.0)
    pub confidence_threshold: f32,
    /// Emergency phrases to detect
    pub emergency_phrases: Vec<String>,
    /// Audio sample rate (Hz)
    pub sample_rate: u32,
    /// Buffer size for audio processing
    pub buffer_size: usize,
    /// Enable continuous listening
    pub continuous_listening: bool,
    /// Timeout for voice detection (seconds)
    pub detection_timeout: u64,
}

/// Audio processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    /// Default volume level (0.0-1.0)
    pub default_volume: f32,
    /// Emergency volume level (0.0-1.0)
    pub emergency_volume: f32,
    /// Text-to-speech voice
    pub tts_voice: String,
    /// Audio output device
    pub output_device: Option<String>,
    /// Enable audio recording
    pub enable_recording: bool,
    /// Recording quality (Hz)
    pub recording_sample_rate: u32,
    /// Recording format
    pub recording_format: String,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
    pub emergency_instructions_path: String,
    pub user_profiles_path: String,
    pub emergency_history_path: String,
}

/// Connectivity configuration for hybrid architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectivityConfig {
    /// User preference for connectivity mode
    pub user_preference: Option<String>, // "offline", "online", "hybrid"
    /// Network timeout in seconds
    pub network_timeout: u64,
    /// Connectivity check interval in seconds
    pub check_interval: u64,
    /// Enable automatic mode switching
    pub auto_switch: bool,
    /// Fallback to offline mode on network failure
    pub offline_fallback: bool,
    /// LLM API endpoints for online mode
    pub llm_endpoints: Vec<String>,
    /// API keys for online services
    pub api_keys: std::collections::HashMap<String, String>,
}

/// Device coordination configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationConfig {
    /// Bluetooth service UUID
    pub service_uuid: String,
    /// Coordination timeout (seconds)
    pub coordination_timeout: u64,
    /// Maximum nearby devices to coordinate with
    pub max_nearby_devices: u32,
    /// Enable automatic device discovery
    pub enable_discovery: bool,
    /// Device name for BLE advertising
    pub device_name: String,
    /// Battery level threshold for coordination decisions
    pub battery_threshold: f32,
}

/// Emergency response configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyConfig {
    /// Enable automatic 911 dialing
    pub auto_dial_911: bool,
    /// Enable location sharing
    pub enable_location_sharing: bool,
    /// Enable audio recording during emergency
    pub enable_audio_recording: bool,
    /// Recording duration (seconds)
    pub recording_duration: u64,
    /// Emergency contact numbers
    pub emergency_contacts: Vec<String>,
    /// Enable family alerts
    pub enable_family_alerts: bool,
    /// Alert timeout (seconds)
    pub alert_timeout: u64,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    /// UI theme (light/dark/auto)
    pub theme: String,
    /// Enable animations
    pub enable_animations: bool,
    /// UI refresh rate (Hz)
    pub refresh_rate: u32,
    /// Enable accessibility features
    pub enable_accessibility: bool,
    /// Language for UI text
    pub language: String,
}

/// Blockchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainConfig {
    /// Solana RPC endpoint
    pub rpc_endpoint: String,
    /// Wallet keypair path
    pub wallet_path: Option<String>,
    /// Program ID for Crisis Companion
    pub program_id: String,
    /// Enable blockchain features
    pub enable_blockchain: bool,
    /// Gas fee limit (lamports)
    pub gas_fee_limit: u64,
}

impl AppConfig {
    /// Load configuration from file
    pub fn load<P: AsRef<Path>>(path: P) -> AppResult<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .add_source(config::Environment::with_prefix("CRISIS_COMPANION"))
            .build()?;
        
        Ok(settings.try_deserialize()?)
    }
    
    /// Get default configuration
    pub fn default() -> Self {
        Self {
            voice: VoiceConfig::default(),
            audio: AudioConfig::default(),
            database: DatabaseConfig::default(),
            coordination: CoordinationConfig::default(),
            emergency: EmergencyConfig::default(),
            ui: UIConfig::default(),
            blockchain: BlockchainConfig::default(),
            connectivity: ConnectivityConfig::default(),
        }
    }
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            model_path: "models/vosk-model-small-en-us-0.15".to_string(),
            confidence_threshold: 0.8,
            emergency_phrases: vec![
                "drowning help".to_string(),
                "fire help".to_string(),
                "heart attack help".to_string(),
                "choking help".to_string(),
                "bleeding help".to_string(),
            ],
            sample_rate: 16000,
            buffer_size: 4096,
            continuous_listening: true,
            detection_timeout: 30,
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            default_volume: 0.5,
            emergency_volume: 1.0,
            tts_voice: "en-US-Standard-A".to_string(),
            output_device: None,
            enable_recording: true,
            recording_sample_rate: 44100,
            recording_format: "wav".to_string(),
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: "data/emergencies.db".to_string(),
            emergency_instructions_path: "data/emergency_instructions.json".to_string(),
            user_profiles_path: "data/user_profiles.json".to_string(),
            emergency_history_path: "data/emergency_history.json".to_string(),
        }
    }
}

impl Default for ConnectivityConfig {
    fn default() -> Self {
        Self {
            user_preference: Some("hybrid".to_string()),
            network_timeout: 10,
            check_interval: 60,
            auto_switch: true,
            offline_fallback: true,
            llm_endpoints: vec!["https://api.openai.com/v1/chat/completions".to_string()],
            api_keys: HashMap::new(),
        }
    }
}

impl Default for CoordinationConfig {
    fn default() -> Self {
        Self {
            service_uuid: "12345678-1234-1234-1234-123456789abc".to_string(),
            coordination_timeout: 10,
            max_nearby_devices: 5,
            enable_discovery: true,
            device_name: "Crisis Companion".to_string(),
            battery_threshold: 0.2,
        }
    }
}

impl Default for EmergencyConfig {
    fn default() -> Self {
        Self {
            auto_dial_911: true,
            enable_location_sharing: true,
            enable_audio_recording: true,
            recording_duration: 300, // 5 minutes
            emergency_contacts: vec!["911".to_string()],
            enable_family_alerts: false,
            alert_timeout: 60,
        }
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            theme: "auto".to_string(),
            enable_animations: true,
            refresh_rate: 60,
            enable_accessibility: true,
            language: "en".to_string(),
        }
    }
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            rpc_endpoint: "https://api.mainnet-beta.solana.com".to_string(),
            wallet_path: None,
            program_id: "CrisisCompanion111111111111111111111111111111111".to_string(),
            enable_blockchain: false,
            gas_fee_limit: 5000,
        }
    }
} 