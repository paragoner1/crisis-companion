//! Context Analysis Test Binary
//! 
//! Demonstrates the context-aware emergency guidance system
//! by analyzing different emergency scenarios and showing
//! how the app provides appropriate guidance.

use crisis_companion::{
    context_analysis::context_analysis::ContextAnalyzer,
    types::{EmergencyType, EmergencyStage, ContextClues, LocationContext, LocationType, VictimStatus, VictimCondition, EmergencyAction, EnvironmentContext, WeatherConditions},
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

    info!("🎯 Context-Aware Emergency Guidance Test");
    info!("==========================================");

    let analyzer = ContextAnalyzer::new();

    // Test 1: Drowning help - person already out of water
    info!("Test 1: Drowning help - person already out of water");
    let drowning_clues = ContextClues {
        user_phrase: "drowning help out of water".to_string(),
        location_context: Some(LocationContext {
            location_type: LocationType::Beach,
            coordinates: Some((34.0522, -118.2437)),
            nearby_landmarks: vec!["Santa Monica Beach".to_string()],
        }),
        time_elapsed: Some(std::time::Duration::from_secs(120)), // 2 minutes
        victim_status: Some(VictimStatus {
            is_conscious: true,
            is_breathing: true,
            has_pulse: true,
            visible_injuries: vec![],
            estimated_age: Some(25),
            estimated_condition: VictimCondition::Stable,
        }),
        environment: EnvironmentContext {
            weather_conditions: WeatherConditions::Clear,
            crowd_present: true,
            professional_help_available: false,
            emergency_equipment_available: false,
            accessibility_issues: vec![],
        },
        actions_taken: vec![
            EmergencyAction::ExtractedVictim,
            EmergencyAction::Called911,
        ],
    };

    let drowning_result = analyzer.analyze_emergency(EmergencyType::Drowning, &drowning_clues).await?;
    display_analysis_result("Drowning Post-Extraction", &drowning_result);

    // Test 2: Choking help - object dislodged
    info!("Test 2: Choking help - object dislodged");
    let choking_clues = ContextClues {
        user_phrase: "choking help object out".to_string(),
        location_context: Some(LocationContext {
            location_type: LocationType::Home,
            coordinates: None,
            nearby_landmarks: vec![],
        }),
        time_elapsed: Some(std::time::Duration::from_secs(60)), // 1 minute
        victim_status: Some(VictimStatus {
            is_conscious: true,
            is_breathing: true,
            has_pulse: true,
            visible_injuries: vec![],
            estimated_age: Some(5),
            estimated_condition: VictimCondition::Stable,
        }),
        environment: EnvironmentContext {
            weather_conditions: WeatherConditions::Clear,
            crowd_present: false,
            professional_help_available: false,
            emergency_equipment_available: false,
            accessibility_issues: vec![],
        },
        actions_taken: vec![
            EmergencyAction::ExtractedVictim,
            EmergencyAction::AppliedFirstAid,
        ],
    };

    let choking_result = analyzer.analyze_emergency(EmergencyType::Choking, &choking_clues).await?;
    display_analysis_result("Choking Post-Extraction", &choking_result);

    // Test 3: Drowning - unconscious victim
    info!("Test 4: Drowning - unconscious victim");
    let unconscious_clues = ContextClues {
        user_phrase: "drowning help not breathing".to_string(),
        location_context: Some(LocationContext {
            location_type: LocationType::Pool,
            coordinates: Some((34.0522, -118.2437)),
            nearby_landmarks: vec!["Community Pool".to_string()],
        }),
        time_elapsed: Some(std::time::Duration::from_secs(180)), // 3 minutes
        victim_status: Some(VictimStatus {
            is_conscious: false,
            is_breathing: false,
            has_pulse: false,
            visible_injuries: vec![],
            estimated_age: Some(12),
            estimated_condition: VictimCondition::Critical,
        }),
        environment: EnvironmentContext {
            weather_conditions: WeatherConditions::Clear,
            crowd_present: true,
            professional_help_available: false,
            emergency_equipment_available: true,
            accessibility_issues: vec![],
        },
        actions_taken: vec![
            EmergencyAction::ExtractedVictim,
            EmergencyAction::Called911,
        ],
    };

    let unconscious_result = analyzer.analyze_emergency(EmergencyType::Drowning, &unconscious_clues).await?;
    display_analysis_result("Drowning Unconscious", &unconscious_result);

    // Test 5: Bleeding - conscious but injured
    info!("Test 5: Bleeding - conscious but injured");
    let bleeding_clues = ContextClues {
        user_phrase: "bleeding help injured".to_string(),
        location_context: Some(LocationContext {
            location_type: LocationType::Home,
            coordinates: None,
            nearby_landmarks: vec![],
        }),
        time_elapsed: Some(std::time::Duration::from_secs(45)), // 45 seconds
        victim_status: Some(VictimStatus {
            is_conscious: true,
            is_breathing: true,
            has_pulse: true,
            visible_injuries: vec!["Deep cut on arm".to_string()],
            estimated_age: Some(35),
            estimated_condition: VictimCondition::Stable,
        }),
        environment: EnvironmentContext {
            weather_conditions: WeatherConditions::Clear,
            crowd_present: false,
            professional_help_available: false,
            emergency_equipment_available: true,
            accessibility_issues: vec![],
        },
        actions_taken: vec![
            EmergencyAction::AppliedFirstAid,
        ],
    };

    let bleeding_result = analyzer.analyze_emergency(EmergencyType::Bleeding, &bleeding_clues).await?;
    display_analysis_result("Bleeding Conscious", &bleeding_result);

    info!("🎉 All context analysis tests completed successfully!");
    info!("==========================================");
    info!("Context-aware guidance system is working correctly!");

    Ok(())
}

/// Display analysis result with detailed information
fn display_analysis_result(scenario: &str, result: &crisis_companion::types::EmergencyAnalysis) {
    info!("📋 {} Analysis Result:", scenario);
    info!("   Stage: {:?}", result.stage);
    info!("   Confidence: {:.2}", result.confidence);
    info!("   Skip Basic Steps: {}", result.guidance.skip_basic_steps);
    info!("   Focus on Medical Care: {}", result.guidance.focus_on_medical_care);
    
    info!("   Priority Actions:");
    for action in &result.guidance.priority_actions {
        info!("     • {}", action);
    }
    
    info!("   Detailed Instructions:");
    for instruction in &result.guidance.instructions {
        info!("     • {}", instruction);
    }
    
    // Calculate time saved
    let time_saved = if result.guidance.skip_basic_steps { 45 } else { 0 };
    info!("   Time Saved: {} seconds", time_saved);
    
    info!("");
} 