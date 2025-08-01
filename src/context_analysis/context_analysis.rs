//! Main Context Analysis System
//! 
//! Combines stage detection and guidance generation to provide
//! context-appropriate emergency instructions.

use crate::types::{EmergencyType, EmergencyStage, EmergencyGuidance, ContextClues, EmergencyAnalysis};
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use super::stage_detection::StageDetector;
use super::guidance_generation::GuidanceGenerator;

/// Main context analyzer for emergency situations
pub struct ContextAnalyzer {
    stage_detector: StageDetector,
    guidance_generator: GuidanceGenerator,
}

impl ContextAnalyzer {
    pub fn new() -> Self {
        Self {
            stage_detector: StageDetector::new(),
            guidance_generator: GuidanceGenerator::new(),
        }
    }

    /// Analyze emergency context and provide appropriate guidance
    pub async fn analyze_emergency(
        &self,
        emergency_type: EmergencyType,
        clues: &ContextClues,
    ) -> Result<EmergencyAnalysis, AppError> {
        info!("Analyzing emergency context for {:?}", emergency_type);
        
        // 1. Detect current emergency stage
        let stage = self.stage_detector.detect_stage(emergency_type.clone(), clues).await?;
        
        // 2. Generate context-appropriate guidance
        let guidance = self.guidance_generator.generate_guidance(
            emergency_type.clone(),
            stage.clone(),
            clues,
        ).await?;
        
        // 3. Calculate confidence in analysis
        let confidence = self.calculate_analysis_confidence(clues, &stage);
        
        Ok(EmergencyAnalysis {
            stage,
            guidance,
            confidence,
            context_clues: clues.clone(),
        })
    }

    /// Calculate confidence in the analysis
    fn calculate_analysis_confidence(&self, clues: &ContextClues, stage: &EmergencyStage) -> f32 {
        let mut confidence = 0.0;
        let mut factors = 0;

        // Phrase analysis confidence
        if !clues.user_phrase.is_empty() {
            confidence += 0.3;
            factors += 1;
        }

        // Location context confidence
        if clues.location_context.is_some() {
            confidence += 0.2;
            factors += 1;
        }

        // Actions taken confidence
        if !clues.actions_taken.is_empty() {
            confidence += 0.3;
            factors += 1;
        }

        // Victim status confidence
        if clues.victim_status.is_some() {
            confidence += 0.4;
            factors += 1;
        }

        // Stage-specific confidence adjustments
        match stage {
            EmergencyStage::VictimExtracted => confidence += 0.1,
            EmergencyStage::Unconscious => confidence += 0.2,
            EmergencyStage::ServicesEnRoute => confidence += 0.1,
            _ => {}
        }

        if factors > 0 {
            (confidence / factors as f32).min(1.0)
        } else {
            0.5 // Default confidence
        }
    }

    /// Create context clues from user input
    pub fn create_context_clues(
        user_phrase: String,
        emergency_type: EmergencyType,
        location_context: Option<crate::types::LocationContext>,
        actions_taken: Vec<crate::types::EmergencyAction>,
        victim_status: Option<crate::types::VictimStatus>,
    ) -> ContextClues {
        ContextClues {
            user_phrase,
            location_context,
            time_elapsed: None, // Will be set by emergency handler
            victim_status,
            environment: crate::types::EnvironmentContext {
                weather_conditions: crate::types::WeatherConditions::Clear,
                crowd_present: false,
                professional_help_available: false,
                emergency_equipment_available: false,
                accessibility_issues: vec![],
            },
            actions_taken,
        }
    }

    /// Example analysis for "drowning help" when person is out of water
    pub async fn example_drowning_post_extraction() -> Result<EmergencyAnalysis, AppError> {
        let analyzer = ContextAnalyzer::new();
        
        let clues = ContextClues {
            user_phrase: "drowning help out of water".to_string(),
            location_context: Some(crate::types::LocationContext {
                location_type: crate::types::LocationType::Beach,
                coordinates: Some((34.0522, -118.2437)),
                nearby_landmarks: vec!["Santa Monica Beach".to_string()],
            }),
            time_elapsed: Some(std::time::Duration::from_secs(120)), // 2 minutes
            victim_status: Some(crate::types::VictimStatus {
                is_conscious: true,
                is_breathing: true,
                has_pulse: true,
                visible_injuries: vec![],
                estimated_age: Some(25),
                estimated_condition: crate::types::VictimCondition::Stable,
            }),
            environment: crate::types::EnvironmentContext {
                weather_conditions: crate::types::WeatherConditions::Clear,
                crowd_present: true,
                professional_help_available: false,
                emergency_equipment_available: false,
                accessibility_issues: vec![],
            },
            actions_taken: vec![
                crate::types::EmergencyAction::ExtractedVictim,
                crate::types::EmergencyAction::Called911,
            ],
        };

        analyzer.analyze_emergency(EmergencyType::Drowning, &clues).await
    }

    /// Example analysis for "choking help" when object is dislodged
    pub async fn example_choking_post_extraction() -> Result<EmergencyAnalysis, AppError> {
        let analyzer = ContextAnalyzer::new();
        
        let clues = ContextClues {
            user_phrase: "choking help object out".to_string(),
            location_context: Some(crate::types::LocationContext {
                location_type: crate::types::LocationType::Home,
                coordinates: None,
                nearby_landmarks: vec![],
            }),
            time_elapsed: Some(std::time::Duration::from_secs(60)), // 1 minute
            victim_status: Some(crate::types::VictimStatus {
                is_conscious: true,
                is_breathing: true,
                has_pulse: true,
                visible_injuries: vec![],
                estimated_age: Some(5),
                estimated_condition: crate::types::VictimCondition::Stable,
            }),
            environment: crate::types::EnvironmentContext {
                weather_conditions: crate::types::WeatherConditions::Clear,
                crowd_present: false,
                professional_help_available: false,
                emergency_equipment_available: false,
                accessibility_issues: vec![],
            },
            actions_taken: vec![
                crate::types::EmergencyAction::ExtractedVictim,
                crate::types::EmergencyAction::AppliedFirstAid,
            ],
        };

        analyzer.analyze_emergency(EmergencyType::Choking, &clues).await
    }

    /// Example analysis for "heart attack" initial response
    pub async fn example_heart_attack_initial() -> Result<EmergencyAnalysis, AppError> {
        let analyzer = ContextAnalyzer::new();
        
        let clues = ContextClues {
            user_phrase: "heart attack help".to_string(),
            location_context: Some(crate::types::LocationContext {
                location_type: crate::types::LocationType::Public,
                coordinates: Some((40.7128, -74.0060)), // NYC
                nearby_landmarks: vec!["Central Park".to_string()],
            }),
            time_elapsed: Some(std::time::Duration::from_secs(30)), // 30 seconds
            victim_status: Some(crate::types::VictimStatus {
                is_conscious: true,
                is_breathing: true,
                has_pulse: true,
                visible_injuries: vec![],
                estimated_age: Some(65),
                estimated_condition: crate::types::VictimCondition::Critical,
            }),
            environment: crate::types::EnvironmentContext {
                weather_conditions: crate::types::WeatherConditions::Clear,
                crowd_present: true,
                professional_help_available: true,
                emergency_equipment_available: false,
                accessibility_issues: vec![],
            },
            actions_taken: vec![
                crate::types::EmergencyAction::Called911,
            ],
        };

        analyzer.analyze_emergency(EmergencyType::Bleeding, &clues).await
    }
}

/// Context analysis result with detailed information
#[derive(Debug, Clone)]
pub struct ContextAnalysisResult {
    pub analysis: EmergencyAnalysis,
    pub stage_detection_confidence: f32,
    pub guidance_appropriateness: f32,
    pub time_saved_seconds: u32,
    pub skipped_steps: Vec<String>,
}

impl ContextAnalysisResult {
    /// Calculate time saved by skipping irrelevant steps
    pub fn calculate_time_saved(&self) -> u32 {
        if self.analysis.guidance.skip_basic_steps {
            // Estimate 30-60 seconds saved by skipping basic instructions
            45
        } else {
            0
        }
    }

    /// Get list of steps that were skipped
    pub fn get_skipped_steps(&self) -> Vec<String> {
        if self.analysis.guidance.skip_basic_steps {
            vec![
                "Scene assessment".to_string(),
                "Basic safety instructions".to_string(),
                "Initial rescue preparation".to_string(),
            ]
        } else {
            vec![]
        }
    }

    /// Get priority actions for immediate execution
    pub fn get_priority_actions(&self) -> Vec<String> {
        self.analysis.guidance.priority_actions.clone()
    }

    /// Get detailed instructions for the current stage
    pub fn get_detailed_instructions(&self) -> Vec<String> {
        self.analysis.guidance.instructions.clone()
    }
}

/// Performance metrics for context analysis
#[derive(Debug, Clone)]
pub struct ContextAnalysisMetrics {
    pub total_analyses: u32,
    pub average_confidence: f32,
    pub average_time_saved: f32,
    pub stage_detection_accuracy: f32,
    pub guidance_appropriateness: f32,
}

impl ContextAnalysisMetrics {
    pub fn new() -> Self {
        Self {
            total_analyses: 0,
            average_confidence: 0.0,
            average_time_saved: 0.0,
            stage_detection_accuracy: 0.0,
            guidance_appropriateness: 0.0,
        }
    }

    /// Update metrics with new analysis result
    pub fn update(&mut self, result: &ContextAnalysisResult) {
        self.total_analyses += 1;
        
        // Update running averages
        let total = self.total_analyses as f32;
        self.average_confidence = (self.average_confidence * (total - 1.0) + result.analysis.confidence) / total;
        self.average_time_saved = (self.average_time_saved * (total - 1.0) + result.time_saved_seconds as f32) / total;
    }
} 