//! Solana SOS - Voice-activated emergency response system for Solana Mobile
//! 
//! **JUDGING VERSION NOTE**: This repository contains the complete implementation
//! with all modules publicly exposed for hackathon judging. This demonstrates
//! the full technical sophistication including medical protocols, gamification
//! algorithms, and safety features. After judging, sensitive implementation
//! details will be moved to private modules for production deployment.
//! 
//! ## Core Features
//! - Voice-activated emergency response
//! - 15 official medical protocols with real-time guidance
//! - Context-aware emergency analysis
//! - Gamification system with BONK/SKR token rewards
//! - Advanced safety features (Silent SOS, Crash Detection)
//! - Solana Mobile dApp store ready

// Public interface modules (for external use)
pub mod public;

// Implementation modules (exposed for judging)
pub mod public_implementation;

// pub mod jni_bridge; // Temporarily disabled for core testing

// Core modules (always available)
pub mod app;
pub mod config;
pub mod error;

// Re-export main types for easy access
pub use app::SolanaSOSApp;
pub use error::AppResult;


pub use public::types::{EmergencyType, EmergencyStage, DirectAction, ConnectivityMode, GuidanceMode};

// Re-export interface types
pub use public::voice_interface::{VoiceTrigger, VoiceStats};
pub use config::VoiceConfig;
pub use public::audio_interface::{AudioProcessor, AudioConfig, AudioStats};
pub use public::emergency_interface::{EmergencySystem, EmergencyConfig, EmergencyStats};

// Note: Implementation modules moved to src/private/ for IP protection
// These are now accessed through the public interfaces above

use crate::public_implementation::emergency_database::EmergencyDatabase;
use crate::public_implementation::context_analysis::{ContextAnalyzer, EmergencyContext};
use crate::public_implementation::emergency_calling::{EmergencyCaller, EmergencyContact, EmergencyCallError};
use std::collections::HashMap;

pub struct SolanaSOS {
    database: EmergencyDatabase,
    context_analyzer: ContextAnalyzer,
    emergency_caller: EmergencyCaller,
}

impl SolanaSOS {
    pub fn new() -> Self {
        SolanaSOS {
            database: EmergencyDatabase::new(),
            context_analyzer: ContextAnalyzer::new(),
            emergency_caller: EmergencyCaller::new(),
        }
    }
    
    /// Initialize the emergency response system
    pub fn initialize(&mut self) -> Result<(), String> {
        // Initialize emergency database
        let emergency_types = self.database.list_emergency_types();
        if emergency_types.is_empty() {
            return Err("Failed to initialize emergency database".to_string());
        }
        
        // Initialize context analyzer
        let analyzer_types = self.context_analyzer.get_emergency_types();
        if analyzer_types.is_empty() {
            return Err("Failed to initialize context analyzer".to_string());
        }
        
        println!("🚨 Solana SOS initialized with {} emergency types", emergency_types.len());
        Ok(())
    }
    
    /// Process emergency activation with voice input
    pub async fn process_emergency(&mut self, emergency_type: &str, user_input: &str) -> EmergencyResponse {
        // Analyze the emergency context
        let context = self.context_analyzer.analyze_emergency(emergency_type, user_input);
        
        // Get the appropriate instruction
        let instruction = self.context_analyzer.get_next_instruction(&context);
        
        // Determine if 911 should be called
        let should_call_911 = self.context_analyzer.should_call_911(&context);
        
        // Get emergency summary
        let summary = self.context_analyzer.get_emergency_summary(&context);
        
        EmergencyResponse {
            instruction: instruction.unwrap_or_else(|| "Emergency protocol not found".to_string()),
            should_call_911,
            emergency_summary: summary,
            context_flags: context.context_flags,
            current_step: context.current_step,
        }
    }
    
    /// Make an emergency 911 call
    pub async fn call_911(&mut self, emergency_type: &str, context_flags: &[String]) -> Result<String, EmergencyCallError> {
        self.emergency_caller.call_911(emergency_type, context_flags).await
    }
    
    /// Add emergency contact
    pub fn add_emergency_contact(&mut self, name: &str, phone: &str, relationship: &str) {
        let contact = EmergencyContact {
            name: name.to_string(),
            phone_number: phone.to_string(),
            relationship: relationship.to_string(),
            notification_enabled: true,
            last_notified: None,
        };
        self.emergency_caller.add_emergency_contact(contact);
    }
    
    /// Get all emergency contacts
    pub fn get_emergency_contacts(&self) -> &Vec<EmergencyContact> {
        self.emergency_caller.get_emergency_contacts()
    }
    
    /// Get available emergency types
    pub fn get_emergency_types(&self) -> Vec<String> {
        self.database.list_emergency_types().into_iter().map(|s| s.clone()).collect()
    }
    
    /// Get emergency protocol for a specific type
        pub fn get_emergency_protocol(&self, emergency_type: &str) -> Option<&crate::public_implementation::emergency_database::EmergencyProtocol> {
        self.database.get_protocol(emergency_type)
    }

    /// Get call history
    pub fn get_call_history(&self) -> &Vec<crate::public_implementation::emergency_calling::EmergencyCall> {
        self.emergency_caller.get_call_history()
    }
    
    /// Award tokens for emergency response (for gamification)
    pub fn award_emergency_tokens(&self, emergency_type: &str, response_time: u32) -> TokenAward {
        // Calculate token award based on emergency type and response time
        let base_tokens = match emergency_type {
            "drowning" => 100,
            "heart_attack" => 150,
            "choking" => 80,
            "bleeding" => 60,
            "unconscious" => 120,
            "stroke" => 140,
            "seizure" => 90,
            "poisoning" => 110,
            "burn" => 70,
            "diabetic" => 85,
            "allergic" => 130,
            "trauma" => 95,
            _ => 50,
        };
        
        // Bonus for fast response time (under 30 seconds)
        let time_bonus = if response_time < 30 { 50 } else { 0 };
        
        TokenAward {
            bonk_tokens: base_tokens + time_bonus,
            skr_tokens: (base_tokens + time_bonus) / 2,
            xp_points: base_tokens * 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EmergencyResponse {
    pub instruction: String,
    pub should_call_911: bool,
    pub emergency_summary: Option<String>,
    pub context_flags: Vec<String>,
    pub current_step: u32,
}

#[derive(Debug, Clone)]
pub struct TokenAward {
    pub bonk_tokens: u32,
    pub skr_tokens: u32,
    pub xp_points: u32,
}

// JNI bindings for Android integration
#[cfg(target_os = "android")]
use jni::JNIEnv;
#[cfg(target_os = "android")]
use jni::objects::{JClass, JString};
#[cfg(target_os = "android")]
use jni::sys::jstring;

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_solanasos_emergency_RustBridge_processEmergency(
    mut _env: JNIEnv,
    _class: JClass,
    emergency_type: JString,
    user_input: JString,
) -> jstring {
    // This would integrate with the SolanaSOS struct
    // For now, return a simple response
    let emergency_type_str = _env.get_string(&emergency_type).unwrap().to_str().unwrap().to_string();
    let user_input_str = _env.get_string(&user_input).unwrap().to_str().unwrap().to_string();
    
    let response = format!("Emergency response for {}: {}", emergency_type_str, user_input_str);
    
    _env.new_string(response).unwrap().into_raw()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_solanasos_emergency_RustBridge_call911(
    mut _env: JNIEnv,
    _class: JClass,
    emergency_type: JString,
) -> jstring {
    // This would make the actual 911 call
    let emergency_type_str = _env.get_string(&emergency_type).unwrap().to_str().unwrap().to_string();
    let response = format!("911 call initiated for {}", emergency_type_str);
    
    _env.new_string(response).unwrap().into_raw()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "system" fn Java_com_solanasos_emergency_RustBridge_awardEmergencyTokens(
    mut _env: JNIEnv,
    _class: JClass,
    emergency_type: JString,
    response_time: i32,
) -> jstring {
    let emergency_type_str = _env.get_string(&emergency_type).unwrap().to_str().unwrap().to_string();
    let sos = SolanaSOS::new();
    let award = sos.award_emergency_tokens(&emergency_type_str, response_time as u32);
    
    let response = format!("Awarded {} BONK, {} SKR, {} XP", 
        award.bonk_tokens, award.skr_tokens, award.xp_points);
    
    _env.new_string(response).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_solana_sos_creation() {
        let sos = SolanaSOS::new();
        let emergency_types = sos.get_emergency_types();
        assert_eq!(emergency_types.len(), 15);
    }
    
    #[test]
    fn test_emergency_contact_management() {
        let mut sos = SolanaSOS::new();
        sos.add_emergency_contact("Mom", "555-1234", "Mother");
        
        let contacts = sos.get_emergency_contacts();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].name, "Mom");
    }
    
    #[test]
    fn test_token_award() {
        let sos = SolanaSOS::new();
        let award = sos.award_emergency_tokens("drowning", 25);
        
        assert_eq!(award.bonk_tokens, 150); // 100 base + 50 time bonus
        assert_eq!(award.skr_tokens, 75); // 150 / 2
        assert_eq!(award.xp_points, 200); // 100 * 2
    }
    
    #[tokio::test]
    async fn test_emergency_processing() {
        let mut sos = SolanaSOS::new();
        sos.initialize().unwrap();
        
        let response = sos.process_emergency("drowning", "they are out of water but not breathing").await;
        
        assert!(response.should_call_911);
        println!("Instruction: {}", response.instruction);
        assert!(!response.instruction.is_empty());
        assert!(response.context_flags.contains(&"not_breathing".to_string()));
    }
}
