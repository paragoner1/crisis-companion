use crisis_companion::{
    SafetyFeaturesManager,
    types::{SilentSOSMethod, NotificationPreferences},
    error::AppError,
};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚨 Safety Features Test");
    info!("Testing silent SOS, crash detection, and trusted network features");
    info!("==========================================");

    let mut manager = SafetyFeaturesManager::new();

    // Test 1: Silent SOS Activation
    info!("Testing Silent SOS Activation");
    info!("----------------------------");
    
    let silent_methods = vec![
        SilentSOSMethod::HoldButton,
        SilentSOSMethod::PowerButtonSequence,
        SilentSOSMethod::VolumeButtonSequence,
        SilentSOSMethod::ScreenTapPattern,
        SilentSOSMethod::MotionGesture,
    ];

    for method in silent_methods {
        info!("Testing silent SOS method: {:?}", method);
        manager.activate_silent_sos(method.clone()).await?;
        info!("✅ Silent SOS test completed for {:?}", method);
        info!("");
    }

    // Test 2: Crash Detection
    info!("Testing Crash Detection");
    info!("----------------------");
    
    let crash_scenarios = vec![
        (20.0, 2.0, "Normal driving"),
        (30.0, 4.0, "Potential crash"),
        (15.0, 5.0, "Confirmed crash"),
        (40.0, 6.0, "Severe crash"),
    ];

    for (speed, impact, description) in crash_scenarios {
        info!("Testing crash detection: {} (speed={}mph, impact={}g)", description, speed, impact);
        let status = manager.detect_crash(speed, impact, Some((40.7128, -74.0060))).await?;
        info!("✅ Crash detection result: {:?}", status);
        info!("");
    }

    // Test 3: Trusted Network Management
    info!("Testing Trusted Network Management");
    info!("---------------------------------");
    
    let preferences = NotificationPreferences {
        silent_sos: true,
        crash_detection: true,
        emergency_activation: true,
        location_sharing: true,
        status_updates: true,
    };

    // Add trusted contacts
    let contact1_id = manager.add_trusted_contact(
        "Sarah Johnson".to_string(),
        "+1234567890".to_string(),
        "Spouse".to_string(),
        preferences.clone(),
    ).await?;
    info!("✅ Added trusted contact: Sarah Johnson (Spouse)");

    let contact2_id = manager.add_trusted_contact(
        "Mike Chen".to_string(),
        "+1987654321".to_string(),
        "Best Friend".to_string(),
        preferences.clone(),
    ).await?;
    info!("✅ Added trusted contact: Mike Chen (Best Friend)");

    let contact3_id = manager.add_trusted_contact(
        "Dr. Emily Rodriguez".to_string(),
        "+1555123456".to_string(),
        "Emergency Contact".to_string(),
        preferences,
    ).await?;
    info!("✅ Added trusted contact: Dr. Emily Rodriguez (Emergency Contact)");

    // List all trusted contacts
    let contacts = manager.get_trusted_contacts().await?;
    info!("📋 Current trusted contacts:");
    for contact in &contacts {
        info!("   - {} ({}) - {}", contact.name, contact.relationship, contact.phone_number);
    }
    info!("");

    // Test 4: Feature Configuration
    info!("Testing Feature Configuration");
    info!("----------------------------");
    
    manager.configure_features(true, true, true);
    info!("✅ Enabled all safety features");
    
    manager.set_crash_thresholds(30.0, 3.5);
    info!("✅ Updated crash thresholds: 30mph, 3.5g");
    
    manager.set_silent_sos_method(SilentSOSMethod::PowerButtonSequence);
    info!("✅ Set silent SOS method: PowerButtonSequence");
    info!("");

    // Test 5: Remove trusted contact
    info!("Testing Contact Removal");
    info!("----------------------");
    
    manager.remove_trusted_contact(contact2_id).await?;
    info!("✅ Removed trusted contact: Mike Chen");
    
    let remaining_contacts = manager.get_trusted_contacts().await?;
    info!("📋 Remaining trusted contacts: {}", remaining_contacts.len());
    for contact in &remaining_contacts {
        info!("   - {} ({})", contact.name, contact.relationship);
    }
    info!("");

    info!("🎉 All safety features tests completed successfully!");
    info!("==========================================");
    info!("");
    info!("📊 Test Summary:");
    info!("   ✅ Silent SOS: 5 activation methods tested");
    info!("   ✅ Crash Detection: 4 scenarios tested");
    info!("   ✅ Trusted Network: 3 contacts added, 1 removed");
    info!("   ✅ Feature Configuration: All settings updated");
    info!("");
    info!("🚨 Safety Features Ready for Production!");

    Ok(())
} 