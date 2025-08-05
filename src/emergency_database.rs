use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyStep {
    pub step_number: u32,
    pub instruction: String,
    pub critical: bool,
    pub time_estimate: u32, // seconds
    pub context_dependent: bool,
    pub context_conditions: Vec<String>,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Check if victim is breathing and conscious".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "If not breathing, begin CPR immediately".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["not_breathing".to_string()],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "If breathing but unconscious, place in recovery position".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["breathing".to_string(), "unconscious".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Keep victim warm and monitor breathing".to_string(),
                    critical: false,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
            ],
            warning: "Drowning can cause delayed complications. Always seek medical attention even if victim appears fine.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 7,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Have victim sit down and rest".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Loosen any tight clothing around neck and chest".to_string(),
                    critical: false,
                    time_estimate: 15,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "If prescribed, help victim take nitroglycerin".to_string(),
                    critical: false,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["has_nitroglycerin".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Monitor breathing and consciousness".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
            ],
            warning: "Heart attack symptoms can vary. When in doubt, call 911 immediately.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 8,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "If unable to speak, perform 5 back blows between shoulder blades".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["cannot_speak".to_string()],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "If back blows fail, perform 5 abdominal thrusts (Heimlich)".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["back_blows_failed".to_string()],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Alternate 5 back blows and 5 abdominal thrusts".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["continuing_choking".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "If victim becomes unconscious, begin CPR".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["unconscious".to_string()],
                },
            ],
            warning: "Call 911 if choking persists or victim becomes unconscious.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 6,
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
                    instruction: "Apply direct pressure to wound with clean cloth or bandage".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Elevate injured area above heart if possible".to_string(),
                    critical: false,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Apply pressure for at least 10-15 minutes".to_string(),
                    critical: true,
                    time_estimate: 900,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "If bleeding continues, apply additional bandages on top".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["bleeding_continues".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Call 911 if bleeding is severe or doesn't stop".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["severe_bleeding".to_string()],
                },
            ],
            warning: "Severe bleeding can be life-threatening. Call 911 immediately if bleeding is heavy or doesn't stop.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 7,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Check breathing - look, listen, feel for 10 seconds".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "If not breathing, begin CPR immediately".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["not_breathing".to_string()],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "If breathing, place in recovery position".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["breathing".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Call 911 and monitor breathing until help arrives".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
            ],
            warning: "Unconsciousness can indicate serious medical emergency. Call 911 immediately.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 8,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Use FAST test: Face drooping, Arm weakness, Speech difficulty, Time to call 911".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Have victim sit down and rest".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Note time symptoms started".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Monitor consciousness and breathing".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
            ],
            warning: "Stroke is a medical emergency. Call 911 immediately - do not wait for symptoms to improve.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 7,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Place something soft under head".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Do not restrain or put anything in mouth".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Time the seizure duration".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Call 911 if seizure lasts more than 5 minutes or person is injured".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["seizure_over_5_minutes".to_string(), "injury".to_string()],
                },
            ],
            warning: "Call 911 if seizure lasts more than 5 minutes, person is injured, or this is their first seizure.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 8,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Do not induce vomiting unless directed by Poison Control".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Save container or substance for identification".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Call 911 if person is unconscious or having difficulty breathing".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["unconscious".to_string(), "difficulty_breathing".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Monitor for symptoms and follow Poison Control instructions".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
            ],
            warning: "Do not induce vomiting unless specifically directed by Poison Control.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 8,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Remove jewelry and tight items from burned area".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Cover with clean, dry cloth".to_string(),
                    critical: false,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Call 911 for severe burns (large area, face, hands, feet, genitals)".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["severe_burn".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Do not break blisters or apply creams".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
            ],
            warning: "Call 911 for burns larger than 3 inches, on face/hands/feet/genitals, or if person has difficulty breathing.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 8,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "If conscious, give sugar (glucose tablets, juice, candy)".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["conscious".to_string()],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "If unconscious, call 911 immediately".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["unconscious".to_string()],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Wait 15 minutes, then recheck blood sugar if possible".to_string(),
                    critical: true,
                    time_estimate: 900,
                    context_dependent: true,
                    context_conditions: vec!["conscious".to_string()],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Call 911 if condition doesn't improve or person becomes unconscious".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["no_improvement".to_string()],
                },
            ],
            warning: "Call 911 if person is unconscious or condition doesn't improve after giving sugar.".to_string(),
            call_911_immediately: false,
            estimated_ems_time: 7,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Use epinephrine auto-injector if available".to_string(),
                    critical: true,
                    time_estimate: 30,
                    context_dependent: true,
                    context_conditions: vec!["has_epinephrine".to_string()],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Help person sit up and lean forward".to_string(),
                    critical: true,
                    time_estimate: 10,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Monitor breathing and consciousness".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Be prepared to perform CPR if person becomes unconscious".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["unconscious".to_string()],
                },
            ],
            warning: "Anaphylaxis is life-threatening. Call 911 immediately and use epinephrine if available.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 6,
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
                },
                EmergencyStep {
                    step_number: 2,
                    instruction: "Do not move person unless in immediate danger".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 3,
                    instruction: "Control bleeding with direct pressure if present".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: true,
                    context_conditions: vec!["bleeding".to_string()],
                },
                EmergencyStep {
                    step_number: 4,
                    instruction: "Keep person warm and comfortable".to_string(),
                    critical: false,
                    time_estimate: 30,
                    context_dependent: false,
                    context_conditions: vec![],
                },
                EmergencyStep {
                    step_number: 5,
                    instruction: "Monitor breathing and consciousness".to_string(),
                    critical: true,
                    time_estimate: 0,
                    context_dependent: false,
                    context_conditions: vec![],
                },
            ],
            warning: "Serious trauma requires immediate medical attention. Do not move person unless absolutely necessary.".to_string(),
            call_911_immediately: true,
            estimated_ems_time: 8,
        };
        
        self.protocols.insert("trauma".to_string(), protocol);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_database_initialization() {
        let db = EmergencyDatabase::new();
        assert_eq!(db.protocols.len(), 12);
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