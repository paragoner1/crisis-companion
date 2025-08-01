# 📚 Solana SOS API Documentation

## Overview

This document provides comprehensive API documentation for Solana SOS, covering all public interfaces and their usage.

## Core Interfaces

### Voice Interface

The voice interface handles emergency phrase detection and voice recognition.

#### VoiceTrigger

```rust
pub struct VoiceTrigger {
    pub is_active: bool,
    pub confidence_threshold: f32,
    pub current_confidence: f32,
}
```

**Methods:**

- `new() -> Self` - Creates a new voice trigger instance
- `detect_emergency_phrase(&mut self, phrase: &str) -> AppResult<bool>` - Detects emergency phrases
- `activate(&mut self) -> AppResult<()>` - Activates voice trigger
- `deactivate(&mut self) -> AppResult<()>` - Deactivates voice trigger
- `set_confidence_threshold(&mut self, threshold: f32)` - Sets confidence threshold
- `generate_audio_hash(phrase: &str) -> String` - Generates audio hash
- `simulate_phrase_detection(phrase: &str) -> AppResult<EmergencyType>` - Simulates phrase detection

#### VoiceConfig

```rust
pub struct VoiceConfig {
    pub wake_word: String,
    pub emergency_phrases: Vec<String>,
    pub noise_filter_enabled: bool,
}
```

### Audio Interface

The audio interface manages audio input/output and processing.

#### AudioProcessor

```rust
pub struct AudioProcessor {
    pub is_active: bool,
    pub sample_rate: u32,
    pub buffer_size: usize,
}
```

**Methods:**

- `new() -> AppResult<Self>` - Creates new audio processor
- `initialize_streams(&mut self) -> AppResult<()>` - Initializes audio streams
- `process_audio_input(&self, audio_data: &[u8]) -> AppResult<Vec<f32>>` - Processes audio input
- `play_audio_data(&self, audio_data: &[f32]) -> AppResult<Sink>` - Plays audio data
- `generate_tts(&self, text: &str) -> AppResult<Vec<f32>>` - Generates text-to-speech
- `apply_noise_filter(&self, audio_data: &[f32]) -> AppResult<Vec<f32>>` - Applies noise filtering

### Emergency Interface

The emergency interface handles emergency response coordination.

#### EmergencySystem

```rust
pub struct EmergencySystem {
    pub is_active: bool,
    pub current_emergency: Option<EmergencyType>,
    pub response_status: EmergencyStatus,
}
```

**Methods:**

- `new() -> Self` - Creates new emergency system
- `initiate_emergency_response(&mut self, emergency_type: EmergencyType) -> AppResult<()>` - Initiates emergency response
- `call_911(&self, location: &str) -> AppResult<()>` - Makes emergency call
- `share_location(&self, latitude: f64, longitude: f64) -> AppResult<()>` - Shares location
- `record_emergency_call(&self, call_data: EmergencyCallData) -> AppResult<()>` - Records emergency call
- `get_emergency_instructions(&self) -> AppResult<Vec<String>>` - Gets emergency instructions

#### EmergencyStatus

```rust
pub enum EmergencyStatus {
    Idle,
    Active,
    ServicesContacted,
    Resolved,
    Failed,
}
```

### Gamification Interface

The gamification interface manages the SOS Hero system.

#### GamificationManager

```rust
pub struct GamificationManager {
    pub hero_profiles: RwLock<HashMap<String, HeroProfile>>,
    pub achievements: Vec<Achievement>,
    pub rewards: Vec<Reward>,
}
```

**Methods:**

- `new() -> AppResult<Self>` - Creates new gamification manager
- `initialize_achievements(&mut self) -> AppResult<()>` - Initializes achievements
- `initialize_rewards(&mut self) -> AppResult<()>` - Initializes rewards
- `get_or_create_profile(&self, user_id: &str) -> AppResult<HeroProfile>` - Gets or creates hero profile
- `award_experience(&self, user_id: &str, xp: u32) -> AppResult<()>` - Awards experience points
- `complete_learning_module(&self, user_id: &str, module_name: &str) -> AppResult<()>` - Records learning completion
- `record_intervention(&self, user_id: &str, emergency_type: &str) -> AppResult<()>` - Records emergency intervention
- `expand_network(&self, user_id: &str, contact_count: u32) -> AppResult<()>` - Expands trusted network

#### HeroLevel

```rust
pub enum HeroLevel {
    Novice = 1,
    Trainee = 2,
    Responder = 3,
    Guardian = 4,
    Protector = 5,
    Defender = 6,
    Sentinel = 7,
    Champion = 8,
    GuardianAngel = 9,
    Legend = 10,
}
```

### Safety Interface

The safety interface manages advanced safety features.

#### SilentSOS

```rust
pub struct SilentSOS {
    pub is_active: bool,
    pub hold_duration: std::time::Duration,
    pub activation_method: SilentSOSMethod,
}
```

**Methods:**

- `new() -> Self` - Creates new silent SOS instance
- `activate(&mut self, method: SilentSOSMethod) -> AppResult<()>` - Activates silent SOS
- `deactivate(&mut self) -> AppResult<()>` - Deactivates silent SOS
- `should_trigger(&self, hold_time: std::time::Duration) -> bool` - Checks if SOS should trigger

#### CrashDetection

```rust
pub struct CrashDetection {
    pub is_active: bool,
    pub speed_threshold: f32,
    pub impact_threshold: f32,
    pub sensitivity: CrashSensitivity,
}
```

**Methods:**

- `new() -> Self` - Creates new crash detection instance
- `activate(&mut self) -> AppResult<()>` - Activates crash detection
- `deactivate(&mut self) -> AppResult<()>` - Deactivates crash detection
- `process_sensor_data(&self, accelerometer_data: &[f32], gps_data: &GPSData) -> AppResult<bool>` - Processes sensor data
- `trigger_crash_response(&self, crash_data: CrashData) -> AppResult<()>` - Triggers crash response

### Blockchain Interface

The blockchain interface manages Solana integration.

#### SolanaConnection

```rust
pub struct SolanaConnection {
    pub is_connected: bool,
    pub endpoint: String,
    pub status: ConnectionStatus,
}
```

**Methods:**

- `new(endpoint: &str) -> AppResult<Self>` - Creates new Solana connection
- `connect(&mut self) -> AppResult<()>` - Connects to Solana network
- `disconnect(&mut self) -> AppResult<()>` - Disconnects from Solana network
- `get_status(&self) -> ConnectionStatus` - Gets connection status

#### TransactionManager

```rust
pub struct TransactionManager {
    pub is_active: bool,
    pub transaction_fee: u64,
    pub gas_limit: u64,
}
```

**Methods:**

- `new() -> AppResult<Self>` - Creates new transaction manager
- `record_emergency(&self, emergency_data: EmergencyData) -> AppResult<String>` - Records emergency on blockchain
- `get_emergency_record(&self, record_id: &str) -> AppResult<EmergencyRecord>` - Retrieves emergency record
- `verify_record(&self, record: &EmergencyRecord) -> AppResult<bool>` - Verifies record authenticity

### Database Interface

The database interface manages local data storage.

#### DatabaseManager

```rust
pub struct DatabaseManager {
    pub is_connected: bool,
    pub database_path: String,
    pub status: DatabaseStatus,
}
```

**Methods:**

- `new(database_path: &str) -> AppResult<Self>` - Creates new database manager
- `connect(&mut self) -> AppResult<()>` - Connects to database
- `disconnect(&mut self) -> AppResult<()>` - Disconnects from database
- `get_status(&self) -> DatabaseStatus` - Gets connection status

#### QueryManager

```rust
pub struct QueryManager {
    pub is_active: bool,
    pub query_timeout: std::time::Duration,
}
```

**Methods:**

- `new() -> AppResult<Self>` - Creates new query manager
- `get_emergency_instructions(&self, emergency_type: &str, stage: &str) -> AppResult<Vec<EmergencyInstruction>>` - Gets emergency instructions
- `get_user_profile(&self, user_id: &str) -> AppResult<UserProfile>` - Gets user profile
- `save_user_profile(&self, profile: &UserProfile) -> AppResult<()>` - Saves user profile
- `record_emergency_history(&self, history: &EmergencyHistory) -> AppResult<()>` - Records emergency history
- `get_emergency_history(&self, user_id: &str, limit: u32) -> AppResult<Vec<EmergencyHistory>>` - Gets emergency history
- `search_instructions(&self, query: &str) -> AppResult<Vec<EmergencyInstruction>>` - Searches instructions

### UI Interface

The UI interface manages user interface interactions.

#### UIManager

```rust
pub struct UIManager {
    pub is_active: bool,
    pub current_state: UIState,
    pub config: UIConfig,
}
```

**Methods:**

- `new() -> AppResult<Self>` - Creates new UI manager
- `initialize(&mut self) -> AppResult<()>` - Initializes user interface
- `show_emergency_interface(&mut self, emergency_type: &str, instructions: &[String]) -> AppResult<()>` - Shows emergency interface
- `show_voice_interface(&mut self) -> AppResult<()>` - Shows voice interface
- `show_gamification_interface(&mut self, hero_profile: &str) -> AppResult<()>` - Shows gamification interface
- `show_settings_interface(&mut self) -> AppResult<()>` - Shows settings interface
- `show_safety_interface(&mut self) -> AppResult<()>` - Shows safety interface
- `update_ui(&mut self, data: &UIData) -> AppResult<()>` - Updates UI with new data
- `handle_input(&mut self, input: &UIInput) -> AppResult<UIResponse>` - Handles user input

## Common Types

### EmergencyType

```rust
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
```

### EmergencyStage

```rust
pub enum EmergencyStage {
    InitialDetection,
    VictimExtracted,
    Unconscious,
    ConsciousButInjured,
    BreathingButUnresponsive,
    ServicesEnRoute,
    PostEmergency,
}
```

### DirectAction

```rust
pub enum DirectAction {
    CPR,
    Heimlich,
    AED,
    Tourniquet,
    EpiPen,
    RescueBreathing,
    FirstAid,
    FASTTest,
    PoisonControl,
    CoolBurn,
    MedicalAlert,
}
```

### ConnectivityMode

```rust
pub enum ConnectivityMode {
    Offline,
    Online,
    Hybrid,
}
```

### GuidanceMode

```rust
pub enum GuidanceMode {
    Offline,
    Online,
    Hybrid,
}
```

## Error Handling

All API methods return `AppResult<T>` which is an alias for `Result<T, AppError>`. The `AppError` enum includes:

```rust
pub enum AppError {
    Voice(String),
    Audio(String),
    Emergency(String),
    Database(String),
    Blockchain(String),
    Gamification(String),
    Safety(String),
    UI(String),
    Config(String),
    Network(String),
    Timeout(String),
    InvalidInput(String),
    NotFound(String),
    PermissionDenied(String),
    Internal(String),
}
```

## Usage Examples

### Basic Emergency Response

```rust
use solana_sos::{VoiceTrigger, EmergencySystem, EmergencyType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create voice trigger
    let mut voice_trigger = VoiceTrigger::new();
    voice_trigger.activate()?;
    
    // Create emergency system
    let mut emergency_system = EmergencySystem::new();
    
    // Detect emergency phrase
    if voice_trigger.detect_emergency_phrase("drowning help")? {
        // Initiate emergency response
        emergency_system.initiate_emergency_response(EmergencyType::Drowning)?;
        
        // Get emergency instructions
        let instructions = emergency_system.get_emergency_instructions()?;
        println!("Emergency instructions: {:?}", instructions);
    }
    
    Ok(())
}
```

### Gamification Integration

```rust
use solana_sos::{GamificationManager, HeroLevel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create gamification manager
    let gamification_manager = GamificationManager::new()?;
    
    // Get or create user profile
    let profile = gamification_manager.get_or_create_profile("user123").await?;
    println!("Hero level: {:?}", profile.hero_level);
    
    // Award experience for emergency response
    gamification_manager.award_experience("user123", 200).await?;
    
    // Record emergency intervention
    gamification_manager.record_intervention("user123", "drowning").await?;
    
    Ok(())
}
```

### Safety Features

```rust
use solana_sos::{SilentSOS, CrashDetection, SilentSOSMethod};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create silent SOS
    let mut silent_sos = SilentSOS::new();
    
    // Create crash detection
    let mut crash_detection = CrashDetection::new();
    crash_detection.activate()?;
    
    // Check if silent SOS should trigger
    let hold_time = std::time::Duration::from_secs(5);
    if silent_sos.should_trigger(hold_time) {
        silent_sos.activate(SilentSOSMethod::HoldButton)?;
    }
    
    Ok(())
}
```

## Best Practices

### Error Handling
- Always check return values from API calls
- Use proper error propagation with `?` operator
- Implement graceful fallbacks for critical operations

### Performance
- Reuse interface instances when possible
- Use async/await for I/O operations
- Implement proper cleanup in drop implementations

### Security
- Validate all input parameters
- Use secure random number generation
- Implement proper access controls
- Encrypt sensitive data

### Testing
- Write unit tests for all public methods
- Use integration tests for complex workflows
- Mock external dependencies
- Test error conditions and edge cases

## Conclusion

The Solana SOS API provides a comprehensive interface for voice-activated emergency response. All interfaces are designed for reliability, performance, and ease of use. The modular architecture allows for easy integration and extension of functionality. 