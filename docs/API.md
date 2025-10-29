# API Documentation

## Overview

This document provides comprehensive API documentation for the Solana SOS emergency response system. The API is designed for reliability, performance, and safety in emergency situations.

## Core Components

### Voice Interface (`src/public/voice_interface.rs`)

The voice interface handles all voice recognition and audio processing functionality.

#### Key Functions

```rust
/// Processes emergency voice input and returns guidance
/// 
/// # Arguments
/// * `input` - Raw audio input from microphone
/// * `emergency_type` - Type of emergency detected
/// 
/// # Returns
/// * `Result<Guidance, Error>` - Emergency guidance or error
pub fn process_emergency_input(
    input: &[u8], 
    emergency_type: EmergencyType
) -> Result<Guidance, EmergencyError>

/// Activates voice recognition for emergency detection
/// 
/// # Returns
/// * `Result<(), VoiceError>` - Success or error
pub fn activate_voice_recognition() -> Result<(), VoiceError>

/// Processes voice input for emergency phrase detection
/// 
/// # Arguments
/// * `audio_data` - Raw audio data from microphone
/// 
/// # Returns
/// * `Result<EmergencyType, VoiceError>` - Detected emergency type or error
pub fn detect_emergency_phrase(audio_data: &[u8]) -> Result<EmergencyType, VoiceError>
```

#### Voice Recognition Configuration

```rust
pub struct VoiceConfig {
    pub activation_phrase: String,        // "Hey SOS"
    pub recognition_threshold: f32,       // Confidence threshold
    pub noise_reduction: bool,            // Enable RNNoise filtering
    pub offline_mode: bool,               // Use offline Vosk recognition
}
```

### Emergency System (`src/public/emergency_interface.rs`)

The emergency system manages all emergency response protocols and coordination.

#### Key Functions

```rust
/// Initiates emergency response for detected emergency
/// 
/// # Arguments
/// * `emergency_type` - Type of emergency detected
/// * `location` - GPS coordinates
/// 
/// # Returns
/// * `Result<EmergencyResponse, EmergencyError>` - Response or error
pub fn initiate_emergency_response(
    emergency_type: EmergencyType,
    location: Location
) -> Result<EmergencyResponse, EmergencyError>

/// Provides step-by-step guidance for emergency
/// 
/// # Arguments
/// * `emergency_type` - Type of emergency
/// * `step_number` - Current step in protocol
/// 
/// # Returns
/// * `Result<Guidance, EmergencyError>` - Guidance or error
pub fn get_emergency_guidance(
    emergency_type: EmergencyType,
    step_number: u32
) -> Result<Guidance, EmergencyError>

/// Records emergency response on blockchain
/// 
/// # Arguments
/// * `emergency_data` - Emergency response data
/// 
/// # Returns
/// * `Result<Transaction, BlockchainError>` - Transaction or error
pub fn record_emergency_response(
    emergency_data: EmergencyData
) -> Result<Transaction, BlockchainError>
```

#### Emergency Response Structure

```rust
pub struct EmergencyResponse {
    pub emergency_type: EmergencyType,
    pub guidance: Vec<GuidanceStep>,
    pub contacts_notified: Vec<Contact>,
    pub emergency_services_called: bool,
    pub blockchain_recorded: bool,
    pub timestamp: DateTime<Utc>,
}

pub struct GuidanceStep {
    pub step_number: u32,
    pub title: String,
    pub description: String,
    pub estimated_duration: Duration,
    pub audio_file: Option<String>,
}
```

### Gamification (`src/public/gamification_interface.rs`)

The gamification system manages SOS Hero rewards and achievements.

#### Key Functions

```rust
/// Awards tokens for emergency preparedness actions
/// 
/// # Arguments
/// * `action` - Type of action performed
/// * `user_wallet` - User's Solana wallet
/// 
/// # Returns
/// * `Result<TokenReward, GamificationError>` - Reward or error
pub fn award_tokens(
    action: PreparednessAction,
    user_wallet: Wallet
) -> Result<TokenReward, GamificationError>

/// Unlocks achievement for user
/// 
/// # Arguments
/// * `achievement` - Achievement to unlock
/// * `user_id` - User identifier
/// 
/// # Returns
/// * `Result<Achievement, GamificationError>` - Achievement or error
pub fn unlock_achievement(
    achievement: AchievementType,
    user_id: String
) -> Result<Achievement, GamificationError>

/// Gets user's SOS Hero level and progress
/// 
/// # Arguments
/// * `user_id` - User identifier
/// 
/// # Returns
/// * `Result<HeroLevel, GamificationError>` - Level or error
pub fn get_hero_level(user_id: String) -> Result<HeroLevel, GamificationError>
```

#### Gamification Structures

```rust
pub struct TokenReward {
    pub bonk_tokens: u64,
    pub skr_tokens: u64,
    pub xp_points: u32,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

pub struct Achievement {
    pub achievement_type: AchievementType,
    pub title: String,
    pub description: String,
    pub tokens_awarded: TokenReward,
    pub unlocked_at: DateTime<Utc>,
}

pub struct HeroLevel {
    pub level: u32,
    pub xp_current: u32,
    pub xp_required: u32,
    pub achievements: Vec<Achievement>,
    pub total_tokens: TokenReward,
}
```

### Safety Features (`src/public/safety_interface.rs`)

The safety features system manages Silent SOS, crash detection, and trusted network functionality.

#### Key Functions

```rust
/// Activates Silent SOS for dangerous situations
/// 
/// # Arguments
/// * `duration` - How long to hold button
/// * `location` - Current GPS location
/// 
/// # Returns
/// * `Result<SilentSOSResponse, SafetyError>` - Response or error
pub fn activate_silent_sos(
    duration: Duration,
    location: Location
) -> Result<SilentSOSResponse, SafetyError>

/// Detects potential crash based on sensor data
/// 
/// # Arguments
/// * `sensor_data` - Accelerometer and GPS data
/// 
/// # Returns
/// * `Result<CrashDetection, SafetyError>` - Detection or error
pub fn detect_crash(sensor_data: SensorData) -> Result<CrashDetection, SafetyError>

/// Notifies trusted contacts of emergency
/// 
/// # Arguments
/// * `emergency_data` - Emergency information
/// * `contacts` - List of trusted contacts
/// 
/// # Returns
/// * `Result<NotificationResponse, SafetyError>` - Response or error
pub fn notify_trusted_contacts(
    emergency_data: EmergencyData,
    contacts: Vec<Contact>
) -> Result<NotificationResponse, SafetyError>
```

#### Safety Feature Structures

```rust
pub struct SilentSOSResponse {
    pub activated: bool,
    pub contacts_notified: Vec<Contact>,
    pub emergency_services_called: bool,
    pub location_shared: bool,
    pub timestamp: DateTime<Utc>,
}

pub struct CrashDetection {
    pub detected: bool,
    pub severity: CrashSeverity,
    pub location: Location,
    pub speed_at_impact: f32,
    pub force_measurement: f32,
    pub timestamp: DateTime<Utc>,
}

pub struct NotificationResponse {
    pub contacts_notified: Vec<Contact>,
    pub messages_sent: u32,
    pub responses_received: u32,
    pub timestamp: DateTime<Utc>,
}
```

## Data Types

### Emergency Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    SuicidePrevention,
    OverdoseReversal,
    HypothermiaSelfRescue,
}
```

### Location Data

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f32,
    pub timestamp: DateTime<Utc>,
}
```

### Contact Information

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub relationship: String,
    pub notification_preferences: NotificationPreferences,
}
```

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum EmergencyError {
    #[error("Voice recognition failed: {0}")]
    VoiceRecognitionError(String),
    
    #[error("Emergency protocol not found: {0}")]
    ProtocolNotFound(EmergencyType),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    
    #[error("Blockchain error: {0}")]
    BlockchainError(#[from] solana_sdk::transaction::TransactionError),
    
    #[error("Network error: {0}")]
    NetworkError(String),
}
```

### Error Handling Patterns

```rust
// Example error handling
match process_emergency_input(audio_data, emergency_type) {
    Ok(guidance) => {
        // Process guidance
        display_emergency_guidance(guidance);
    }
    Err(EmergencyError::VoiceRecognitionError(msg)) => {
        // Handle voice recognition failure
        fallback_to_manual_activation();
    }
    Err(EmergencyError::ProtocolNotFound(emergency_type)) => {
        // Handle missing protocol
        request_online_protocol(emergency_type);
    }
    Err(e) => {
        // Handle other errors
        log_error(&e);
        activate_fallback_mode();
    }
}
```

## Performance Requirements

### Response Times

- **Voice Recognition**: < 500ms
- **Emergency Activation**: < 100ms
- **Guidance Generation**: < 200ms
- **Contact Notification**: < 1 second
- **Blockchain Recording**: < 2 seconds

### Memory Usage

- **Voice Recognition**: < 50MB
- **Emergency Protocols**: < 10MB
- **Database**: < 5MB
- **Total App Memory**: < 100MB

### Battery Impact

- **Background Monitoring**: < 1% per hour
- **Voice Recognition**: < 5% per hour
- **Crash Detection**: < 2% per hour
- **Total Battery Impact**: < 8% per hour

## Security Considerations

### Data Encryption

- All sensitive data encrypted with AES-256
- Database encrypted at rest
- Network communications encrypted with TLS
- Private keys never leave device

### Access Control

- Biometric authentication for sensitive features
- PIN protection for emergency bypass
- Granular permissions for data access
- Emergency-only data sharing

### Privacy Protection

- Local-first data processing
- Minimal data collection
- User-controlled data sharing
- Automatic data deletion

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergency_voice_recognition() {
        let input = b"Hey SOS drowning help";
        let result = process_emergency_input(input, EmergencyType::Drowning);
        assert!(result.is_ok());
    }

    #[test]
    fn test_silent_sos_activation() {
        let location = Location {
            latitude: 40.7128,
            longitude: -74.0060,
            accuracy: 10.0,
            timestamp: Utc::now(),
        };
        let result = activate_silent_sos(Duration::from_secs(3), location);
        assert!(result.is_ok());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_complete_emergency_response() {
    // Test full emergency scenario
    let audio_data = load_test_audio("drowning_emergency.wav");
    let emergency_type = EmergencyType::Drowning;
    
    let response = initiate_emergency_response(emergency_type, test_location()).await;
    assert!(response.is_ok());
    
    let guidance = get_emergency_guidance(emergency_type, 1).await;
    assert!(guidance.is_ok());
}
```

## Examples

### Basic Emergency Response

```rust
// Detect emergency from voice input
let emergency_type = detect_emergency_phrase(audio_data)?;

// Get current location
let location = get_current_location()?;

// Initiate emergency response
let response = initiate_emergency_response(emergency_type, location)?;

// Display guidance
for step in response.guidance {
    display_guidance_step(step);
    wait_for_user_confirmation();
}

// Record on blockchain
record_emergency_response(response.emergency_data)?;
```

### Silent SOS Activation

```rust
// Monitor button press duration
let press_duration = monitor_button_press();

if press_duration >= Duration::from_secs(3) {
    let location = get_current_location()?;
    let response = activate_silent_sos(press_duration, location)?;
    
    if response.activated {
        // Notify contacts silently
        notify_trusted_contacts(emergency_data, trusted_contacts)?;
        
        // Call emergency services
        call_emergency_services(location)?;
    }
}
```

### Crash Detection

```rust
// Monitor sensor data
let sensor_data = get_sensor_data();

if let Ok(crash) = detect_crash(sensor_data) {
    if crash.detected && crash.severity >= CrashSeverity::Moderate {
        // Initiate emergency response
        let response = initiate_emergency_response(
            EmergencyType::Trauma,
            crash.location
        )?;
        
        // Notify contacts
        notify_trusted_contacts(emergency_data, trusted_contacts)?;
        
        // Call emergency services
        call_emergency_services(crash.location)?;
    }
}
```

## Blockchain Integration

### Solana Token Rewards

The blockchain integration uses Solana for decentralized token rewards and verifiable emergency records.

#### Reward Functions

```rust
/// Calculate training rewards based on performance
/// 
/// # Arguments
/// * `module` - Training module completed
/// * `score` - Performance score (0-100)
/// * `completion_time` - Time taken in seconds
/// 
/// # Returns
/// * `TokenReward` - BONK and SKR token amounts
pub fn calculate_training_reward(
    module: TrainingModule,
    score: u8,
    completion_time: u32,
) -> TokenReward {
    let base_reward = match module {
        TrainingModule::CPR => 150,
        TrainingModule::FirstAid => 100,
        TrainingModule::Heimlich => 100,
        TrainingModule::AED => 125,
        _ => 100,
    };
    
    let multiplier = match score {
        90..=100 => 1.5,
        80..=89 => 1.2,
        70..=79 => 1.0,
        _ => 0.5,
    };
    
    let bonk_tokens = (base_reward as f32 * multiplier) as u64;
    let skr_tokens = bonk_tokens / 2;
    
    TokenReward {
        bonk_tokens,
        skr_tokens,
        xp_points: score as u32 * 10,
        reason: format!("Training: {:?}", module),
        timestamp: Utc::now(),
    }
}

/// Submit token reward transaction to Solana blockchain
/// 
/// # Arguments
/// * `wallet` - User's Solana wallet address
/// * `reward` - Token reward to distribute
/// 
/// # Returns
/// * `Result<Signature, BlockchainError>` - Transaction signature or error
pub async fn submit_reward_transaction(
    wallet: Pubkey,
    reward: TokenReward,
) -> Result<Signature, BlockchainError>
```

#### Mobile Wallet Adapter Integration

```kotlin
// Kotlin/Android integration with Mobile Wallet Adapter
class MobileWalletAdapter {
    fun connectWallet(): Result<PublicKey> {
        // Connect to Solana Mobile wallet
        val walletAdapter = MobileWalletAdapterClient()
        return walletAdapter.authorize(
            identity: AppIdentity(
                identityName: "Solana SOS",
                identityUri: "https://solanasos.com",
                iconUri: "https://solanasos.com/icon.png"
            )
        )
    }
    
    suspend fun signRewardTransaction(
        transaction: Transaction
    ): Result<ByteArray> {
        // Sign transaction via Mobile Wallet Adapter
        return walletAdapter.signTransaction(transaction)
    }
}
```

### Emergency Record Verification

```rust
/// Store emergency response record on-chain
/// 
/// # Arguments
/// * `emergency_type` - Type of emergency
/// * `outcome` - Success/failure of intervention
/// * `timestamp` - When emergency occurred
/// 
/// # Returns
/// * `Result<Signature, BlockchainError>` - On-chain record signature
pub async fn record_emergency_on_chain(
    emergency_type: EmergencyType,
    outcome: EmergencyOutcome,
    timestamp: DateTime<Utc>,
) -> Result<Signature, BlockchainError> {
    // Create minimal privacy-preserving record
    let record = EmergencyRecord {
        emergency_type_hash: hash_emergency_type(emergency_type),
        outcome_flag: outcome as u8,
        timestamp: timestamp.timestamp() as u64,
        // No personal data, only verification proof
    };
    
    submit_emergency_record(record).await
}
```

## JNI Bridge (Android Integration)

### Rust to Kotlin Bridge

The JNI bridge enables seamless communication between Rust core and Android UI.

#### Native Method Declarations

```kotlin
// Kotlin declarations
class EmergencyNative {
    external fun initializeEmergencySystem(): Boolean
    external fun processVoiceInput(audioData: ByteArray): String
    external fun getEmergencyProtocol(emergencyType: String): String
    external fun calculateReward(module: String, score: Int): Long
    
    companion object {
        init {
            System.loadLibrary("solana_sos")
        }
    }
}
```

#### JNI Implementation

```rust
// Rust JNI implementation
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_EmergencyNative_initializeEmergencySystem(
    env: JNIEnv,
    _class: JClass,
) -> jboolean {
    match EmergencySystem::new() {
        Ok(_) => JNI_TRUE,
        Err(e) => {
            log::error!("Failed to initialize: {}", e);
            JNI_FALSE
        }
    }
}

#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_EmergencyNative_processVoiceInput(
    env: JNIEnv,
    _class: JClass,
    audio_data: jbyteArray,
) -> jstring {
    // Convert Java byte array to Rust
    let audio_bytes = env.convert_byte_array(audio_data)
        .expect("Failed to convert audio data");
    
    // Process voice input
    let result = process_emergency_voice(&audio_bytes);
    
    // Convert result to Java string
    env.new_string(result.to_json())
        .expect("Failed to create string")
        .into_inner()
}
```

## Configuration

### System Configuration

```rust
pub struct SystemConfig {
    /// Voice recognition settings
    pub voice_config: VoiceConfig,
    
    /// Database path and encryption
    pub database_path: PathBuf,
    pub database_key: [u8; 32],
    
    /// Blockchain RPC endpoint
    pub solana_rpc_url: String,
    pub solana_cluster: Cluster,
    
    /// Emergency settings
    pub auto_call_911: bool,
    pub contact_notification_delay: Duration,
    pub crash_detection_sensitivity: f32,
    
    /// Privacy settings
    pub data_retention_days: u32,
    pub location_precision: LocationPrecision,
    pub anonymous_stats: bool,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            voice_config: VoiceConfig::default(),
            database_path: PathBuf::from("emergency.db"),
            database_key: generate_secure_key(),
            solana_rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            solana_cluster: Cluster::Mainnet,
            auto_call_911: true,
            contact_notification_delay: Duration::from_secs(10),
            crash_detection_sensitivity: 0.85,
            data_retention_days: 365,
            location_precision: LocationPrecision::High,
            anonymous_stats: true,
        }
    }
}
```

## Advanced Features

### Emergency Protocol Customization

```rust
/// Register custom emergency protocol
/// 
/// # Arguments
/// * `emergency_type` - Type of emergency
/// * `protocol` - Custom protocol steps
/// 
/// # Returns
/// * `Result<(), ProtocolError>` - Success or error
pub fn register_custom_protocol(
    emergency_type: EmergencyType,
    protocol: Protocol,
) -> Result<(), ProtocolError> {
    // Validate protocol meets safety standards
    validate_protocol(&protocol)?;
    
    // Register in database
    db.insert_protocol(emergency_type, protocol)?;
    
    Ok(())
}

/// Protocol validation rules
fn validate_protocol(protocol: &Protocol) -> Result<(), ProtocolError> {
    // Must have at least 3 steps
    if protocol.steps.len() < 3 {
        return Err(ProtocolError::InsufficientSteps);
    }
    
    // Must include 911 call instruction
    if !protocol.includes_emergency_call() {
        return Err(ProtocolError::MissingEmergencyCall);
    }
    
    // Steps must have clear instructions
    for step in &protocol.steps {
        if step.description.len() < 10 {
            return Err(ProtocolError::InsufficientDetail);
        }
    }
    
    Ok(())
}
```

### Sensor Integration

```rust
/// Process accelerometer data for crash detection
/// 
/// # Arguments
/// * `accel_data` - Raw accelerometer readings (x, y, z)
/// * `timestamp` - When reading was taken
/// 
/// # Returns
/// * `Option<CrashEvent>` - Crash event if detected
pub fn process_accelerometer_data(
    accel_data: (f32, f32, f32),
    timestamp: DateTime<Utc>,
) -> Option<CrashEvent> {
    let (x, y, z) = accel_data;
    
    // Calculate g-force magnitude
    let magnitude = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
    
    // Crash threshold: > 4G
    if magnitude > 4.0 {
        Some(CrashEvent {
            force: magnitude,
            timestamp,
            location: get_current_location().ok(),
        })
    } else {
        None
    }
}

/// Monitor GPS for unusual patterns
/// 
/// # Arguments
/// * `location_history` - Recent GPS coordinates
/// 
/// # Returns
/// * `Option<AlertType>` - Alert if unusual pattern detected
pub fn analyze_location_pattern(
    location_history: &[Location],
) -> Option<AlertType> {
    if location_history.len() < 5 {
        return None;
    }
    
    // Detect if user stationary in unusual location
    // Could indicate medical emergency
    let is_stationary = location_history
        .windows(2)
        .all(|w| distance(&w[0], &w[1]) < 10.0);
    
    if is_stationary {
        Some(AlertType::StationaryAlert)
    } else {
        None
    }
}
```

## Versioning

The API follows semantic versioning (SemVer):

- **Major version**: Breaking changes
- **Minor version**: New features, backward compatible
- **Patch version**: Bug fixes, backward compatible

Current version: **1.0.0**

### API Stability Guarantees

- **Public API** (`src/public/*`): Stable, follows SemVer
- **Internal API** (`src/*`): May change between minor versions
- **JNI Bridge**: Stable within major versions
- **Protocol Format**: Backward compatible within major versions

## Support

For API questions or issues:

- **GitHub Issues**: [Report bugs](https://github.com/paragoner1/crisis-companion/issues)
- **Documentation**: [Complete guides](https://github.com/paragoner1/crisis-companion/tree/main/docs)
- **Examples**: [Code examples](https://github.com/paragoner1/crisis-companion/tree/main/examples)

---

**Remember**: This API is designed for emergency situations. All functions prioritize reliability and safety over performance optimization. 