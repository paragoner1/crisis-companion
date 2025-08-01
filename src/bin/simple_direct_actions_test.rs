use crisis_companion::{
    context_analysis::guidance_generation::GuidanceGenerator,
    types::{EmergencyType, EmergencyStage, ContextClues, EnvironmentContext, WeatherConditions},
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

    info!("🚨 Simple Direct Actions Test");
    info!("Testing guidance generation for direct actions");
    info!("==========================================");

    let guidance_generator = GuidanceGenerator::new();
    
    // Test cases for direct actions
    let direct_actions = vec![
        ("CPR", "CPR instructions"),
        ("Heimlich", "Heimlich maneuver"),
        ("AED", "AED usage"),
        ("Tourniquet", "Tourniquet application"),
        ("EpiPen", "EpiPen administration"),
        ("Rescue Breathing", "Rescue breathing"),
        ("First Aid", "First aid"),
        ("FAST test", "FAST test for stroke"),
        ("Poison Control", "Poison control call"),
        ("Cool Burn", "Burn cooling"),
        ("Medical Alert", "Medical alert check"),
    ];

    for (phrase, description) in direct_actions {
        info!("Testing: {} - {}", phrase, description);
        
        let context_clues = ContextClues {
            user_phrase: phrase.to_string(),
            location_context: None,
            time_elapsed: None,
            victim_status: None,
            environment: EnvironmentContext {
                weather_conditions: WeatherConditions::Clear,
                crowd_present: false,
                professional_help_available: false,
                emergency_equipment_available: false,
                accessibility_issues: vec![],
            },
            actions_taken: vec![],
        };
        
        let guidance = guidance_generator.generate_guidance(
            EmergencyType::Unconscious, // Assuming a default emergency type for testing
            EmergencyStage::InitialDetection,
            &context_clues,
        ).await?;
        
        info!("✅ {} test completed", phrase);
        info!("   Stage: {:?}", guidance.stage);
        info!("   Instructions: {:?}", guidance.instructions);
        info!("   Skip basic steps: {}", guidance.skip_basic_steps);
        info!("   Focus on medical care: {}", guidance.focus_on_medical_care);
        info!("");
    }

    info!("🎉 All direct actions guidance tests completed successfully!");
    info!("==========================================");

    Ok(())
} 