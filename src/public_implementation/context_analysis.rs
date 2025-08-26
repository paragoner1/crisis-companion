//! Context Analysis Module
//! 
//! Provides intelligent emergency response analysis based on user input,
//! emergency context, and situational awareness.

use crate::error::AppResult;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EmergencyContext {
    pub emergency_type: String,
    pub current_step: u32,
    pub context_flags: Vec<String>,
    pub user_input: String,
    pub severity_level: EmergencySeverity,
    pub time_critical: bool,
    pub requires_immediate_action: bool,
    pub location_known: bool,
    pub help_available: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmergencySeverity {
    Low,
    Medium,
    High,
    Critical,
    LifeThreatening,
}

pub struct ContextAnalyzer {
    emergency_types: HashMap<String, EmergencyType>,
    context_patterns: HashMap<String, Vec<String>>,
    severity_mapping: HashMap<String, EmergencySeverity>,
}

impl ContextAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = ContextAnalyzer {
            emergency_types: HashMap::new(),
            context_patterns: HashMap::new(),
            severity_mapping: HashMap::new(),
        };
        
        analyzer.initialize_patterns();
        analyzer
    }
    
    fn initialize_patterns(&mut self) {
        // Initialize emergency types
        let emergency_types = vec![
            "drowning", "heart_attack", "stroke", "choking", "bleeding",
            "unconscious", "seizure", "poisoning", "burn", "diabetic",
            "allergic", "trauma", "suicide_prevention", "overdose_reversal", "hypothermia_self_rescue"
        ];
        
        for emergency_type in emergency_types {
            self.emergency_types.insert(emergency_type.to_string(), EmergencyType::new(emergency_type));
        }
        
        // Initialize context patterns for different emergency types
        self.context_patterns.insert("drowning".to_string(), vec![
            "not breathing".to_string(), "underwater".to_string(), "blue lips".to_string(),
            "unconscious".to_string(), "cold".to_string(), "water".to_string()
        ]);
        
        self.context_patterns.insert("heart_attack".to_string(), vec![
            "chest pain".to_string(), "shortness of breath".to_string(), "nausea".to_string(),
            "sweating".to_string(), "arm pain".to_string(), "jaw pain".to_string()
        ]);
        
        self.context_patterns.insert("choking".to_string(), vec![
            "can't breathe".to_string(), "choking".to_string(), "food stuck".to_string(),
            "hands on throat".to_string(), "turning blue".to_string()
        ]);
        
        self.context_patterns.insert("bleeding".to_string(), vec![
            "bleeding".to_string(), "blood".to_string(), "cut".to_string(),
            "wound".to_string(), "severe bleeding".to_string()
        ]);
        
        // Initialize severity mapping
        self.severity_mapping.insert("drowning".to_string(), EmergencySeverity::LifeThreatening);
        self.severity_mapping.insert("heart_attack".to_string(), EmergencySeverity::LifeThreatening);
        self.severity_mapping.insert("stroke".to_string(), EmergencySeverity::LifeThreatening);
        self.severity_mapping.insert("choking".to_string(), EmergencySeverity::LifeThreatening);
        self.severity_mapping.insert("bleeding".to_string(), EmergencySeverity::Critical);
        self.severity_mapping.insert("unconscious".to_string(), EmergencySeverity::Critical);
        self.severity_mapping.insert("seizure".to_string(), EmergencySeverity::High);
        self.severity_mapping.insert("poisoning".to_string(), EmergencySeverity::Critical);
        self.severity_mapping.insert("burn".to_string(), EmergencySeverity::High);
        self.severity_mapping.insert("diabetic".to_string(), EmergencySeverity::High);
        self.severity_mapping.insert("allergic".to_string(), EmergencySeverity::LifeThreatening);
        self.severity_mapping.insert("trauma".to_string(), EmergencySeverity::Critical);
        self.severity_mapping.insert("suicide_prevention".to_string(), EmergencySeverity::LifeThreatening);
        self.severity_mapping.insert("overdose_reversal".to_string(), EmergencySeverity::LifeThreatening);
        self.severity_mapping.insert("hypothermia_self_rescue".to_string(), EmergencySeverity::Critical);
    }
    
    pub fn get_emergency_types(&self) -> Vec<String> {
        self.emergency_types.keys().cloned().collect()
    }
    
    pub fn analyze_emergency(&self, emergency_type: &str, user_input: &str) -> EmergencyContext {
        let severity = self.severity_mapping.get(emergency_type)
            .cloned()
            .unwrap_or(EmergencySeverity::Medium);
        
        let context_flags = self.extract_context_flags(emergency_type, user_input);
        let current_step = self.determine_current_step(emergency_type, &context_flags);
        let time_critical = self.is_time_critical(emergency_type, &context_flags);
        let requires_immediate_action = self.requires_immediate_action(emergency_type, &context_flags);
        
        EmergencyContext {
            emergency_type: emergency_type.to_string(),
            current_step,
            context_flags,
            user_input: user_input.to_string(),
            severity_level: severity,
            time_critical,
            requires_immediate_action,
            location_known: true, // Assume location is available
            help_available: true, // Assume help is available
        }
    }
    
    fn extract_context_flags(&self, emergency_type: &str, user_input: &str) -> Vec<String> {
        let mut flags = Vec::new();
        let input_lower = user_input.to_lowercase();
        
        if let Some(patterns) = self.context_patterns.get(emergency_type) {
            for pattern in patterns {
                if input_lower.contains(pattern) {
                    flags.push(pattern.clone());
                }
            }
        }
        
        // Add general context flags
        if input_lower.contains("not breathing") || input_lower.contains("unconscious") {
            flags.push("not_breathing".to_string());
        }
        
        if input_lower.contains("bleeding") || input_lower.contains("blood") {
            flags.push("bleeding".to_string());
        }
        
        if input_lower.contains("pain") || input_lower.contains("hurt") {
            flags.push("pain".to_string());
        }
        
        if input_lower.contains("cold") || input_lower.contains("freezing") {
            flags.push("cold".to_string());
        }
        
        flags
    }
    
    fn determine_current_step(&self, emergency_type: &str, context_flags: &[String]) -> u32 {
        match emergency_type {
            "drowning" => {
                if context_flags.contains(&"not_breathing".to_string()) {
                    3 // Start CPR
                } else if context_flags.contains(&"underwater".to_string()) {
                    1 // Check breathing
                } else {
                    1
                }
            },
            "heart_attack" => {
                if context_flags.contains(&"chest pain".to_string()) {
                    1 // Call 911 immediately
                } else {
                    1
                }
            },
            "choking" => {
                if context_flags.contains(&"can't breathe".to_string()) {
                    2 // Back blows
                } else {
                    1 // Assess severity
                }
            },
            _ => 1,
        }
    }
    
    fn is_time_critical(&self, emergency_type: &str, context_flags: &[String]) -> bool {
        match emergency_type {
            "drowning" | "heart_attack" | "stroke" | "choking" | "allergic" => true,
            "bleeding" => context_flags.contains(&"severe bleeding".to_string()),
            "unconscious" => context_flags.contains(&"not_breathing".to_string()),
            _ => false,
        }
    }
    
    fn requires_immediate_action(&self, emergency_type: &str, context_flags: &[String]) -> bool {
        self.is_time_critical(emergency_type, context_flags)
    }
    
    pub fn get_next_instruction(&self, context: &EmergencyContext) -> Option<String> {
        match context.emergency_type.as_str() {
            "drowning" => {
                match context.current_step {
                    1 => Some("Check if the person is breathing. Look, listen, and feel for breathing for 5-10 seconds.".to_string()),
                    2 => Some("If not breathing, call 911 immediately and get help.".to_string()),
                    3 => Some("Begin chest compressions at rate of 100-120 per minute, depth 2-2.4 inches for adults.".to_string()),
                    4 => Some("After 30 compressions, give 2 rescue breaths. Allow full chest recoil between compressions.".to_string()),
                    5 => Some("Continue cycles of 30 compressions and 2 breaths until emergency services arrive.".to_string()),
                    _ => Some("Continue following emergency protocol until help arrives.".to_string()),
                }
            },
            "heart_attack" => {
                match context.current_step {
                    1 => Some("Call 911 immediately. Time is critical for heart attacks - every minute counts.".to_string()),
                    2 => Some("Have the person sit down and rest comfortably. Loosen any tight clothing.".to_string()),
                    3 => Some("If available and not allergic, give one adult aspirin (325mg) to chew and swallow.".to_string()),
                    4 => Some("Stay with the person and monitor their condition until help arrives.".to_string()),
                    _ => Some("Continue monitoring until emergency services arrive.".to_string()),
                }
            },
            "choking" => {
                match context.current_step {
                    1 => Some("Ask 'Are you choking?' - if they can speak, encourage coughing.".to_string()),
                    2 => Some("Give 5 back blows between shoulder blades using heel of hand.".to_string()),
                    3 => Some("Give 5 abdominal thrusts (Heimlich maneuver) - place fist above navel, grasp with other hand, thrust inward and upward.".to_string()),
                    4 => Some("Continue alternating 5 back blows and 5 abdominal thrusts until object is expelled or person becomes unconscious.".to_string()),
                    5 => Some("Call 911 if person becomes unconscious and begin CPR.".to_string()),
                    _ => Some("Continue following choking protocol until object is expelled.".to_string()),
                }
            },
            _ => Some("Follow emergency protocol for this situation.".to_string()),
        }
    }
    
    pub fn should_call_911(&self, context: &EmergencyContext) -> bool {
        context.time_critical || context.requires_immediate_action
    }
    
    pub fn get_emergency_summary(&self, context: &EmergencyContext) -> Option<String> {
        Some(format!("{} emergency detected. Severity: {:?}. Time critical: {}.", 
            context.emergency_type, context.severity_level, context.time_critical))
    }
}

#[derive(Debug, Clone)]
struct EmergencyType {
    name: String,
    description: String,
    steps: Vec<String>,
}

impl EmergencyType {
    fn new(name: &str) -> Self {
        EmergencyType {
            name: name.to_string(),
            description: format!("Emergency response for {}", name),
            steps: Vec::new(),
        }
    }
}
