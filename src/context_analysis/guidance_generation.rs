//! Context-Aware Guidance Generation
//! 
//! Generates appropriate emergency guidance based on the current stage
//! and context, skipping irrelevant steps.

use crate::types::{EmergencyType, EmergencyStage, EmergencyGuidance, ContextClues, VictimStatus};
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

/// Guidance generator for context-aware emergency instructions
pub struct GuidanceGenerator {
    stage_guidance: StageGuidance,
    emergency_adapters: Vec<EmergencyAdapter>,
}

impl GuidanceGenerator {
    pub fn new() -> Self {
        Self {
            stage_guidance: StageGuidance::new(),
            emergency_adapters: Self::load_emergency_adapters(),
        }
    }

    /// Generate context-appropriate guidance
    pub async fn generate_guidance(
        &self,
        emergency_type: EmergencyType,
        stage: EmergencyStage,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        info!("Generating guidance for {:?} at stage {:?}", emergency_type, stage);
        
        // Check if user specifically requested direct actions
        let user_phrase_lower = clues.user_phrase.to_lowercase();
        
        // Direct action phrases
        if user_phrase_lower.contains("cpr") || user_phrase_lower.contains("cardiopulmonary") {
            return self.generate_cpr_guidance(clues);
        }
        
        if user_phrase_lower.contains("heimlich") || user_phrase_lower.contains("abdominal thrusts") || user_phrase_lower.contains("choking maneuver") {
            return self.generate_heimlich_guidance(clues);
        }
        
        if user_phrase_lower.contains("aed") || user_phrase_lower.contains("defibrillator") || 
           (user_phrase_lower.contains("shock") && (user_phrase_lower.contains("heart") || user_phrase_lower.contains("chest") || user_phrase_lower.contains("defibrillator"))) {
            return self.generate_aed_guidance(clues);
        }
        
        // Medical shock (not AED shock)
        if (user_phrase_lower.contains("shock") && !user_phrase_lower.contains("defibrillator") && !user_phrase_lower.contains("aed")) ||
           user_phrase_lower.contains("medical shock") || user_phrase_lower.contains("anaphylactic shock") {
            return self.generate_medical_shock_guidance(clues);
        }
        
        // Electric shock/electrocution
        if user_phrase_lower.contains("electric shock") || user_phrase_lower.contains("electrocution") || 
           (user_phrase_lower.contains("shock") && user_phrase_lower.contains("electric")) {
            return self.generate_electric_shock_guidance(clues);
        }
        
        if user_phrase_lower.contains("tourniquet") || 
           (user_phrase_lower.contains("stop bleeding") && user_phrase_lower.contains("tourniquet")) {
            return self.generate_tourniquet_guidance(clues);
        }
        
        // Direct pressure for bleeding
        if user_phrase_lower.contains("direct pressure") || 
           (user_phrase_lower.contains("pressure") && user_phrase_lower.contains("bleeding")) {
            return self.generate_direct_pressure_guidance(clues);
        }
        
        // Blood pressure issues
        if user_phrase_lower.contains("blood pressure") || (user_phrase_lower.contains("pressure") && user_phrase_lower.contains("blood")) {
            return self.generate_blood_pressure_guidance(clues);
        }
        
        if user_phrase_lower.contains("epipen") || user_phrase_lower.contains("epinephrine") || user_phrase_lower.contains("allergic reaction") {
            return self.generate_epipen_guidance(clues);
        }
        
        if user_phrase_lower.contains("rescue breathing") || user_phrase_lower.contains("mouth to mouth") || user_phrase_lower.contains("ventilation") {
            return self.generate_rescue_breathing_guidance(clues);
        }
        
        // Not breathing
        if user_phrase_lower.contains("not breathing") || user_phrase_lower.contains("no breathing") {
            return self.generate_not_breathing_guidance(clues);
        }
        
        // Breathing problems
        if user_phrase_lower.contains("breathing problems") || user_phrase_lower.contains("difficulty breathing") {
            return self.generate_breathing_problems_guidance(clues);
        }
        
        if user_phrase_lower.contains("first aid") || user_phrase_lower.contains("bandage") || user_phrase_lower.contains("wound care") {
            return self.generate_first_aid_guidance(clues);
        }
        
        // Stop bleeding - specific
        if user_phrase_lower.contains("stop bleeding") && !user_phrase_lower.contains("tourniquet") {
            return self.generate_stop_bleeding_guidance(clues);
        }
        
        // FAST test for stroke
        if user_phrase_lower.contains("fast test") || user_phrase_lower.contains("stroke test") || 
           (user_phrase_lower.contains("face") && user_phrase_lower.contains("arm") && user_phrase_lower.contains("speech")) {
            return self.generate_fast_test_guidance(clues);
        }
        
        // Poison control call
        if user_phrase_lower.contains("poison control") || user_phrase_lower.contains("call poison") {
            return self.generate_poison_control_guidance(clues);
        }
        
        // Cool burn
        if user_phrase_lower.contains("cool burn") || user_phrase_lower.contains("burn cooling") {
            return self.generate_cool_burn_guidance(clues);
        }
        
        // Medical alert check
        if user_phrase_lower.contains("medical alert") || user_phrase_lower.contains("check bracelet") || 
           user_phrase_lower.contains("medical id") {
            return self.generate_medical_alert_guidance(clues);
        }
        
        let guidance = match (emergency_type.clone(), stage.clone()) {
            (EmergencyType::Drowning, EmergencyStage::VictimExtracted) => {
                self.generate_drowning_post_extraction_guidance(clues)?
            }
            (EmergencyType::Drowning, EmergencyStage::Unconscious) => {
                self.generate_drowning_unconscious_guidance(clues)?
            }
            (EmergencyType::Choking, EmergencyStage::VictimExtracted) => {
                self.generate_choking_post_extraction_guidance(clues)?
            }
            (EmergencyType::HeartAttack, EmergencyStage::InitialDetection) => {
                self.generate_heart_attack_initial_guidance(clues)?
            }
            (EmergencyType::Stroke, EmergencyStage::InitialDetection) => {
                self.generate_stroke_guidance(clues)?
            }
            (EmergencyType::Seizure, EmergencyStage::InitialDetection) => {
                self.generate_seizure_guidance(clues)?
            }
            (EmergencyType::Poisoning, EmergencyStage::InitialDetection) => {
                self.generate_poisoning_guidance(clues)?
            }
            (EmergencyType::SevereBurns, EmergencyStage::InitialDetection) => {
                self.generate_severe_burns_guidance(clues)?
            }
            (EmergencyType::DiabeticEmergency, EmergencyStage::InitialDetection) => {
                self.generate_diabetic_emergency_guidance(clues)?
            }
            (EmergencyType::Bleeding, EmergencyStage::ConsciousButInjured) => {
                self.generate_bleeding_guidance(clues)?
            }
            _ => {
                self.generate_generic_stage_guidance(emergency_type, stage, clues)?
            }
        };

        Ok(guidance)
    }

    /// Generate generic CPR guidance for any emergency type
    fn generate_cpr_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Begin CPR immediately - 30 compressions, 2 breaths".to_string(),
            "Place hands on center of chest, between nipples".to_string(),
            "Compress at rate of 100-120 per minute".to_string(),
            "Allow chest to fully recoil between compressions".to_string(),
            "Continue until victim responds or help arrives".to_string(),
        ];

        let priority_actions = vec![
            "Start chest compressions now".to_string(),
            "Call 911 if not done".to_string(),
            "Monitor for breathing and pulse".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::Unconscious,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate Heimlich maneuver guidance
    fn generate_heimlich_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Stand behind the person".to_string(),
            "Wrap your arms around their waist".to_string(),
            "Make a fist with one hand".to_string(),
            "Place it above the navel, below the ribcage".to_string(),
            "Grasp your fist with your other hand".to_string(),
            "Give 5 quick, upward abdominal thrusts".to_string(),
            "Continue until object is dislodged or person becomes unconscious".to_string(),
        ];

        let priority_actions = vec![
            "Position yourself behind victim".to_string(),
            "Perform abdominal thrusts immediately".to_string(),
            "Monitor for object dislodgement".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::VictimExtracted,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate AED guidance
    fn generate_aed_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Turn on the AED and follow voice prompts".to_string(),
            "Expose the victim's chest completely".to_string(),
            "Apply electrode pads as shown in diagram".to_string(),
            "Ensure no one is touching the victim".to_string(),
            "Press the shock button when advised".to_string(),
            "Continue CPR between shocks if needed".to_string(),
        ];

        let priority_actions = vec![
            "Activate AED immediately".to_string(),
            "Apply electrode pads correctly".to_string(),
            "Follow AED voice prompts".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::Unconscious,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate tourniquet guidance
    fn generate_tourniquet_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Apply tourniquet 2-3 inches above the wound".to_string(),
            "Never apply directly over a joint".to_string(),
            "Tighten until bleeding stops".to_string(),
            "Secure the tourniquet in place".to_string(),
            "Note the time of application".to_string(),
            "Call 911 immediately".to_string(),
        ];

        let priority_actions = vec![
            "Apply tourniquet above wound".to_string(),
            "Tighten until bleeding stops".to_string(),
            "Note application time".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::ConsciousButInjured,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate EpiPen guidance
    fn generate_epipen_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Remove the EpiPen from its case".to_string(),
            "Hold the EpiPen in your fist with the orange tip pointing down".to_string(),
            "Remove the blue safety cap".to_string(),
            "Place the orange tip against the outer thigh".to_string(),
            "Press firmly and hold for 3 seconds".to_string(),
            "Call 911 immediately after use".to_string(),
            "Monitor the person for improvement".to_string(),
        ];

        let priority_actions = vec![
            "Administer EpiPen to outer thigh".to_string(),
            "Hold for 3 seconds".to_string(),
            "Call 911 immediately".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::ConsciousButInjured,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate rescue breathing guidance
    fn generate_rescue_breathing_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Tilt the head back and lift the chin".to_string(),
            "Pinch the nose closed".to_string(),
            "Give 2 rescue breaths".to_string(),
            "Watch for chest rise with each breath".to_string(),
            "Continue with 30 compressions, 2 breaths".to_string(),
            "Continue until victim responds or help arrives".to_string(),
        ];

        let priority_actions = vec![
            "Open airway and give rescue breaths".to_string(),
            "Continue CPR cycle".to_string(),
            "Monitor for breathing".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::BreathingButUnresponsive,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate first aid guidance
    fn generate_first_aid_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Clean the wound with soap and water".to_string(),
            "Apply direct pressure to stop bleeding".to_string(),
            "Cover with sterile bandage".to_string(),
            "Elevate injured area if possible".to_string(),
            "Monitor for signs of infection".to_string(),
            "Seek medical attention if needed".to_string(),
        ];

        let priority_actions = vec![
            "Clean and cover the wound".to_string(),
            "Apply direct pressure".to_string(),
            "Monitor for complications".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::ConsciousButInjured,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate medical shock guidance
    fn generate_medical_shock_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Lay the person flat on their back".to_string(),
            "Elevate their legs 12 inches".to_string(),
            "Keep them warm with blankets".to_string(),
            "Loosen tight clothing".to_string(),
            "Do not give them anything to eat or drink".to_string(),
            "Call 911 immediately".to_string(),
        ];

        let priority_actions = vec![
            "Lay person flat and elevate legs".to_string(),
            "Keep them warm".to_string(),
            "Call 911 immediately".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::ConsciousButInjured,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate electric shock guidance
    fn generate_electric_shock_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Do not touch the person if they are still in contact with electricity".to_string(),
            "Turn off the power source if possible".to_string(),
            "Use a non-conductive object to move them away from the source".to_string(),
            "Check for breathing and pulse".to_string(),
            "Begin CPR if they are not breathing".to_string(),
            "Call 911 immediately".to_string(),
        ];

        let priority_actions = vec![
            "Turn off power source safely".to_string(),
            "Check breathing and pulse".to_string(),
            "Begin CPR if needed".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::Unconscious,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate direct pressure guidance
    fn generate_direct_pressure_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Apply direct pressure to the wound with a clean cloth".to_string(),
            "Use your hand if no cloth is available".to_string(),
            "Maintain pressure for at least 10 minutes".to_string(),
            "Do not remove the cloth to check bleeding".to_string(),
            "Add more cloth on top if blood soaks through".to_string(),
            "Call 911 if bleeding is severe".to_string(),
        ];

        let priority_actions = vec![
            "Apply direct pressure with clean cloth".to_string(),
            "Maintain pressure for 10 minutes".to_string(),
            "Call 911 if severe bleeding".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::ConsciousButInjured,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate blood pressure guidance
    fn generate_blood_pressure_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Have the person sit or lie down".to_string(),
            "Loosen tight clothing".to_string(),
            "Keep them calm and comfortable".to_string(),
            "Monitor their symptoms".to_string(),
            "Call 911 if they have chest pain or severe symptoms".to_string(),
            "Seek medical attention for persistent issues".to_string(),
        ];

        let priority_actions = vec![
            "Have person sit or lie down".to_string(),
            "Keep them calm".to_string(),
            "Call 911 for severe symptoms".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::ConsciousButInjured,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate not breathing guidance
    fn generate_not_breathing_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Check for responsiveness".to_string(),
            "Call 911 immediately".to_string(),
            "Begin CPR if not breathing".to_string(),
            "30 chest compressions, 2 rescue breaths".to_string(),
            "Continue until help arrives".to_string(),
            "Use AED if available".to_string(),
        ];

        let priority_actions = vec![
            "Call 911 immediately".to_string(),
            "Begin CPR now".to_string(),
            "Use AED if available".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::Unconscious,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate breathing problems guidance
    fn generate_breathing_problems_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Help them sit in a comfortable position".to_string(),
            "Loosen tight clothing around neck and waist".to_string(),
            "Encourage slow, deep breathing".to_string(),
            "Keep them calm and reassured".to_string(),
            "Call 911 if breathing is severely difficult".to_string(),
            "Monitor for worsening symptoms".to_string(),
        ];

        let priority_actions = vec![
            "Help them sit comfortably".to_string(),
            "Loosen tight clothing".to_string(),
            "Call 911 for severe difficulty".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::ConsciousButInjured,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate stop bleeding guidance
    fn generate_stop_bleeding_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Apply direct pressure with a clean cloth".to_string(),
            "Elevate the injured area if possible".to_string(),
            "Maintain pressure for at least 10 minutes".to_string(),
            "Do not remove cloth to check bleeding".to_string(),
            "Add more cloth if blood soaks through".to_string(),
            "Call 911 if bleeding is severe or doesn't stop".to_string(),
        ];

        let priority_actions = vec![
            "Apply direct pressure".to_string(),
            "Elevate injured area".to_string(),
            "Call 911 for severe bleeding".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::ConsciousButInjured,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate drowning guidance for post-extraction
    fn generate_drowning_post_extraction_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Check if victim is breathing and has a pulse".to_string(),
            "If not breathing, begin rescue breathing immediately".to_string(),
            "If no pulse, start chest compressions".to_string(),
            "Keep victim warm and dry".to_string(),
            "Monitor for secondary drowning symptoms".to_string(),
        ];

        let priority_actions = vec![
            "Assess breathing and pulse".to_string(),
            "Begin rescue breathing if needed".to_string(),
            "Start CPR if no pulse".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::VictimExtracted,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate drowning guidance for unconscious victim
    fn generate_drowning_unconscious_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Begin CPR immediately - 30 compressions, 2 breaths".to_string(),
            "Continue until victim responds or help arrives".to_string(),
            "Monitor for breathing and pulse".to_string(),
            "Keep victim warm".to_string(),
            "Prepare for emergency services arrival".to_string(),
        ];

        let priority_actions = vec![
            "Start CPR now".to_string(),
            "Call 911 if not done".to_string(),
            "Monitor vital signs".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::Unconscious,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate choking guidance for post-extraction
    fn generate_choking_post_extraction_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Check if object is still lodged".to_string(),
            "If conscious, encourage coughing".to_string(),
            "If unconscious, begin rescue breathing".to_string(),
            "Monitor breathing and consciousness".to_string(),
            "Prepare for emergency services".to_string(),
        ];

        let priority_actions = vec![
            "Assess airway obstruction".to_string(),
            "Begin rescue breathing if needed".to_string(),
            "Monitor breathing".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::VictimExtracted,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate heart attack initial guidance
    fn generate_heart_attack_initial_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Call 911 immediately".to_string(),
            "Have victim sit or lie down".to_string(),
            "Loosen tight clothing".to_string(),
            "Monitor breathing and consciousness".to_string(),
            "Prepare for emergency services".to_string(),
        ];

        let priority_actions = vec![
            "Call 911 now".to_string(),
            "Keep victim calm".to_string(),
            "Monitor vital signs".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions,
            priority_actions,
            skip_basic_steps: false,
            focus_on_medical_care: true,
        })
    }

    /// Generate stroke guidance
    fn generate_stroke_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Call 911 immediately - time is critical".to_string(),
            "Note the time symptoms started".to_string(),
            "Use FAST test: Face drooping, Arm weakness, Speech difficulty, Time to call 911".to_string(),
            "Keep person calm and comfortable".to_string(),
            "Do not give food or drink".to_string(),
            "Monitor breathing and consciousness".to_string(),
        ];

        let priority_actions = vec![
            "Call 911 now".to_string(),
            "Note time of onset".to_string(),
            "Use FAST test".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions,
            priority_actions,
            skip_basic_steps: false,
            focus_on_medical_care: true,
        })
    }

    /// Generate poisoning guidance
    fn generate_poisoning_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Call 911 immediately".to_string(),
            "Call Poison Control: 1-800-222-1222".to_string(),
            "Do NOT induce vomiting".to_string(),
            "Identify the substance if possible".to_string(),
            "Keep the container or packaging".to_string(),
            "Monitor breathing and consciousness".to_string(),
        ];

        let priority_actions = vec![
            "Call 911 now".to_string(),
            "Call Poison Control".to_string(),
            "Do not induce vomiting".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions,
            priority_actions,
            skip_basic_steps: false,
            focus_on_medical_care: true,
        })
    }

    /// Generate severe burns guidance
    fn generate_severe_burns_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Call 911 immediately".to_string(),
            "Cool the burn with cool (not cold) water for 10-20 minutes".to_string(),
            "Do NOT use ice or very cold water".to_string(),
            "Do NOT pop blisters".to_string(),
            "Cover with clean, dry cloth".to_string(),
            "Do NOT apply ointments or creams".to_string(),
        ];

        let priority_actions = vec![
            "Call 911 now".to_string(),
            "Cool with water".to_string(),
            "Do not pop blisters".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions,
            priority_actions,
            skip_basic_steps: false,
            focus_on_medical_care: true,
        })
    }

    /// Generate diabetic emergency guidance
    fn generate_diabetic_emergency_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Call 911 immediately".to_string(),
            "Look for medical alert bracelet or necklace".to_string(),
            "Check if person is conscious and responsive".to_string(),
            "Do NOT give insulin unless you are trained".to_string(),
            "Do NOT give food or drink if unconscious".to_string(),
            "Monitor breathing and consciousness".to_string(),
        ];

        let priority_actions = vec![
            "Call 911 now".to_string(),
            "Look for medical alert".to_string(),
            "Do not give insulin".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions,
            priority_actions,
            skip_basic_steps: false,
            focus_on_medical_care: true,
        })
    }

    /// Generate seizure guidance
    fn generate_seizure_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Call 911 immediately".to_string(),
            "Clear the area of dangerous objects".to_string(),
            "Do not restrain the person".to_string(),
            "Do not put anything in their mouth".to_string(),
            "Time the seizure duration".to_string(),
            "Turn them on their side if possible".to_string(),
        ];

        let priority_actions = vec![
            "Call 911 now".to_string(),
            "Clear the area".to_string(),
            "Time the seizure".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions,
            priority_actions,
            skip_basic_steps: false,
            focus_on_medical_care: true,
        })
    }

    /// Generate bleeding guidance
    fn generate_bleeding_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Apply direct pressure to wound".to_string(),
            "Elevate injured area if possible".to_string(),
            "Use clean cloth or bandage".to_string(),
            "Monitor for signs of shock".to_string(),
            "Call 911 for severe bleeding".to_string(),
        ];

        let priority_actions = vec![
            "Stop bleeding with pressure".to_string(),
            "Call 911 if severe".to_string(),
            "Monitor victim condition".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::ConsciousButInjured,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate generic guidance for any stage
    fn generate_generic_stage_guidance(
        &self,
        emergency_type: EmergencyType,
        stage: EmergencyStage,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Assess the situation".to_string(),
            "Call 911 if not already done".to_string(),
            "Provide basic first aid".to_string(),
            "Monitor victim condition".to_string(),
            "Prepare for emergency services".to_string(),
        ];

        let priority_actions = vec![
            "Call 911".to_string(),
            "Assess victim".to_string(),
            "Provide care".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: stage.clone(),
            instructions,
            priority_actions,
            skip_basic_steps: stage != EmergencyStage::InitialDetection,
            focus_on_medical_care: true,
        })
    }

    /// Generate medical alert check guidance
    fn generate_medical_alert_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Look for medical alert bracelet or necklace".to_string(),
            "Check for medical ID card in wallet".to_string(),
            "Look for medical alert tattoo".to_string(),
            "Check phone for medical ID app".to_string(),
            "Ask bystanders if they know the person".to_string(),
            "Call 911 immediately if found".to_string(),
        ];

        let priority_actions = vec![
            "Check for medical alert jewelry".to_string(),
            "Look for medical ID".to_string(),
            "Call 911 immediately".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate FAST test guidance
    fn generate_fast_test_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "F - Face: Ask person to smile. Does one side droop?".to_string(),
            "A - Arms: Ask person to raise both arms. Does one drift down?".to_string(),
            "S - Speech: Ask person to repeat a simple phrase. Is speech slurred?".to_string(),
            "T - Time: If any of these signs are present, call 911 immediately".to_string(),
            "Note the time symptoms started".to_string(),
            "Keep person calm and comfortable".to_string(),
        ];

        let priority_actions = vec![
            "Perform FAST test now".to_string(),
            "Call 911 immediately if any sign present".to_string(),
            "Note time of onset".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate poison control guidance
    fn generate_poison_control_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Call Poison Control immediately: 1-800-222-1222".to_string(),
            "Have the container or packaging ready".to_string(),
            "Do NOT induce vomiting unless directed".to_string(),
            "Identify the substance if possible".to_string(),
            "Note the time of exposure".to_string(),
            "Call 911 if person is unconscious or having trouble breathing".to_string(),
        ];

        let priority_actions = vec![
            "Call Poison Control: 1-800-222-1222".to_string(),
            "Have container ready".to_string(),
            "Do not induce vomiting".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Generate cool burn guidance
    fn generate_cool_burn_guidance(
        &self,
        clues: &ContextClues,
    ) -> Result<EmergencyGuidance, AppError> {
        let instructions = vec![
            "Cool the burn with cool (not cold) water for 10-20 minutes".to_string(),
            "Do NOT use ice or very cold water".to_string(),
            "Do NOT pop any blisters".to_string(),
            "Remove jewelry or tight items from burned area".to_string(),
            "Cover with clean, dry cloth".to_string(),
            "Do NOT apply ointments or creams".to_string(),
            "Call 911 for severe burns".to_string(),
        ];

        let priority_actions = vec![
            "Cool with cool water for 10-20 minutes".to_string(),
            "Do not use ice".to_string(),
            "Do not pop blisters".to_string(),
        ];

        Ok(EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions,
            priority_actions,
            skip_basic_steps: true,
            focus_on_medical_care: true,
        })
    }

    /// Load emergency adapters
    fn load_emergency_adapters() -> Vec<EmergencyAdapter> {
        vec![
            EmergencyAdapter::new("drowning_adapter"),
            EmergencyAdapter::new("choking_adapter"),
            EmergencyAdapter::new("heart_attack_adapter"),
            EmergencyAdapter::new("bleeding_adapter"),
        ]
    }
}

/// Stage guidance system
pub struct StageGuidance {
    stage_templates: Vec<StageTemplate>,
}

impl StageGuidance {
    pub fn new() -> Self {
        Self {
            stage_templates: Self::load_stage_templates(),
        }
    }

    /// Load stage templates
    fn load_stage_templates() -> Vec<StageTemplate> {
        vec![
            StageTemplate {
                stage: EmergencyStage::VictimExtracted,
                template: "Post-Extraction Care",
                skip_basic: true,
            },
            StageTemplate {
                stage: EmergencyStage::Unconscious,
                template: "Unconscious Care",
                skip_basic: true,
            },
            StageTemplate {
                stage: EmergencyStage::InitialDetection,
                template: "Initial Response",
                skip_basic: false,
            },
        ]
    }
}

/// Emergency adapter for specific emergency types
pub struct EmergencyAdapter {
    name: String,
}

impl EmergencyAdapter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

/// Stage template for guidance generation
#[derive(Debug, Clone)]
pub struct StageTemplate {
    pub stage: EmergencyStage,
    pub template: &'static str,
    pub skip_basic: bool,
}

/// Example usage scenarios
pub struct GuidanceExamples;

impl GuidanceExamples {
    /// Example: "Drowning help" when person is already out of water
    pub fn drowning_post_extraction_example() -> EmergencyGuidance {
        EmergencyGuidance {
            stage: EmergencyStage::VictimExtracted,
            instructions: vec![
                "Check if victim is breathing and has a pulse".to_string(),
                "If not breathing, begin rescue breathing immediately".to_string(), 
                "If no pulse, start chest compressions".to_string(),
                "Keep victim warm and dry".to_string(),
                "Monitor for secondary drowning symptoms".to_string(),
            ],
            priority_actions: vec![
                "Assess breathing and pulse".to_string(),
                "Begin rescue breathing if needed".to_string(),
                "Start CPR if no pulse".to_string(),
            ],
            skip_basic_steps: true,
            focus_on_medical_care: true,
        }
    }

    /// Example: "Choking help" when object is dislodged
    pub fn choking_post_extraction_example() -> EmergencyGuidance {
        EmergencyGuidance {
            stage: EmergencyStage::VictimExtracted,
            instructions: vec![
                "Check if object is still lodged".to_string(),
                "If conscious, encourage coughing".to_string(),
                "If unconscious, begin rescue breathing".to_string(),
                "Monitor breathing and consciousness".to_string(),
            ],
            priority_actions: vec![
                "Assess airway obstruction".to_string(),
                "Begin rescue breathing if needed".to_string(),
                "Monitor breathing".to_string(),
            ],
            skip_basic_steps: true,
            focus_on_medical_care: true,
        }
    }

    /// Example: "Heart attack" initial response
    pub fn heart_attack_initial_example() -> EmergencyGuidance {
        EmergencyGuidance {
            stage: EmergencyStage::InitialDetection,
            instructions: vec![
                "Call 911 immediately".to_string(),
                "Have victim sit or lie down".to_string(),
                "Loosen tight clothing".to_string(),
                "Monitor breathing and consciousness".to_string(),
            ],
            priority_actions: vec![
                "Call 911 now".to_string(),
                "Keep victim calm".to_string(),
                "Monitor vital signs".to_string(),
            ],
            skip_basic_steps: false,
            focus_on_medical_care: true,
        }
    }
} 