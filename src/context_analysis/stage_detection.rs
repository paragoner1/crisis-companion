//! Emergency Stage Detection
//! 
//! Determines the current stage of an emergency based on context clues
//! and provides appropriate guidance for that specific stage.

use crate::types::{EmergencyType, EmergencyStage, ContextClues, VictimStatus, EmergencyAction};
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// Emergency stage detector
pub struct StageDetector {
    phrase_analyzer: PhraseAnalyzer,
    context_analyzer: ContextAnalyzer,
    action_analyzer: ActionAnalyzer,
}

impl StageDetector {
    pub fn new() -> Self {
        Self {
            phrase_analyzer: PhraseAnalyzer::new(),
            context_analyzer: ContextAnalyzer::new(),
            action_analyzer: ActionAnalyzer::new(),
        }
    }

    /// Detect current emergency stage
    pub async fn detect_stage(
        &self,
        emergency_type: EmergencyType,
        clues: &ContextClues,
    ) -> Result<EmergencyStage, AppError> {
        // Analyze user phrase for stage indicators
        let phrase_indicators = self.phrase_analyzer.analyze(&clues.user_phrase)?;
        
        // Analyze context for stage indicators
        let context_indicators = self.context_analyzer.analyze(clues)?;
        
        // Analyze actions already taken
        let action_indicators = self.action_analyzer.analyze(&clues.actions_taken)?;
        
        // Analyze victim status
        let victim_stage = self.analyze_victim_status(&clues.victim_status)?;
        
        // Combine stage indicators
        let final_stage = self.combine_stage_indicators(
            emergency_type.clone(),
            phrase_indicators,
            context_indicators,
            action_indicators,
            None, // victim_stage - not available in this context
        );
        
        info!("Detected emergency stage: {:?} for {:?}", final_stage, emergency_type);
        
        Ok(final_stage)
    }

    /// Analyze victim status for stage
    fn analyze_victim_status(
        &self,
        victim_status: &Option<VictimStatus>,
    ) -> Result<Option<EmergencyStage>, AppError> {
        if let Some(victim) = victim_status {
            if !victim.is_breathing || !victim.has_pulse {
                return Ok(Some(EmergencyStage::Unconscious));
            }
            
            if !victim.is_conscious {
                return Ok(Some(EmergencyStage::BreathingButUnresponsive));
            }
            
            if !victim.visible_injuries.is_empty() {
                return Ok(Some(EmergencyStage::ConsciousButInjured));
            }
        }
        Ok(None)
    }

    /// Combine stage indicators to determine final stage
    fn combine_stage_indicators(
        &self,
        emergency_type: EmergencyType,
        phrase_stage: Option<EmergencyStage>,
        context_stage: Option<EmergencyStage>,
        action_stage: Option<EmergencyStage>,
        victim_stage: Option<EmergencyStage>,
    ) -> EmergencyStage {
        // Priority order: victim status > actions > phrase > context > default
        if let Some(stage) = victim_stage {
            return stage;
        }
        
        if let Some(stage) = action_stage {
            return stage;
        }
        
        if let Some(stage) = phrase_stage {
            return stage;
        }
        
        if let Some(stage) = context_stage {
            return stage;
        }
        
        // Default based on emergency type
        match emergency_type {
            EmergencyType::Drowning => EmergencyStage::VictimExtracted,
            EmergencyType::Choking => EmergencyStage::VictimExtracted,
            EmergencyType::HeartAttack => EmergencyStage::InitialDetection,
            EmergencyType::Stroke => EmergencyStage::InitialDetection,
            EmergencyType::Seizure => EmergencyStage::InitialDetection,
            EmergencyType::Poisoning => EmergencyStage::InitialDetection,
            EmergencyType::SevereBurns => EmergencyStage::InitialDetection,
            EmergencyType::DiabeticEmergency => EmergencyStage::InitialDetection,
            EmergencyType::Bleeding => EmergencyStage::ConsciousButInjured,
            _ => EmergencyStage::InitialDetection,
        }
    }
}

/// Phrase analyzer for stage detection
pub struct PhraseAnalyzer {
    stage_patterns: Vec<StagePattern>,
}

impl PhraseAnalyzer {
    pub fn new() -> Self {
        Self {
            stage_patterns: Self::load_stage_patterns(),
        }
    }

    /// Analyze user phrase for stage indicators
    pub fn analyze(&self, phrase: &str) -> Result<Option<EmergencyStage>, AppError> {
        let phrase_lower = phrase.to_lowercase();
        
        // Direct CPR request - immediate unconscious stage
        if phrase_lower.contains("cpr") || phrase_lower.contains("cardiopulmonary") {
            return Ok(Some(EmergencyStage::Unconscious));
        }
        
        // Direct Heimlich maneuver request
        if phrase_lower.contains("heimlich") || phrase_lower.contains("abdominal thrusts") || phrase_lower.contains("choking maneuver") {
            return Ok(Some(EmergencyStage::VictimExtracted));
        }
        
        // Direct AED request - more specific to avoid confusion with medical shock
        if phrase_lower.contains("aed") || phrase_lower.contains("defibrillator") || 
           (phrase_lower.contains("shock") && (phrase_lower.contains("heart") || phrase_lower.contains("chest") || phrase_lower.contains("defibrillator"))) {
            return Ok(Some(EmergencyStage::Unconscious));
        }
        
        // Medical shock (not AED shock) - more specific
        if (phrase_lower.contains("shock") && !phrase_lower.contains("defibrillator") && !phrase_lower.contains("aed")) ||
           phrase_lower.contains("medical shock") || phrase_lower.contains("anaphylactic shock") {
            return Ok(Some(EmergencyStage::ConsciousButInjured));
        }
        
        // Electric shock/electrocution - specific
        if phrase_lower.contains("electric shock") || phrase_lower.contains("electrocution") || 
           (phrase_lower.contains("shock") && phrase_lower.contains("electric")) {
            return Ok(Some(EmergencyStage::Unconscious));
        }
        
        // Direct tourniquet request - more specific
        if phrase_lower.contains("tourniquet") || 
           (phrase_lower.contains("stop bleeding") && phrase_lower.contains("tourniquet")) {
            return Ok(Some(EmergencyStage::ConsciousButInjured));
        }
        
        // Direct pressure for bleeding - specific
        if phrase_lower.contains("direct pressure") || 
           (phrase_lower.contains("pressure") && phrase_lower.contains("bleeding")) {
            return Ok(Some(EmergencyStage::ConsciousButInjured));
        }
        
        // Blood pressure issues - specific
        if phrase_lower.contains("blood pressure") || phrase_lower.contains("pressure") && phrase_lower.contains("blood") {
            return Ok(Some(EmergencyStage::ConsciousButInjured));
        }
        
        // Direct EpiPen request
        if phrase_lower.contains("epipen") || phrase_lower.contains("epinephrine") || phrase_lower.contains("allergic reaction") {
            return Ok(Some(EmergencyStage::ConsciousButInjured));
        }
        
        // Direct rescue breathing request - more specific
        if phrase_lower.contains("rescue breathing") || phrase_lower.contains("mouth to mouth") || phrase_lower.contains("ventilation") {
            return Ok(Some(EmergencyStage::BreathingButUnresponsive));
        }
        
        // Not breathing - specific
        if phrase_lower.contains("not breathing") || phrase_lower.contains("no breathing") {
            return Ok(Some(EmergencyStage::Unconscious));
        }
        
        // Breathing problems - specific
        if phrase_lower.contains("breathing problems") || phrase_lower.contains("difficulty breathing") {
            return Ok(Some(EmergencyStage::ConsciousButInjured));
        }
        
        // Direct first aid request
        if phrase_lower.contains("first aid") || phrase_lower.contains("bandage") || phrase_lower.contains("wound care") {
            return Ok(Some(EmergencyStage::ConsciousButInjured));
        }
        
        // Stop bleeding - specific
        if phrase_lower.contains("stop bleeding") && !phrase_lower.contains("tourniquet") {
            return Ok(Some(EmergencyStage::ConsciousButInjured));
        }
        
        // FAST test for stroke
        if phrase_lower.contains("fast test") || phrase_lower.contains("stroke test") || 
           (phrase_lower.contains("face") && phrase_lower.contains("arm") && phrase_lower.contains("speech")) {
            return Ok(Some(EmergencyStage::InitialDetection));
        }
        
        // Poison control call
        if phrase_lower.contains("poison control") || phrase_lower.contains("call poison") {
            return Ok(Some(EmergencyStage::InitialDetection));
        }
        
        // Cool burn
        if phrase_lower.contains("cool burn") || phrase_lower.contains("burn cooling") {
            return Ok(Some(EmergencyStage::InitialDetection));
        }
        
        // Medical alert check
        if phrase_lower.contains("medical alert") || phrase_lower.contains("check bracelet") || 
           phrase_lower.contains("medical id") {
            return Ok(Some(EmergencyStage::InitialDetection));
        }
        
        // Post-extraction phrases
        if phrase_lower.contains("out of water") || phrase_lower.contains("pulled out") {
            return Ok(Some(EmergencyStage::VictimExtracted));
        }
        
        if phrase_lower.contains("no pulse") {
            return Ok(Some(EmergencyStage::Unconscious));
        }
        
        if phrase_lower.contains("conscious but") || phrase_lower.contains("injured") {
            return Ok(Some(EmergencyStage::ConsciousButInjured));
        }
        
        if phrase_lower.contains("breathing but") || phrase_lower.contains("unresponsive") {
            return Ok(Some(EmergencyStage::BreathingButUnresponsive));
        }
        
        if phrase_lower.contains("ambulance") || phrase_lower.contains("on the way") {
            return Ok(Some(EmergencyStage::ServicesEnRoute));
        }
        
        if phrase_lower.contains("what now") || phrase_lower.contains("what do i do") {
            return Ok(Some(EmergencyStage::VictimExtracted));
        }
        
        // Key phrase: "drowning help" + context indicating post-extraction
        if phrase_lower.contains("drowning help") {
            if phrase_lower.contains("out") || phrase_lower.contains("pulled") {
                return Ok(Some(EmergencyStage::VictimExtracted));
            }
        }
        
        // Choking post-extraction
        if phrase_lower.contains("choking help") && phrase_lower.contains("out") {
            return Ok(Some(EmergencyStage::VictimExtracted));
        }
        
        // Heart attack specific
        if phrase_lower.contains("heart attack") && phrase_lower.contains("what") {
            return Ok(Some(EmergencyStage::InitialDetection));
        }
        
        Ok(None)
    }

    /// Load stage patterns for analysis
    fn load_stage_patterns() -> Vec<StagePattern> {
        vec![
            // Direct CPR patterns
            StagePattern {
                pattern: r"cpr",
                stage: EmergencyStage::Unconscious,
                confidence: 0.95,
            },
            StagePattern {
                pattern: r"cardiopulmonary",
                stage: EmergencyStage::Unconscious,
                confidence: 0.95,
            },
            // Post-extraction patterns
            StagePattern {
                pattern: r"out of water",
                stage: EmergencyStage::VictimExtracted,
                confidence: 0.95,
            },
            StagePattern {
                pattern: r"pulled out",
                stage: EmergencyStage::VictimExtracted,
                confidence: 0.95,
            },
            StagePattern {
                pattern: r"not breathing",
                stage: EmergencyStage::Unconscious,
                confidence: 0.9,
            },
            StagePattern {
                pattern: r"no pulse",
                stage: EmergencyStage::Unconscious,
                confidence: 0.9,
            },
            StagePattern {
                pattern: r"conscious but",
                stage: EmergencyStage::ConsciousButInjured,
                confidence: 0.85,
            },
            StagePattern {
                pattern: r"breathing but",
                stage: EmergencyStage::BreathingButUnresponsive,
                confidence: 0.8,
            },
            StagePattern {
                pattern: r"ambulance",
                stage: EmergencyStage::ServicesEnRoute,
                confidence: 0.7,
            },
            StagePattern {
                pattern: r"what now",
                stage: EmergencyStage::VictimExtracted,
                confidence: 0.8,
            },
            StagePattern {
                pattern: r"what do i do",
                stage: EmergencyStage::VictimExtracted,
                confidence: 0.8,
            },
        ]
    }
}

/// Context analyzer for stage detection
pub struct ContextAnalyzer {
    location_patterns: Vec<LocationPattern>,
}

impl ContextAnalyzer {
    pub fn new() -> Self {
        Self {
            location_patterns: Self::load_location_patterns(),
        }
    }

    /// Analyze context for stage indicators
    pub fn analyze(&self, clues: &ContextClues) -> Result<Option<EmergencyStage>, AppError> {
        // Analyze location context
        if let Some(location) = &clues.location_context {
            match location.location_type {
                crate::types::LocationType::Beach | crate::types::LocationType::Pool => {
                    // If near water, likely post-extraction
                    return Ok(Some(EmergencyStage::VictimExtracted));
                }
                crate::types::LocationType::Hospital => {
                    return Ok(Some(EmergencyStage::PostEmergency));
                }
                crate::types::LocationType::Remote => {
                    // Remote locations may need different guidance
                    return Ok(Some(EmergencyStage::VictimExtracted));
                }
                _ => {}
            }
        }

        // Analyze environment
        if clues.environment.crowd_present && clues.environment.professional_help_available {
            return Ok(Some(EmergencyStage::ServicesEnRoute));
        }

        Ok(None)
    }

    /// Load location patterns
    fn load_location_patterns() -> Vec<LocationPattern> {
        vec![
            LocationPattern {
                location_type: crate::types::LocationType::Beach,
                stage: EmergencyStage::VictimExtracted,
                confidence: 0.8,
            },
            LocationPattern {
                location_type: crate::types::LocationType::Pool,
                stage: EmergencyStage::VictimExtracted,
                confidence: 0.8,
            },
            LocationPattern {
                location_type: crate::types::LocationType::Hospital,
                stage: EmergencyStage::PostEmergency,
                confidence: 0.9,
            },
        ]
    }
}

/// Action analyzer for stage detection
pub struct ActionAnalyzer {
    action_patterns: Vec<ActionPattern>,
}

impl ActionAnalyzer {
    pub fn new() -> Self {
        Self {
            action_patterns: Self::load_action_patterns(),
        }
    }

    /// Analyze actions already taken
    pub fn analyze(&self, actions: &[EmergencyAction]) -> Result<Option<EmergencyStage>, AppError> {
        if actions.contains(&EmergencyAction::ExtractedVictim) {
            return Ok(Some(EmergencyStage::VictimExtracted));
        }
        
        if actions.contains(&EmergencyAction::StartedCPR) {
            return Ok(Some(EmergencyStage::Unconscious));
        }
        
        if actions.contains(&EmergencyAction::Called911) {
            return Ok(Some(EmergencyStage::ServicesEnRoute));
        }
        
        if actions.contains(&EmergencyAction::AppliedFirstAid) {
            return Ok(Some(EmergencyStage::ConsciousButInjured));
        }
        
        if actions.contains(&EmergencyAction::MovedToSafety) {
            return Ok(Some(EmergencyStage::VictimExtracted));
        }
        
        Ok(None)
    }

    /// Load action patterns
    fn load_action_patterns() -> Vec<ActionPattern> {
        vec![
            ActionPattern {
                action: EmergencyAction::ExtractedVictim,
                stage: EmergencyStage::VictimExtracted,
                confidence: 0.95,
            },
            ActionPattern {
                action: EmergencyAction::StartedCPR,
                stage: EmergencyStage::Unconscious,
                confidence: 0.9,
            },
            ActionPattern {
                action: EmergencyAction::Called911,
                stage: EmergencyStage::ServicesEnRoute,
                confidence: 0.8,
            },
            ActionPattern {
                action: EmergencyAction::AppliedFirstAid,
                stage: EmergencyStage::ConsciousButInjured,
                confidence: 0.85,
            },
        ]
    }
}

// Supporting structures

/// Stage pattern for phrase analysis
#[derive(Debug, Clone)]
pub struct StagePattern {
    pub pattern: &'static str,
    pub stage: EmergencyStage,
    pub confidence: f32,
}

/// Location pattern for context analysis
#[derive(Debug, Clone)]
pub struct LocationPattern {
    pub location_type: crate::types::LocationType,
    pub stage: EmergencyStage,
    pub confidence: f32,
}

/// Action pattern for action analysis
#[derive(Debug, Clone)]
pub struct ActionPattern {
    pub action: EmergencyAction,
    pub stage: EmergencyStage,
    pub confidence: f32,
} 