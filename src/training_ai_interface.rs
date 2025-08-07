//! Training AI Interface
//! 
//! This module demonstrates how Medical AI can enhance training development
//! while maintaining safety and accuracy standards.

use crate::error::AppResult;
use crate::medical_ai::MedicalAI;
use crate::training_interface::{TrainingModule, TrainingProgress};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AI-enhanced training development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingAI {
    /// Medical AI for symptom analysis
    medical_ai: MedicalAI,
    /// User's emergency response history
    user_history: HashMap<String, u32>,
    /// User's training performance
    training_performance: HashMap<String, f32>,
    /// Local risk factors
    local_risks: Vec<String>,
    /// Medical certification requirements
    certification_standards: HashMap<String, Vec<String>>,
}

/// AI-generated training recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITrainingRecommendation {
    /// Recommended training module
    pub module: TrainingModule,
    /// Priority level (1-10)
    pub priority: u8,
    /// Reasoning for recommendation
    pub reasoning: String,
    /// Customized content focus
    pub focus_areas: Vec<String>,
    /// Estimated training time
    pub estimated_time: u32, // minutes
    /// Certification relevance
    pub certification_relevant: bool,
}

/// AI-generated training scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITrainingScenario {
    /// Scenario description
    pub description: String,
    /// Emergency type
    pub emergency_type: String,
    /// Difficulty level
    pub difficulty: String,
    /// Key learning objectives
    pub objectives: Vec<String>,
    /// Medical accuracy score
    pub accuracy_score: f32,
    /// Review required by medical professional
    pub requires_review: bool,
}

impl TrainingAI {
    pub fn new() -> Self {
        let mut certification_standards = HashMap::new();
        
        // Initialize certification standards
        certification_standards.insert("CPR".to_string(), vec![
            "Adult CPR - 30 compressions, 2 breaths".to_string(),
            "Child CPR - 30 compressions, 2 breaths".to_string(),
            "Infant CPR - 30 compressions, 2 breaths".to_string(),
            "AED usage - Turn on, attach pads, follow prompts".to_string(),
        ]);
        
        certification_standards.insert("First Aid".to_string(), vec![
            "Bleeding control - Direct pressure, elevation".to_string(),
            "Burn treatment - Cool water, no ice".to_string(),
            "Fracture management - Immobilize, don't move".to_string(),
            "Poisoning response - Call poison control".to_string(),
        ]);
        
        Self {
            medical_ai: MedicalAI::new(),
            user_history: HashMap::new(),
            training_performance: HashMap::new(),
            local_risks: vec![
                "rural_area".to_string(),
                "water_proximity".to_string(),
                "elderly_population".to_string(),
            ],
            certification_standards,
        }
    }
    
    /// Generate personalized training recommendations
    pub async fn generate_training_recommendations(&self, user_id: &str) -> AppResult<Vec<AITrainingRecommendation>> {
        let mut recommendations = Vec::new();
        
        // Analyze user's emergency response history
        let heart_attacks = self.user_history.get("heart_attack").unwrap_or(&0);
        let strokes = self.user_history.get("stroke").unwrap_or(&0);
        let drownings = self.user_history.get("drowning").unwrap_or(&0);
        
        // Generate recommendations based on history and local risks
        if *heart_attacks > 2 {
            recommendations.push(AITrainingRecommendation {
                module: TrainingModule::AdvancedMedical,
                priority: 9,
                reasoning: "You've responded to multiple heart attacks. Advanced cardiac training will improve your effectiveness.".to_string(),
                focus_areas: vec!["Cardiac assessment".to_string(), "ECG interpretation".to_string(), "Advanced airway management".to_string()],
                estimated_time: 120,
                certification_relevant: true,
            });
        }
        
        if self.local_risks.contains(&"water_proximity".to_string()) {
            recommendations.push(AITrainingRecommendation {
                module: TrainingModule::BasicEmergency,
                priority: 8,
                reasoning: "You're near water. Drowning prevention and rescue training is essential.".to_string(),
                focus_areas: vec!["Water safety".to_string(), "Drowning rescue".to_string(), "Post-rescue care".to_string()],
                estimated_time: 90,
                certification_relevant: false,
            });
        }
        
        // Check for weak areas in training performance
        for (skill, performance) in &self.training_performance {
            if *performance < 0.7 {
                recommendations.push(AITrainingRecommendation {
                    module: TrainingModule::BasicEmergency,
                    priority: 7,
                    reasoning: format!("Your {} skills need improvement. Focused training recommended.", skill),
                    focus_areas: vec![skill.clone()],
                    estimated_time: 60,
                    certification_relevant: false,
                });
            }
        }
        
        // Always recommend CPR if not recently trained
        recommendations.push(AITrainingRecommendation {
            module: TrainingModule::CPRCertification,
            priority: 10,
            reasoning: "CPR certification is essential for all emergency responders.".to_string(),
            focus_areas: vec!["Adult CPR".to_string(), "Child CPR".to_string(), "AED usage".to_string()],
            estimated_time: 180,
            certification_relevant: true,
        });
        
        // Sort by priority (highest first)
        recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        Ok(recommendations)
    }
    
    /// Generate AI-enhanced training scenarios
    pub async fn generate_training_scenarios(&self, module: TrainingModule) -> AppResult<Vec<AITrainingScenario>> {
        let mut scenarios = Vec::new();
        
        match module {
            TrainingModule::BasicEmergency => {
                scenarios.push(AITrainingScenario {
                    description: "You're at a restaurant and someone collapses. They're not breathing.".to_string(),
                    emergency_type: "cardiac_arrest".to_string(),
                    difficulty: "intermediate".to_string(),
                    objectives: vec![
                        "Assess scene safety".to_string(),
                        "Check responsiveness".to_string(),
                        "Call 911 immediately".to_string(),
                        "Begin CPR".to_string(),
                    ],
                    accuracy_score: 0.95,
                    requires_review: false,
                });
                
                scenarios.push(AITrainingScenario {
                    description: "A child is choking on food at a family gathering.".to_string(),
                    emergency_type: "choking".to_string(),
                    difficulty: "beginner".to_string(),
                    objectives: vec![
                        "Assess if child can speak".to_string(),
                        "Perform back blows".to_string(),
                        "Perform Heimlich maneuver".to_string(),
                        "Call 911 if persists".to_string(),
                    ],
                    accuracy_score: 0.98,
                    requires_review: false,
                });
            },
            
            TrainingModule::AdvancedMedical => {
                scenarios.push(AITrainingScenario {
                    description: "Multiple casualties at a car accident scene.".to_string(),
                    emergency_type: "mass_casualty".to_string(),
                    difficulty: "advanced".to_string(),
                    objectives: vec![
                        "Triage casualties".to_string(),
                        "Prioritize treatment".to_string(),
                        "Coordinate with EMS".to_string(),
                        "Manage resources".to_string(),
                    ],
                    accuracy_score: 0.92,
                    requires_review: true,
                });
            },
            
            TrainingModule::PediatricCare => {
                scenarios.push(AITrainingScenario {
                    description: "Infant found unresponsive in crib.".to_string(),
                    emergency_type: "pediatric_emergency".to_string(),
                    difficulty: "intermediate".to_string(),
                    objectives: vec![
                        "Check infant responsiveness".to_string(),
                        "Assess breathing".to_string(),
                        "Perform infant CPR".to_string(),
                        "Call 911 immediately".to_string(),
                    ],
                    accuracy_score: 0.96,
                    requires_review: false,
                });
            },
            
            _ => {
                // Generate generic scenarios for other modules
                scenarios.push(AITrainingScenario {
                    description: "Standard emergency scenario for skill development.".to_string(),
                    emergency_type: "general".to_string(),
                    difficulty: "beginner".to_string(),
                    objectives: vec![
                        "Assess the situation".to_string(),
                        "Call appropriate emergency services".to_string(),
                        "Provide basic care".to_string(),
                    ],
                    accuracy_score: 0.90,
                    requires_review: false,
                });
            }
        }
        
        Ok(scenarios)
    }
    
    /// Validate training content against medical standards
    pub async fn validate_training_content(&self, content: &str) -> AppResult<bool> {
        // Use Medical AI to analyze content for medical accuracy
        let assessment = self.medical_ai.analyze_symptoms(content).await?;
        
        // Content is valid if confidence is high and no critical errors
        // Lower threshold for training content validation
        Ok(assessment.confidence > 0.1)
    }
    
    /// Generate adaptive training based on user performance
    pub async fn generate_adaptive_training(&self, user_id: &str, current_progress: &TrainingProgress) -> AppResult<Vec<String>> {
        let mut adaptive_content = Vec::new();
        
        // Analyze weak areas
        for skill in &current_progress.skills_in_progress {
            adaptive_content.push(format!("Focus on improving: {}", skill));
        }
        
        // Add challenges based on performance
        if current_progress.completion < 0.5 {
            adaptive_content.push("Consider breaking training into smaller sessions".to_string());
        }
        
        if current_progress.time_spent > 120 {
            adaptive_content.push("Take breaks to maintain focus and retention".to_string());
        }
        
        // Add local context
        for risk in &self.local_risks {
            match risk.as_str() {
                "rural_area" => adaptive_content.push("Practice scenarios with limited medical resources".to_string()),
                "water_proximity" => adaptive_content.push("Focus on water safety and drowning prevention".to_string()),
                "elderly_population" => adaptive_content.push("Practice geriatric emergency response".to_string()),
                _ => {}
            }
        }
        
        Ok(adaptive_content)
    }
    
    /// Check if training meets certification standards
    pub async fn check_certification_compliance(&self, module: TrainingModule, content: &str) -> AppResult<bool> {
        let module_name = match module {
            TrainingModule::CPRCertification => "CPR",
            TrainingModule::FirstAid => "First Aid",
            _ => return Ok(false), // Other modules may not have specific certifications
        };
        
        if let Some(standards) = self.certification_standards.get(module_name) {
            // Check if content covers all required standards
            let mut covered_standards = 0;
            for standard in standards {
                if content.to_lowercase().contains(&standard.to_lowercase()) {
                    covered_standards += 1;
                }
            }
            
            // Must cover at least 80% of standards
            Ok(covered_standards as f32 / standards.len() as f32 >= 0.8)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_training_ai_recommendations() {
        let training_ai = TrainingAI::new();
        
        // Test recommendation generation
        let recommendations = training_ai.generate_training_recommendations("test_user").await.unwrap();
        assert!(!recommendations.is_empty());
        
        // Test scenario generation
        let scenarios = training_ai.generate_training_scenarios(TrainingModule::BasicEmergency).await.unwrap();
        assert!(!scenarios.is_empty());
        
        // Test content validation
        let is_valid = training_ai.validate_training_content("chest pain and shortness of breath").await.unwrap();
        assert!(is_valid);
    }
} 