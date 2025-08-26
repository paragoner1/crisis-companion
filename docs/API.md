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

## Versioning

The API follows semantic versioning (SemVer):

- **Major version**: Breaking changes
- **Minor version**: New features, backward compatible
- **Patch version**: Bug fixes, backward compatible

Current version: **1.0.0**

## Support

For API questions or issues:

- **GitHub Issues**: [Report bugs](https://github.com/paragoner1/crisis-companion/issues)
- **Documentation**: [Complete guides](https://github.com/paragoner1/crisis-companion/tree/main/docs)
- **Examples**: [Code examples](https://github.com/paragoner1/crisis-companion/tree/main/examples)

---

**Remember**: This API is designed for emergency situations. All functions prioritize reliability and safety over performance optimization. 