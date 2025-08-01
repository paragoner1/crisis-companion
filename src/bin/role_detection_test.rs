use crisis_companion::{
    role_detection::{RoleDetector, RoleDetectionConfig, EmergencyContext},
    types::{UserRole, EmergencyType, UserProfile, DeviceMovement, LocationType, LocationContext, AudioEnvironment, SensorData},
};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🧪 Role Detection Test");
    info!("=====================");

    // Create role detector with default config
    let config = RoleDetectionConfig::default();
    let mut detector = RoleDetector::new(config);

    // Test 1: User Profile (Caregiver)
    info!("Test 1: User Profile (Caregiver)");
    let caregiver_profile = UserProfile {
        default_role: UserRole::Bystander,
        is_caregiver: true,
        emergency_contacts: vec!["family@example.com".to_string()],
        medical_info: None,
        voice_model: None,
    };
    detector.set_user_profile(caregiver_profile);

    let context = EmergencyContext::new(
        "Help, my child is drowning!".to_string(),
        EmergencyType::Drowning,
    );

    let result = detector.detect_role(&context).await?;
    info!("✅ Role detected: {:?} via {:?} (confidence: {:.2})", 
          result.role, result.method, result.confidence);

    // Test 2: AI Inference (Victim)
    info!("Test 2: AI Inference (Victim)");
    let mut detector2 = RoleDetector::new(RoleDetectionConfig::default());
    
    let context2 = EmergencyContext::new(
        "Help, I'm drowning!".to_string(),
        EmergencyType::Drowning,
    );

    let result2 = detector2.detect_role(&context2).await?;
    info!("✅ Role detected: {:?} via {:?} (confidence: {:.2})", 
          result2.role, result2.method, result2.confidence);

    // Test 3: Sensor Fusion (Bystander)
    info!("Test 3: Sensor Fusion (Bystander)");
    let mut detector3 = RoleDetector::new(RoleDetectionConfig::default());
    
    let sensor_data = SensorData {
        device_movement: DeviceMovement::Stationary,
        location_context: Some(LocationContext {
            location_type: LocationType::Beach,
            coordinates: Some((34.0522, -118.2437)), // LA coordinates
            nearby_landmarks: vec!["Santa Monica Beach".to_string()],
        }),
        audio_environment: AudioEnvironment {
            noise_level: 0.3,
            crowd_density: 0.7,
            wind_level: 0.5,
            background_sounds: vec!["waves".to_string(), "crowd".to_string()],
        },
        battery_level: 0.8,
    };

    let context3 = EmergencyContext::new(
        "Someone is drowning!".to_string(),
        EmergencyType::Drowning,
    ).with_sensor_data(sensor_data);

    let result3 = detector3.detect_role(&context3).await?;
    info!("✅ Role detected: {:?} via {:?} (confidence: {:.2})", 
          result3.role, result3.method, result3.confidence);

    // Test 4: Multi-Modal Confirmation (Fallback to Voice)
    info!("Test 4: Multi-Modal Confirmation (Fallback to Voice)");
    let mut detector4 = RoleDetector::new(RoleDetectionConfig::default());
    
    let context4 = EmergencyContext::new(
        "Emergency situation!".to_string(),
        EmergencyType::Choking,
    );

    let result4 = detector4.detect_role(&context4).await?;
    info!("✅ Role detected: {:?} via {:?} (confidence: {:.2})", 
          result4.role, result4.method, result4.confidence);

    // Test 5: Different Emergency Types
    info!("Test 5: Different Emergency Types");
    let test_cases = vec![
        ("I'm having a heart attack!", EmergencyType::HeartAttack),
        ("My baby is choking!", EmergencyType::Choking),
        ("Help me, I'm bleeding!", EmergencyType::Bleeding),
        ("Someone needs help!", EmergencyType::Unconscious),
    ];

    for (phrase, emergency_type) in test_cases {
        let context = EmergencyContext::new(phrase.to_string(), emergency_type.clone());
        let result = detector.detect_role(&context).await?;
        info!("✅ {:?}: {:?} via {:?} (confidence: {:.2})", 
              emergency_type, result.role, result.method, result.confidence);
    }

    info!("🎉 All role detection tests completed successfully!");
    info!("=====================");

    Ok(())
} 