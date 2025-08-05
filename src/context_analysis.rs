use crate::emergency_database::{EmergencyDatabase, EmergencyProtocol, EmergencyStep};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct EmergencyContext {
    pub emergency_type: String,
    pub current_step: u32,
    pub context_flags: Vec<String>,
    pub user_responses: HashMap<String, String>,
    pub time_elapsed: u32, // seconds
    pub location: Option<String>,
    pub weather_conditions: Option<String>,
    pub number_of_victims: u32,
    pub bystanders_available: bool,
}

impl EmergencyContext {
    pub fn new(emergency_type: String) -> Self {
        EmergencyContext {
            emergency_type,
            current_step: 1,
            context_flags: Vec::new(),
            user_responses: HashMap::new(),
            time_elapsed: 0,
            location: None,
            weather_conditions: None,
            number_of_victims: 1,
            bystanders_available: false,
        }
    }
    
    pub fn add_context_flag(&mut self, flag: String) {
        if !self.context_flags.contains(&flag) {
            self.context_flags.push(flag);
        }
    }
    
    pub fn add_user_response(&mut self, question: String, response: String) {
        self.user_responses.insert(question, response);
    }
    
    pub fn update_time_elapsed(&mut self, seconds: u32) {
        self.time_elapsed += seconds;
    }
    
    pub fn next_step(&mut self) {
        self.current_step += 1;
    }
}

pub struct ContextAnalyzer {
    database: EmergencyDatabase,
}

impl ContextAnalyzer {
    pub fn new() -> Self {
        ContextAnalyzer {
            database: EmergencyDatabase::new(),
        }
    }
    
    pub fn analyze_emergency(&self, emergency_type: &str, user_input: &str) -> EmergencyContext {
        let mut context = EmergencyContext::new(emergency_type.to_string());
        
        // Analyze user input for context clues
        self.analyze_user_input(&mut context, user_input);
        
        // Add location-based context
        self.add_location_context(&mut context);
        
        // Add weather context if relevant
        if self.is_weather_relevant(emergency_type) {
            self.add_weather_context(&mut context);
        }
        
        context
    }
    
    fn analyze_user_input(&self, context: &mut EmergencyContext, input: &str) {
        let input_lower = input.to_lowercase();
        
        // Analyze for emergency stage indicators
        match context.emergency_type.as_str() {
            "drowning" => self.analyze_drowning_context(context, &input_lower),
            "heart_attack" => self.analyze_heart_attack_context(context, &input_lower),
            "choking" => self.analyze_choking_context(context, &input_lower),
            "bleeding" => self.analyze_bleeding_context(context, &input_lower),
            "unconscious" => self.analyze_unconscious_context(context, &input_lower),
            "stroke" => self.analyze_stroke_context(context, &input_lower),
            "seizure" => self.analyze_seizure_context(context, &input_lower),
            "poisoning" => self.analyze_poisoning_context(context, &input_lower),
            "burn" => self.analyze_burn_context(context, &input_lower),
            "diabetic" => self.analyze_diabetic_context(context, &input_lower),
            "allergic" => self.analyze_allergic_context(context, &input_lower),
            "trauma" => self.analyze_trauma_context(context, &input_lower),
            _ => {}
        }
    }
    
    fn analyze_drowning_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("out of water") || input.contains("pulled out") {
            context.add_context_flag("out_of_water".to_string());
            context.current_step = 2; // Skip rescue, go to assessment
        }
        
        if input.contains("not breathing") || input.contains("no breath") {
            context.add_context_flag("not_breathing".to_string());
            context.current_step = 3; // Go to CPR
        }
        
        if input.contains("breathing") && input.contains("unconscious") {
            context.add_context_flag("breathing".to_string());
            context.add_context_flag("unconscious".to_string());
            context.current_step = 4; // Go to recovery position
        }
        
        if input.contains("conscious") || input.contains("awake") {
            context.add_context_flag("conscious".to_string());
            context.current_step = 5; // Go to monitoring
        }
    }
    
    fn analyze_heart_attack_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("chest pain") || input.contains("pressure") {
            context.add_context_flag("chest_pain".to_string());
        }
        
        if input.contains("nitro") || input.contains("nitroglycerin") {
            context.add_context_flag("has_nitroglycerin".to_string());
        }
        
        if input.contains("unconscious") || input.contains("passed out") {
            context.add_context_flag("unconscious".to_string());
        }
    }
    
    fn analyze_choking_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("can't speak") || input.contains("cannot talk") {
            context.add_context_flag("cannot_speak".to_string());
            context.current_step = 2; // Go to back blows
        }
        
        if input.contains("back blows") || input.contains("didn't work") {
            context.add_context_flag("back_blows_failed".to_string());
            context.current_step = 3; // Go to Heimlich
        }
        
        if input.contains("still choking") || input.contains("continuing") {
            context.add_context_flag("continuing_choking".to_string());
            context.current_step = 4; // Go to alternating
        }
        
        if input.contains("unconscious") || input.contains("passed out") {
            context.add_context_flag("unconscious".to_string());
            context.current_step = 5; // Go to CPR
        }
    }
    
    fn analyze_bleeding_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("still bleeding") || input.contains("continuing") {
            context.add_context_flag("bleeding_continues".to_string());
            context.current_step = 4; // Go to additional bandages
        }
        
        if input.contains("severe") || input.contains("heavy") || input.contains("lots of blood") {
            context.add_context_flag("severe_bleeding".to_string());
            context.current_step = 5; // Go to call 911
        }
    }
    
    fn analyze_unconscious_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("not breathing") || input.contains("no breath") {
            context.add_context_flag("not_breathing".to_string());
            context.current_step = 3; // Go to CPR
        }
        
        if input.contains("breathing") {
            context.add_context_flag("breathing".to_string());
            context.current_step = 4; // Go to recovery position
        }
    }
    
    fn analyze_stroke_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("face") && input.contains("droop") {
            context.add_context_flag("face_drooping".to_string());
        }
        
        if input.contains("arm") && (input.contains("weak") || input.contains("numb")) {
            context.add_context_flag("arm_weakness".to_string());
        }
        
        if input.contains("speech") && (input.contains("difficult") || input.contains("slurred")) {
            context.add_context_flag("speech_difficulty".to_string());
        }
    }
    
    fn analyze_seizure_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("over 5 minutes") || input.contains("more than 5") {
            context.add_context_flag("seizure_over_5_minutes".to_string());
            context.current_step = 5; // Go to call 911
        }
        
        if input.contains("injured") || input.contains("hurt") {
            context.add_context_flag("injury".to_string());
            context.current_step = 5; // Go to call 911
        }
    }
    
    fn analyze_poisoning_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("unconscious") || input.contains("passed out") {
            context.add_context_flag("unconscious".to_string());
            context.current_step = 4; // Go to call 911
        }
        
        if input.contains("difficulty breathing") || input.contains("can't breathe") {
            context.add_context_flag("difficulty_breathing".to_string());
            context.current_step = 4; // Go to call 911
        }
    }
    
    fn analyze_burn_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("severe") || input.contains("large") || input.contains("face") || 
           input.contains("hands") || input.contains("feet") || input.contains("genitals") {
            context.add_context_flag("severe_burn".to_string());
            context.current_step = 4; // Go to call 911
        }
    }
    
    fn analyze_diabetic_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("unconscious") || input.contains("passed out") {
            context.add_context_flag("unconscious".to_string());
            context.current_step = 3; // Go to call 911
        }
        
        if input.contains("no improvement") || input.contains("not better") {
            context.add_context_flag("no_improvement".to_string());
            context.current_step = 5; // Go to call 911
        }
    }
    
    fn analyze_allergic_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("epi") || input.contains("epinephrine") {
            context.add_context_flag("has_epinephrine".to_string());
        }
        
        if input.contains("unconscious") || input.contains("passed out") {
            context.add_context_flag("unconscious".to_string());
            context.current_step = 5; // Go to CPR preparation
        }
    }
    
    fn analyze_trauma_context(&self, context: &mut EmergencyContext, input: &str) {
        if input.contains("bleeding") {
            context.add_context_flag("bleeding".to_string());
        }
    }
    
    fn add_location_context(&self, context: &mut EmergencyContext) {
        // This would integrate with GPS/location services
        // For now, we'll add some common location-based flags
        if let Some(location) = &context.location {
            match location.as_str() {
                "beach" | "pool" | "lake" => {
                    if context.emergency_type == "drowning" {
                        context.add_context_flag("water_location".to_string());
                    }
                },
                "kitchen" => {
                    if context.emergency_type == "burn" {
                        context.add_context_flag("kitchen_burn".to_string());
                    }
                },
                "road" | "highway" => {
                    if context.emergency_type == "trauma" {
                        context.add_context_flag("vehicle_accident".to_string());
                    }
                },
                _ => {}
            }
        }
    }
    
    fn add_weather_context(&self, context: &mut EmergencyContext) {
        if let Some(weather) = &context.weather_conditions {
            match weather.as_str() {
                "storm" | "lightning" => {
                    context.add_context_flag("storm_conditions".to_string());
                },
                "cold" | "freezing" => {
                    context.add_context_flag("cold_weather".to_string());
                },
                "hot" | "heat" => {
                    context.add_context_flag("hot_weather".to_string());
                },
                _ => {}
            }
        }
    }
    
    fn is_weather_relevant(&self, emergency_type: &str) -> bool {
        matches!(emergency_type, "drowning" | "heart_attack" | "heat_stroke" | "hypothermia")
    }
    
    pub fn get_next_instruction(&self, context: &EmergencyContext) -> Option<String> {
        if let Some(protocol) = self.database.get_protocol(&context.emergency_type) {
            // Find the most appropriate step based on context
            if let Some(step) = self.database.get_context_appropriate_step(
                &context.emergency_type, 
                &context.context_flags
            ) {
                return Some(step.instruction.clone());
            }
            
            // Fall back to current step
            if let Some(step) = self.database.get_step(&context.emergency_type, context.current_step) {
                return Some(step.instruction.clone());
            }
        }
        None
    }
    
    pub fn get_emergency_summary(&self, context: &EmergencyContext) -> Option<String> {
        if let Some(protocol) = self.database.get_protocol(&context.emergency_type) {
            let mut summary = format!("🚨 {} Emergency\n\n", context.emergency_type.replace("_", " "));
            summary.push_str(&format!("Immediate Action: {}\n\n", protocol.immediate_action));
            summary.push_str(&format!("Warning: {}\n\n", protocol.warning));
            
            if protocol.call_911_immediately {
                summary.push_str("⚠️ CALL 911 IMMEDIATELY\n\n");
            }
            
            summary.push_str(&format!("Estimated EMS arrival: {} minutes\n", protocol.estimated_ems_time));
            
            Some(summary)
        } else {
            None
        }
    }
    
    pub fn should_call_911(&self, context: &EmergencyContext) -> bool {
        if let Some(protocol) = self.database.get_protocol(&context.emergency_type) {
            protocol.call_911_immediately
        } else {
            false
        }
    }
    
    pub fn get_emergency_types(&self) -> Vec<String> {
        self.database.list_emergency_types().into_iter().map(|s| s.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_context_analyzer_creation() {
        let analyzer = ContextAnalyzer::new();
        assert_eq!(analyzer.get_emergency_types().len(), 12);
    }
    
    #[test]
    fn test_drowning_context_analysis() {
        let analyzer = ContextAnalyzer::new();
        let context = analyzer.analyze_emergency("drowning", "they are out of water but not breathing");
        
        assert!(context.context_flags.contains(&"out_of_water".to_string()));
        assert!(context.context_flags.contains(&"not_breathing".to_string()));
        assert_eq!(context.current_step, 3); // Should go to CPR step
    }
    
    #[test]
    fn test_heart_attack_context_analysis() {
        let analyzer = ContextAnalyzer::new();
        let context = analyzer.analyze_emergency("heart_attack", "chest pain and they have nitroglycerin");
        
        assert!(context.context_flags.contains(&"chest_pain".to_string()));
        assert!(context.context_flags.contains(&"has_nitroglycerin".to_string()));
    }
} 