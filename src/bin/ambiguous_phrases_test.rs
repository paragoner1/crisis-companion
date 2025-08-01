use crisis_companion::{
    context_analysis::guidance_generation::GuidanceGenerator,
    types::{EmergencyType, EmergencyStage, ContextClues, EnvironmentContext, WeatherConditions},
};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚨 Ambiguous Phrases Test");
    info!("Testing context-aware phrase detection");
    info!("==========================================");

    let guidance_generator = GuidanceGenerator::new();
    
    // Test ambiguous phrases with context
    let ambiguous_tests = vec![
        // Shock tests
        ("shock", "AED shock", "AED usage"),
        ("shock", "medical shock", "Medical shock treatment"),
        ("shock", "electric shock", "Electric shock treatment"),
        ("shock", "anaphylactic shock", "Anaphylactic shock treatment"),
        
        // Pressure tests
        ("pressure", "direct pressure", "Direct pressure for bleeding"),
        ("pressure", "blood pressure", "Blood pressure issues"),
        ("pressure", "pressure bleeding", "Direct pressure for bleeding"),
        
        // Stop tests
        ("stop", "stop bleeding", "Stop bleeding procedures"),
        
        // Breathing tests
        ("breathing", "not breathing", "Not breathing - CPR needed"),
        ("breathing", "breathing problems", "Breathing problems treatment"),
        ("breathing", "difficulty breathing", "Breathing problems treatment"),
        ("breathing", "rescue breathing", "Rescue breathing procedures"),
    ];

    for (ambiguous_word, specific_phrase, description) in ambiguous_tests {
        info!("Testing: '{}' vs '{}' - {}", ambiguous_word, specific_phrase, description);
        
        let context_clues = ContextClues {
            user_phrase: specific_phrase.to_string(),
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
            EmergencyType::Unconscious, // Default type
            EmergencyStage::InitialDetection,
            &context_clues,
        ).await?;
        
        info!("✅ '{}' test completed", specific_phrase);
        info!("   Stage: {:?}", guidance.stage);
        info!("   Instructions: {:?}", guidance.instructions);
        info!("   Skip basic steps: {}", guidance.skip_basic_steps);
        info!("   Focus on medical care: {}", guidance.focus_on_medical_care);
        info!("");
    }

    info!("🎉 All ambiguous phrase tests completed successfully!");
    info!("==========================================");

    Ok(())
} 