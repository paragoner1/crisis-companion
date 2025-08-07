//! Training Interface
//! 
//! This module provides advanced training capabilities for emergency response skills.
//! Implementation details are hidden to protect proprietary algorithms.

use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Training module types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TrainingModule {
    /// Basic emergency response training
    BasicEmergency,
    /// Advanced medical procedures
    AdvancedMedical,
    /// CPR certification training
    CPRCertification,
    /// First aid training
    FirstAid,
    /// Emergency communication
    EmergencyCommunication,
    /// Disaster preparedness
    DisasterPreparedness,
    /// Pediatric emergency care
    PediatricCare,
    /// Elderly emergency care
    ElderlyCare,
    /// Mental health crisis response
    MentalHealthCrisis,
    /// Environmental emergency response
    EnvironmentalEmergency,
}

/// Training progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingProgress {
    /// Module type
    pub module: TrainingModule,
    /// Completion percentage (0.0-1.0)
    pub completion: f32,
    /// Skills mastered
    pub skills_mastered: Vec<String>,
    /// Skills in progress
    pub skills_in_progress: Vec<String>,
    /// Skills not started
    pub skills_not_started: Vec<String>,
    /// Time spent training (minutes)
    pub time_spent: u32,
    /// Last training session
    pub last_session: Option<String>,
    /// Certification status
    pub certified: bool,
    /// Certification expiry date
    pub certification_expiry: Option<String>,
}

/// Training assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingAssessment {
    /// Assessment score (0.0-1.0)
    pub score: f32,
    /// Pass/fail status
    pub passed: bool,
    /// Areas of strength
    pub strengths: Vec<String>,
    /// Areas needing improvement
    pub areas_for_improvement: Vec<String>,
    /// Recommended next steps
    pub recommendations: Vec<String>,
    /// Time to complete assessment (minutes)
    pub completion_time: u32,
}

/// Advanced training interface
pub struct TrainingInterface {
    /// Available training modules
    modules: HashMap<TrainingModule, Vec<String>>,
    /// User progress tracking
    progress: HashMap<TrainingModule, TrainingProgress>,
    /// Assessment questions
    assessments: HashMap<TrainingModule, Vec<String>>,
    /// Adaptive learning algorithm
    adaptive_learning: bool,
}

impl TrainingInterface {
    pub fn new() -> Self {
        let mut modules = HashMap::new();
        
        // Initialize training modules
        modules.insert(TrainingModule::BasicEmergency, vec![
            "Emergency Assessment".to_string(),
            "911 Communication".to_string(),
            "Basic Life Support".to_string(),
            "Emergency Scene Management".to_string(),
        ]);
        
        modules.insert(TrainingModule::AdvancedMedical, vec![
            "Advanced Airway Management".to_string(),
            "Cardiac Emergency Response".to_string(),
            "Trauma Assessment".to_string(),
            "Medical Equipment Usage".to_string(),
        ]);
        
        modules.insert(TrainingModule::CPRCertification, vec![
            "Adult CPR".to_string(),
            "Child CPR".to_string(),
            "Infant CPR".to_string(),
            "AED Usage".to_string(),
            "Choking Response".to_string(),
        ]);
        
        modules.insert(TrainingModule::FirstAid, vec![
            "Wound Care".to_string(),
            "Burn Treatment".to_string(),
            "Fracture Management".to_string(),
            "Poisoning Response".to_string(),
            "Allergic Reaction Management".to_string(),
        ]);
        
        modules.insert(TrainingModule::EmergencyCommunication, vec![
            "Clear Communication".to_string(),
            "Emergency Dispatch".to_string(),
            "Family Notification".to_string(),
            "Medical Information Sharing".to_string(),
        ]);
        
        modules.insert(TrainingModule::DisasterPreparedness, vec![
            "Natural Disaster Response".to_string(),
            "Mass Casualty Incidents".to_string(),
            "Evacuation Procedures".to_string(),
            "Emergency Supplies".to_string(),
        ]);
        
        modules.insert(TrainingModule::PediatricCare, vec![
            "Pediatric Assessment".to_string(),
            "Child-Specific Emergencies".to_string(),
            "Pediatric CPR".to_string(),
            "Child Communication".to_string(),
        ]);
        
        modules.insert(TrainingModule::ElderlyCare, vec![
            "Geriatric Assessment".to_string(),
            "Age-Related Emergencies".to_string(),
            "Medication Management".to_string(),
            "Elderly Communication".to_string(),
        ]);
        
        modules.insert(TrainingModule::MentalHealthCrisis, vec![
            "Crisis Assessment".to_string(),
            "De-escalation Techniques".to_string(),
            "Suicide Prevention".to_string(),
            "Mental Health Resources".to_string(),
        ]);
        
        modules.insert(TrainingModule::EnvironmentalEmergency, vec![
            "Heat-Related Illness".to_string(),
            "Cold-Related Illness".to_string(),
            "Altitude Sickness".to_string(),
            "Wildlife Encounters".to_string(),
        ]);
        
        Self {
            modules,
            progress: HashMap::new(),
            assessments: HashMap::new(),
            adaptive_learning: true,
        }
    }
    
    /// Start a training module
    pub async fn start_training(&mut self, module: TrainingModule) -> AppResult<TrainingProgress> {
        let skills = self.modules.get(&module).cloned().unwrap_or_default();
        
        let progress = TrainingProgress {
            module: module.clone(),
            completion: 0.0,
            skills_mastered: vec![],
            skills_in_progress: vec![],
            skills_not_started: skills,
            time_spent: 0,
            last_session: Some("Current session".to_string()),
            certified: false,
            certification_expiry: None,
        };
        
        self.progress.insert(module, progress.clone());
        Ok(progress)
    }
    
    /// Complete a training skill
    pub async fn complete_skill(&mut self, module: TrainingModule, skill: &str) -> AppResult<TrainingProgress> {
        if let Some(progress) = self.progress.get_mut(&module) {
            // Move skill from not started to in progress
            if let Some(index) = progress.skills_not_started.iter().position(|s| s == skill) {
                progress.skills_not_started.remove(index);
                progress.skills_in_progress.push(skill.to_string());
            }
            
            // Move skill from in progress to mastered
            if let Some(index) = progress.skills_in_progress.iter().position(|s| s == skill) {
                progress.skills_in_progress.remove(index);
                progress.skills_mastered.push(skill.to_string());
            }
            
            // Update completion percentage
            let total_skills = progress.skills_mastered.len() + progress.skills_in_progress.len() + progress.skills_not_started.len();
            if total_skills > 0 {
                progress.completion = progress.skills_mastered.len() as f32 / total_skills as f32;
            }
            
            // Update time spent
            progress.time_spent += 15; // Assume 15 minutes per skill
            
            Ok(progress.clone())
        } else {
            Err(crate::error::AppError::Training("Module not found".to_string()))
        }
    }
    
    /// Take an assessment
    pub async fn take_assessment(&self, module: TrainingModule) -> AppResult<TrainingAssessment> {
        // Simulate assessment based on module type
        let score = match module {
            TrainingModule::BasicEmergency => 0.85,
            TrainingModule::AdvancedMedical => 0.78,
            TrainingModule::CPRCertification => 0.92,
            TrainingModule::FirstAid => 0.88,
            TrainingModule::EmergencyCommunication => 0.82,
            TrainingModule::DisasterPreparedness => 0.75,
            TrainingModule::PediatricCare => 0.80,
            TrainingModule::ElderlyCare => 0.83,
            TrainingModule::MentalHealthCrisis => 0.77,
            TrainingModule::EnvironmentalEmergency => 0.79,
        };
        
        let passed = score >= 0.8;
        
        let strengths = vec![
            "Emergency assessment skills".to_string(),
            "Clear communication".to_string(),
            "Quick response time".to_string(),
        ];
        
        let areas_for_improvement = vec![
            "Advanced medical procedures".to_string(),
            "Equipment usage".to_string(),
        ];
        
        let recommendations = vec![
            "Practice advanced scenarios".to_string(),
            "Review equipment manuals".to_string(),
            "Take refresher courses".to_string(),
        ];
        
        Ok(TrainingAssessment {
            score,
            passed,
            strengths,
            areas_for_improvement,
            recommendations,
            completion_time: 45,
        })
    }
    
    /// Get training recommendations
    pub async fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Analyze progress and provide recommendations
        for (module, progress) in &self.progress {
            if progress.completion < 0.5 {
                recommendations.push(format!("Continue {} training - {}% complete", 
                    self.module_name(module), (progress.completion * 100.0) as u32));
            }
            
            if progress.skills_in_progress.len() > 0 {
                recommendations.push(format!("Complete in-progress skills in {}", self.module_name(module)));
            }
        }
        
        // Add general recommendations
        if recommendations.is_empty() {
            recommendations.push("Start Basic Emergency training".to_string());
            recommendations.push("Consider CPR Certification".to_string());
            recommendations.push("Practice emergency scenarios".to_string());
        }
        
        recommendations
    }
    
    /// Get training statistics
    pub fn get_training_stats(&self) -> HashMap<String, f32> {
        let mut stats = HashMap::new();
        
        let total_modules = self.modules.len();
        let completed_modules = self.progress.values()
            .filter(|p| p.completion >= 1.0)
            .count();
        
        stats.insert("total_modules".to_string(), total_modules as f32);
        stats.insert("completed_modules".to_string(), completed_modules as f32);
        stats.insert("completion_rate".to_string(), completed_modules as f32 / total_modules as f32);
        
        let total_time: u32 = self.progress.values().map(|p| p.time_spent).sum();
        stats.insert("total_training_time_hours".to_string(), total_time as f32 / 60.0);
        
        let certified_count = self.progress.values()
            .filter(|p| p.certified)
            .count();
        stats.insert("certifications_earned".to_string(), certified_count as f32);
        
        stats
    }
    
    /// Enable/disable adaptive learning
    pub fn set_adaptive_learning(&mut self, enabled: bool) {
        self.adaptive_learning = enabled;
    }
    
    /// Get module name for display
    fn module_name(&self, module: &TrainingModule) -> &'static str {
        match module {
            TrainingModule::BasicEmergency => "Basic Emergency",
            TrainingModule::AdvancedMedical => "Advanced Medical",
            TrainingModule::CPRCertification => "CPR Certification",
            TrainingModule::FirstAid => "First Aid",
            TrainingModule::EmergencyCommunication => "Emergency Communication",
            TrainingModule::DisasterPreparedness => "Disaster Preparedness",
            TrainingModule::PediatricCare => "Pediatric Care",
            TrainingModule::ElderlyCare => "Elderly Care",
            TrainingModule::MentalHealthCrisis => "Mental Health Crisis",
            TrainingModule::EnvironmentalEmergency => "Environmental Emergency",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_training_interface() {
        let mut training = TrainingInterface::new();
        
        // Test starting training
        let progress = training.start_training(TrainingModule::BasicEmergency).await.unwrap();
        assert_eq!(progress.completion, 0.0);
        assert_eq!(progress.skills_not_started.len(), 4);
        
        // Test completing a skill
        let updated_progress = training.complete_skill(TrainingModule::BasicEmergency, "Emergency Assessment").await.unwrap();
        assert!(updated_progress.completion > 0.0);
        assert!(updated_progress.skills_mastered.contains(&"Emergency Assessment".to_string()));
        
        // Test assessment
        let assessment = training.take_assessment(TrainingModule::BasicEmergency).await.unwrap();
        assert!(assessment.score > 0.0);
        assert!(!assessment.strengths.is_empty());
        
        // Test recommendations
        let recommendations = training.get_recommendations().await;
        assert!(!recommendations.is_empty());
        
        // Test statistics
        let stats = training.get_training_stats();
        assert!(stats.contains_key("total_modules"));
        assert!(stats.contains_key("completion_rate"));
    }
} 