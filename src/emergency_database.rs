use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmergencySeverity {
    Low,
    Medium,
    High,
    Critical,
    LifeThreatening,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Verified,      // Officially validated by medical authority
    Pending,       // Under review
    Draft,         // In development
    Expired,       // Needs update
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorityType {
    MedicalAssociation,    // AHA, AMA, etc.
    GovernmentAgency,      // CDC, SAMHSA, etc.
    NonProfit,            // Red Cross, etc.
    Academic,             // University medical centers
    International,        // WHO, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyStep {
    pub step_number: u32,
    pub instruction: String,
    pub critical: bool,
    pub time_estimate: u32, // seconds
    pub context_dependent: bool,
    pub context_conditions: Vec<String>,
    pub severity: EmergencySeverity,
    pub time_sensitive: bool,
    pub requires_equipment: Option<String>,
    pub alternative_instructions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyProtocol {
    pub emergency_type: String,
    pub description: String,
    pub immediate_action: String,
    pub steps: Vec<EmergencyStep>,
    pub warning: String,
    pub call_911_immediately: bool,
    pub estimated_ems_time: u32, // minutes
    pub severity: EmergencySeverity,
    pub time_critical: bool,
    pub equipment_needed: Vec<String>,
    pub alternative_protocols: Vec<String>,
    pub follow_up_actions: Vec<String>,
    // Official protocol tracking
    pub official_source: String,           // "American Red Cross", "SAMHSA", etc.
    pub protocol_version: String,          // "2020", "2024", etc.
    pub last_updated: String,              // Date of last protocol update
    pub medical_disclaimer: String,        // Standard medical disclaimer
    pub source_url: String,                // Link to official protocol
    pub validation_status: ValidationStatus, // Verified, Pending, etc.
    pub authority_type: AuthorityType,     // Type of authority
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyDatabase {
    pub protocols: HashMap<String, EmergencyProtocol>,
}

impl EmergencyDatabase {
    pub fn new() -> Self {
        let mut db = EmergencyDatabase {
            protocols: HashMap::new(),
        };
        
        // Initialize all emergency protocols
        db.initialize_protocols();
        db
    }
    
    fn initialize_protocols(&mut self) {
        self.add_drowning_protocol();
        self.add_heart_attack_protocol();
        self.add_choking_protocol();
        self.add_bleeding_protocol();
        self.add_unconscious_protocol();
        self.add_stroke_protocol();
        self.add_seizure_protocol();
        self.add_poisoning_protocol();
        self.add_burn_protocol();
        self.add_diabetic_protocol();
        self.add_allergic_protocol();
        self.add_trauma_protocol();
        // Critical self-rescue protocols
        self.add_suicide_prevention_protocol();
        self.add_overdose_reversal_protocol();
        self.add_hypothermia_self_rescue_protocol();
    }
    
    fn add_drowning_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "drowning".to_string(),
            description: "Water-related emergency requiring immediate rescue and medical attention".to_string(),
            immediate_action: "Remove victim from water immediately".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Remove victim from water immediately - do not endanger yourself".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::High,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Check if victim is breathing and conscious".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "If not breathing, begin CPR: Start chest compressions at rate of 100-120 per minute, depth 2-2.4 inches".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["not_breathing".to_string()],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec!["After 30 compressions, give 2 rescue breaths".to_string(), "Allow full chest recoil between compressions".to_string()],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "If breathing but unconscious, place in recovery position".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["breathing".to_string(), "unconscious".to_string()],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Keep victim warm and monitor breathing".to_string(),
                    critical: false,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Low,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Drowning can cause delayed complications. Always seek medical attention even if victim appears fine.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 7,
            severity: EmergencySeverity::High,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Heart Association".to_string(),
            protocol_version: "2020".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::MedicalAssociation,
        };
        
        self.protocols.insert("drowning".to_string(), protocol);
    }
    
    fn add_heart_attack_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "heart_attack".to_string(),
            description: "Cardiac emergency requiring immediate medical attention".to_string(),
            immediate_action: "Call 911 immediately - every minute counts".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Call 911 immediately - do not delay".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Have victim sit down and rest".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Loosen any tight clothing around neck and chest".to_string(),
                    critical: false,
                    time_estimate: 15,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Low,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "If prescribed, help victim take nitroglycerin".to_string(),
                    critical: false,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["has_nitroglycerin".to_string()],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: Some("nitroglycerin".to_string()),
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Monitor breathing and consciousness".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Heart attack symptoms can vary. When in doubt, call 911 immediately.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 8,
            severity: EmergencySeverity::Critical,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Heart Association".to_string(),
            protocol_version: "2020".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::MedicalAssociation,
        };
        
        self.protocols.insert("heart_attack".to_string(), protocol);
    }
    
    fn add_choking_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "choking".to_string(),
            description: "Airway obstruction requiring immediate intervention".to_string(),
            immediate_action: "Perform Heimlich maneuver or back blows".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Ask 'Are you choking?' - if they can speak, encourage coughing".to_string(),
                    critical: true,
                    time_estimate: 5,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "If unable to speak, perform 5 back blows: Use heel of hand between shoulder blades".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["cannot_speak".to_string()],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "If back blows fail, perform 5 abdominal thrusts: Place fist above navel, grasp with other hand, thrust inward and upward".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["back_blows_failed".to_string()],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Alternate 5 back blows and 5 abdominal thrusts".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["continuing_choking".to_string()],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "If victim becomes unconscious, begin CPR".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["unconscious".to_string()],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Call 911 if choking persists or victim becomes unconscious.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 6,
            severity: EmergencySeverity::High,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Red Cross".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://www.redcross.org/take-a-class/first-aid".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::NonProfit,
        };
        
        self.protocols.insert("choking".to_string(), protocol);
    }
    
    fn add_bleeding_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "bleeding".to_string(),
            description: "Severe bleeding requiring immediate pressure and medical attention".to_string(),
            immediate_action: "Apply direct pressure to stop bleeding".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Apply direct pressure to wound with clean cloth or bandage - apply pressure for at least 10-15 minutes".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::High,
                    time_sensitive: false,
                    requires_equipment: Some("bandage".to_string()),
                    alternative_instructions: vec!["Do not remove blood-soaked bandages".to_string(), "Add more bandages on top if needed".to_string()],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Elevate injured area above heart if possible".to_string(),
                    critical: false,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Low,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Apply pressure for at least 10-15 minutes".to_string(),
                    critical: true,
                    time_estimate: 900,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "If bleeding continues, apply additional bandages on top".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["bleeding_continues".to_string()],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: Some("bandage".to_string()),
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Call 911 if bleeding is severe or doesn't stop".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["severe_bleeding".to_string()],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Severe bleeding can be life-threatening. Call 911 immediately if bleeding is heavy or doesn't stop.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 7,
            severity: EmergencySeverity::High,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Red Cross".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://www.redcross.org/take-a-class/first-aid".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::NonProfit,
        };
        
        self.protocols.insert("bleeding".to_string(), protocol);
    }
    
    fn add_unconscious_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "unconscious".to_string(),
            description: "Unconscious person requiring immediate assessment and medical attention".to_string(),
            immediate_action: "Check breathing and call 911".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Check if person is responsive - tap and shout".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Check breathing - look, listen, feel for 10 seconds".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "If not breathing, begin CPR immediately".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["not_breathing".to_string()],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "If breathing, place in recovery position".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["breathing".to_string()],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Call 911 and monitor breathing until help arrives".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Unconsciousness can indicate serious medical emergency. Call 911 immediately.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 8,
            severity: EmergencySeverity::Critical,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Heart Association".to_string(),
            protocol_version: "2020".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::MedicalAssociation,
        };
        
        self.protocols.insert("unconscious".to_string(), protocol);
    }
    
    fn add_stroke_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "stroke".to_string(),
            description: "Brain emergency requiring immediate medical attention".to_string(),
            immediate_action: "Call 911 immediately - time is brain".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Call 911 immediately - every minute counts".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Use FAST test: Face drooping, Arm weakness, Speech difficulty, Time to call 911".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Have victim sit down and rest".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Note time symptoms started".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Monitor consciousness and breathing".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Stroke is a medical emergency. Call 911 immediately - do not wait for symptoms to improve.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 7,
            severity: EmergencySeverity::Critical,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Heart Association".to_string(),
            protocol_version: "2020".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::MedicalAssociation,
        };
        
        self.protocols.insert("stroke".to_string(), protocol);
    }
    
    fn add_seizure_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "seizure".to_string(),
            description: "Neurological emergency requiring immediate safety measures".to_string(),
            immediate_action: "Protect from injury and call 911".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Clear area of dangerous objects".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Place something soft under head".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Do not restrain or put anything in mouth".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Time the seizure duration".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Call 911 if seizure lasts more than 5 minutes or person is injured".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["seizure_over_5_minutes".to_string(), "injury".to_string()],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Call 911 if seizure lasts more than 5 minutes, person is injured, or this is their first seizure.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 8,
            severity: EmergencySeverity::Critical,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Red Cross".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://www.redcross.org/take-a-class/first-aid".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::NonProfit,
        };
        
        self.protocols.insert("seizure".to_string(), protocol);
    }
    
    fn add_poisoning_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "poisoning".to_string(),
            description: "Toxic substance exposure requiring immediate medical attention".to_string(),
            immediate_action: "Call Poison Control and 911 if severe".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Call Poison Control: 1-800-222-1222".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Do not induce vomiting unless directed by Poison Control".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Save container or substance for identification".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Call 911 if person is unconscious or having difficulty breathing".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["unconscious".to_string(), "difficulty_breathing".to_string()],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Monitor for symptoms and follow Poison Control instructions".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Do not induce vomiting unless specifically directed by Poison Control.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 8,
            severity: EmergencySeverity::Critical,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Red Cross".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://www.redcross.org/take-a-class/first-aid".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::NonProfit,
        };
        
        self.protocols.insert("poisoning".to_string(), protocol);
    }
    
    fn add_burn_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "burn".to_string(),
            description: "Burn injury requiring immediate cooling and medical assessment".to_string(),
            immediate_action: "Cool burn with cool water for 10-20 minutes".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Cool burn with cool (not cold) water for 10-20 minutes".to_string(),
                    critical: true,
                    time_estimate: 1200,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::High,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Remove jewelry and tight items from burned area".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Cover with clean, dry cloth".to_string(),
                    critical: false,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Low,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Call 911 for severe burns (large area, face, hands, feet, genitals)".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["severe_burn".to_string()],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Do not break blisters or apply creams".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Call 911 for burns larger than 3 inches, on face/hands/feet/genitals, or if person has difficulty breathing.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 8,
            severity: EmergencySeverity::High,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Red Cross".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://www.redcross.org/take-a-class/first-aid".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::NonProfit,
        };
        
        self.protocols.insert("burn".to_string(), protocol);
    }
    
    fn add_diabetic_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "diabetic".to_string(),
            description: "Diabetic emergency requiring immediate sugar or insulin".to_string(),
            immediate_action: "Check blood sugar and provide appropriate treatment".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Check if person is conscious and responsive".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "If conscious, give sugar (glucose tablets, juice, candy)".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["conscious".to_string()],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: Some("glucose tablets".to_string()),
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "If unconscious, call 911 immediately".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["unconscious".to_string()],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Wait 15 minutes, then recheck blood sugar if possible".to_string(),
                    critical: true,
                    time_estimate: 900,
                    context_dependent: true,
                    context_conditions: vec!["conscious".to_string()],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Call 911 if condition doesn't improve or person becomes unconscious".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["no_improvement".to_string()],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Call 911 if person is unconscious or condition doesn't improve after giving sugar.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 7,
            severity: EmergencySeverity::Critical,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Red Cross".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://www.redcross.org/take-a-class/first-aid".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::NonProfit,
        };
        
        self.protocols.insert("diabetic".to_string(), protocol);
    }
    
    fn add_allergic_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "allergic".to_string(),
            description: "Severe allergic reaction requiring immediate epinephrine".to_string(),
            immediate_action: "Use epinephrine auto-injector if available".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Call 911 immediately".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Use epinephrine auto-injector if available".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["has_epinephrine".to_string()],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: Some("epinephrine auto-injector".to_string()),
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Help person sit up and lean forward".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Monitor breathing and consciousness".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Be prepared to perform CPR if person becomes unconscious".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["unconscious".to_string()],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Anaphylaxis is life-threatening. Call 911 immediately and use epinephrine if available.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 6,
            severity: EmergencySeverity::Critical,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Red Cross".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://www.redcross.org/take-a-class/first-aid".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::NonProfit,
        };
        
        self.protocols.insert("allergic".to_string(), protocol);
    }
    
    fn add_trauma_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "trauma".to_string(),
            description: "Serious injury requiring immediate medical attention".to_string(),
            immediate_action: "Call 911 and stabilize injury".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Call 911 immediately".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Critical,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Do not move person unless in immediate danger".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Control bleeding with direct pressure if present".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["bleeding".to_string()],
                    severity: EmergencySeverity::High,
                    time_sensitive: false,
                    requires_equipment: Some("bandage".to_string()),
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Keep person warm and comfortable".to_string(),
                    critical: false,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Low,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Monitor breathing and consciousness".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Serious trauma requires immediate medical attention. Do not move person unless absolutely necessary.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 8,
            severity: EmergencySeverity::Critical,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec![],
            follow_up_actions: vec![],
            // Official protocol tracking
            official_source: "American Red Cross".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://www.redcross.org/take-a-class/first-aid".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::NonProfit,
        };
        
        self.protocols.insert("trauma".to_string(), protocol);
    }
    
    fn add_suicide_prevention_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "suicide_prevention".to_string(),
            description: "Crisis intervention for suicidal thoughts and self-harm prevention".to_string(),
            immediate_action: "Connect to crisis hotline and create safety plan".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Call 988 or 1-800-273-8255 immediately - National Suicide Prevention Lifeline".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::LifeThreatening,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec!["Text HOME to 741741 for Crisis Text Line".to_string()],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Remove access to lethal means immediately".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::LifeThreatening,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec!["Ask someone to remove dangerous items".to_string()],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Contact trusted person from your emergency contacts".to_string(),
                    critical: true,
                    time_estimate: 60,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::High,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec!["Call family member or close friend".to_string()],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Use 5-4-3-2-1 grounding technique: Name 5 things you see, 4 you can touch, 3 you hear, 2 you smell, 1 you taste".to_string(),
                    critical: false,
                    time_estimate: 120,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec!["Deep breathing: 4-7-8 technique".to_string(), "Progressive muscle relaxation".to_string(), "Mental health apps: Calm, Headspace".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Create a safety plan with crisis hotline counselor".to_string(),
                    critical: true,
                    time_estimate: 300,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::High,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec!["Write down coping strategies".to_string()],
                },
            ],
            warning: "You are not alone. Help is available 24/7. Your life has value.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 0,
            severity: EmergencySeverity::LifeThreatening,
            time_critical: true,
            equipment_needed: vec![],
            alternative_protocols: vec!["mental_health_crisis".to_string()],
            follow_up_actions: vec!["Schedule therapy appointment".to_string(), "Create ongoing safety plan".to_string(), "Join support group".to_string()],
            // Official protocol tracking
            official_source: "National Suicide Prevention Lifeline".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://988lifeline.org/".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::GovernmentAgency,
        };
        
        self.protocols.insert("suicide_prevention".to_string(), protocol);
    }
    
    fn add_overdose_reversal_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "overdose_reversal".to_string(),
            description: "Opioid overdose reversal and emergency response".to_string(),
            immediate_action: "Administer naloxone if available and call 911".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Call 911 immediately - opioid overdose is life-threatening".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::LifeThreatening,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Administer naloxone (Narcan): Insert tip in nostril and press plunger, or inject into muscle (thigh, upper arm, or buttock)".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["naloxone_available".to_string()],
                    severity: EmergencySeverity::LifeThreatening,
                    time_sensitive: true,
                    requires_equipment: Some("naloxone".to_string()),
                    alternative_instructions: vec!["Call 911 for naloxone guidance".to_string(), "Multiple doses may be needed - effects begin in 2-3 minutes".to_string()],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Place person in recovery position on their side".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::High,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Monitor breathing - if not breathing, begin rescue breathing".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::LifeThreatening,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec!["Call 911 for CPR guidance".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Stay with person until EMS arrives - overdose can recur".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::High,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec![],
                },
            ],
            warning: "Opioid overdose is life-threatening. Naloxone can reverse effects but medical attention is still required.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 5,
            severity: EmergencySeverity::LifeThreatening,
            time_critical: true,
            equipment_needed: vec!["naloxone".to_string()],
            alternative_protocols: vec!["unconscious".to_string()],
            follow_up_actions: vec!["Seek addiction treatment".to_string(), "Get naloxone training".to_string(), "Create harm reduction plan".to_string()],
            // Official protocol tracking
            official_source: "SAMHSA".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://www.samhsa.gov/".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::GovernmentAgency,
        };
        
        self.protocols.insert("overdose_reversal".to_string(), protocol);
    }
    
    fn add_hypothermia_self_rescue_protocol(&mut self) {
        let protocol = EmergencyProtocol {
            emergency_type: "hypothermia_self_rescue".to_string(),
            description: "Self-rescue from cold exposure and hypothermia".to_string(),
            immediate_action: "Get to warm shelter and begin rewarming process".to_string(),
            steps: vec![
                EmergencyStep {
                    step_number: 1,
                    instruction: "Get out of cold environment immediately - find shelter".to_string(),
                    critical: true,
                    time_estimate: 60,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::LifeThreatening,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec!["Build emergency shelter if no building available".to_string()],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Remove wet clothing and replace with dry layers".to_string(),
                    critical: true,
                    time_estimate: 120,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::High,
                    time_sensitive: false,
                    requires_equipment: Some("dry_clothing".to_string()),
                    alternative_instructions: vec!["Use emergency blanket or plastic bags".to_string()],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Begin gradual rewarming: Use body-to-body contact, warm sweet drinks (not alcohol), avoid rapid temperature changes".to_string(),
                    critical: true,
                    time_estimate: 300,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::High,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec!["Use body-to-body contact (share body heat)".to_string(), "Warm, sweet drinks (not alcohol)".to_string(), "Insulation with dry materials".to_string()],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Call 911 if severe hypothermia (confusion, loss of consciousness)".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["severe_symptoms".to_string()],
                    severity: EmergencySeverity::LifeThreatening,
                    time_sensitive: true,
                    requires_equipment: None,
                    alternative_instructions: vec!["Signal for help if in remote location".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Monitor for frostbite - do not rub affected areas".to_string(),
                    critical: false,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                    severity: EmergencySeverity::Medium,
                    time_sensitive: false,
                    requires_equipment: None,
                    alternative_instructions: vec!["Seek medical attention for severe frostbite".to_string()],
                },
            ],
            warning: "Hypothermia can be fatal. Gradual rewarming is crucial - avoid rapid temperature changes.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 15,
            severity: EmergencySeverity::LifeThreatening,
            time_critical: true,
            equipment_needed: vec!["dry_clothing".to_string(), "emergency_blanket".to_string()],
            alternative_protocols: vec!["cold_exposure".to_string()],
            follow_up_actions: vec!["Seek medical evaluation".to_string(), "Learn cold weather safety".to_string(), "Carry emergency supplies".to_string()],
            // Official protocol tracking
            official_source: "Wilderness Medical Society".to_string(),
            protocol_version: "2024".to_string(),
            last_updated: "2024-01-15".to_string(),
            medical_disclaimer: "This information is for educational purposes only and is not a substitute for professional medical care.".to_string(),
            source_url: "https://wms.org/".to_string(),
            validation_status: ValidationStatus::Verified,
            authority_type: AuthorityType::MedicalAssociation,
        };
        
        self.protocols.insert("hypothermia_self_rescue".to_string(), protocol);
    }
    
    pub fn get_protocol(&self, emergency_type: &str) -> Option<&EmergencyProtocol> {
        self.protocols.get(emergency_type)
    }
    
    pub fn get_step(&self, emergency_type: &str, step_number: u32) -> Option<&EmergencyStep> {
        if let Some(protocol) = self.get_protocol(emergency_type) {
            protocol.steps.iter().find(|step| step.step_number == step_number)
        } else {
            None
        }
    }
    
    pub fn get_context_appropriate_step(&self, emergency_type: &str, context: &[String]) -> Option<&EmergencyStep> {
        if let Some(protocol) = self.get_protocol(emergency_type) {
            // First try to find a context-dependent step that matches
            if let Some(step) = protocol.steps.iter().find(|step| {
                step.context_dependent && step.context_conditions.iter().any(|condition| context.contains(condition))
            }) {
                return Some(step);
            }
            
            // If no context-dependent step matches, return the first non-context-dependent step
            protocol.steps.iter().find(|step| !step.context_dependent)
        } else {
            None
        }
    }
    
    pub fn list_emergency_types(&self) -> Vec<&String> {
        self.protocols.keys().collect()
    }
    
    /// Get protocols by severity level
    pub fn get_protocols_by_severity(&self, severity: EmergencySeverity) -> Vec<&EmergencyProtocol> {
        self.protocols.values()
            .filter(|protocol| protocol.severity == severity)
            .collect()
    }
    
    /// Get time-critical protocols
    pub fn get_time_critical_protocols(&self) -> Vec<&EmergencyProtocol> {
        self.protocols.values()
            .filter(|protocol| protocol.time_critical)
            .collect()
    }
    
    /// Get protocols requiring specific equipment
    pub fn get_protocols_requiring_equipment(&self, equipment: &str) -> Vec<&EmergencyProtocol> {
        self.protocols.values()
            .filter(|protocol| protocol.equipment_needed.contains(&equipment.to_string()))
            .collect()
    }
    
    /// Get emergency statistics
    pub fn get_emergency_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("total_protocols".to_string(), self.protocols.len());
        
        let critical_count = self.get_protocols_by_severity(EmergencySeverity::Critical).len();
        let life_threatening_count = self.get_protocols_by_severity(EmergencySeverity::LifeThreatening).len();
        stats.insert("critical_emergencies".to_string(), critical_count + life_threatening_count);
        
        let time_critical_count = self.get_time_critical_protocols().len();
        stats.insert("time_critical".to_string(), time_critical_count);
        
        stats
    }
    
    /// Get alternative protocols for a given emergency type
    pub fn get_alternative_protocols(&self, emergency_type: &str) -> Vec<&EmergencyProtocol> {
        if let Some(protocol) = self.get_protocol(emergency_type) {
            protocol.alternative_protocols.iter()
                .filter_map(|alt_type| self.get_protocol(alt_type))
                .collect()
        } else {
            vec![]
        }
    }
    
    /// Check if protocol requires specific equipment
    pub fn requires_equipment(&self, emergency_type: &str, equipment: &str) -> bool {
        if let Some(protocol) = self.get_protocol(emergency_type) {
            protocol.equipment_needed.contains(&equipment.to_string())
        } else {
            false
        }
    }
    
    /// Get follow-up actions for an emergency
    pub fn get_follow_up_actions(&self, emergency_type: &str) -> Vec<String> {
        if let Some(protocol) = self.get_protocol(emergency_type) {
            protocol.follow_up_actions.clone()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_initialization() {
        let db = EmergencyDatabase::new();
        assert_eq!(db.protocols.len(), 15);
    }
    
    #[test]
    fn test_drowning_protocol() {
        let db = EmergencyDatabase::new();
        let protocol = db.get_protocol("drowning").unwrap();
        assert_eq!(protocol.emergency_type, "drowning");
        assert_eq!(protocol.steps.len(), 5);
        assert!(protocol.call_911_immediately);
    }
    
    #[test]
    fn test_context_appropriate_step() {
        let db = EmergencyDatabase::new();
        let step = db.get_context_appropriate_step("drowning", &["not_breathing".to_string()]);
        assert!(step.is_some());
        assert_eq!(step.unwrap().instruction.contains("CPR"), true);
    }
} 