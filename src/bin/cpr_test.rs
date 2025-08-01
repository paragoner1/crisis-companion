use crisis_companion::{
    app::CrisisCompanionApp,
    types::{EmergencyType, UserRole},
    error::AppError,
};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚨 Direct Actions Functionality Test");
    info!("Testing all direct action phrase detection");
    info!("==========================================");

    // Initialize the application
    let app = CrisisCompanionApp::new("config.toml").await?;
    
    // Test all direct actions
    let direct_actions = vec![
        ("CPR", EmergencyType::Unconscious, "CPR instructions"),
        ("Heimlich", EmergencyType::Choking, "Heimlich maneuver"),
        ("AED", EmergencyType::Unconscious, "AED usage"),
        ("Tourniquet", EmergencyType::Bleeding, "Tourniquet application"),
        ("EpiPen", EmergencyType::AllergicReaction, "EpiPen administration"),
        ("Rescue Breathing", EmergencyType::Unconscious, "Rescue breathing"),
        ("First Aid", EmergencyType::Trauma, "First aid"),
    ];

    for (phrase, emergency_type, description) in direct_actions {
        info!("Testing: {} - {}", phrase, description);
        
        let result = app.handle_emergency_trigger(
            phrase,
            emergency_type,
            UserRole::Bystander,
        ).await?;
        
        info!("✅ {} test completed", phrase);
        info!("   Stage: {:?}", result.analysis.stage);
        info!("   Instructions: {:?}", result.analysis.guidance.instructions);
        info!("   Skip basic steps: {}", result.analysis.guidance.skip_basic_steps);
        info!("   Focus on medical care: {}", result.analysis.guidance.focus_on_medical_care);
        info!("");
    }
    
    // Stop the application
    app.stop().await?;

    info!("🎉 All direct actions functionality tests completed successfully!");
    info!("==========================================");

    Ok(())
} 