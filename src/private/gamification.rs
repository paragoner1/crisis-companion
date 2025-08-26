//! Gamification Module
//! 
//! Implements the SOS Hero gamification system with XP, levels,
//! achievements, and progression tracking.

use crate::error::AppResult;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GamificationSystem {
    user_profile: UserProfile,
    achievements: HashMap<String, Achievement>,
    leaderboard: Vec<LeaderboardEntry>,
    hero_levels: Vec<HeroLevel>,
    xp_rewards: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub current_level: u32,
    pub current_xp: u32,
    pub total_xp: u32,
    pub achievements_earned: Vec<String>,
    pub emergency_interventions: u32,
    pub trusted_contacts: u32,
    pub features_mastered: u32,
    pub join_date: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub xp_reward: u32,
    pub category: AchievementCategory,
    pub requirements: Vec<AchievementRequirement>,
    pub earned_at: Option<DateTime<Utc>>,
    pub icon: String,
}

#[derive(Debug, Clone)]
pub struct LeaderboardEntry {
    pub user_id: String,
    pub username: String,
    pub level: u32,
    pub xp: u32,
    pub achievements: u32,
    pub interventions: u32,
    pub rank: u32,
}

#[derive(Debug, Clone)]
pub struct HeroLevel {
    pub level: u32,
    pub name: String,
    pub xp_required: u32,
    pub rewards: LevelRewards,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct LevelRewards {
    pub bonk_tokens: u32,
    pub skr_tokens: u32,
    pub title: String,
    pub badge: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AchievementCategory {
    EmergencyResponse,
    SafetyFeatures,
    Community,
    Learning,
    Mastery,
}

#[derive(Debug, Clone)]
pub enum AchievementRequirement {
    EmergencyInterventions(u32),
    TrustedContacts(u32),
    FeaturesMastered(u32),
    LearningModules(u32),
    ResponseTime(u32), // seconds
    StreakDays(u32),
}

impl GamificationSystem {
    pub fn new(user_id: String, username: String) -> Self {
        let mut system = GamificationSystem {
            user_profile: UserProfile {
                user_id: user_id.clone(),
                username,
                current_level: 1,
                current_xp: 0,
                total_xp: 0,
                achievements_earned: Vec::new(),
                emergency_interventions: 0,
                trusted_contacts: 0,
                features_mastered: 0,
                join_date: Utc::now(),
                last_active: Utc::now(),
            },
            achievements: HashMap::new(),
            leaderboard: Vec::new(),
            hero_levels: Vec::new(),
            xp_rewards: HashMap::new(),
        };
        
        system.initialize_achievements();
        system.initialize_hero_levels();
        system.initialize_xp_rewards();
        
        system
    }
    
    /// Award XP for an action
    pub fn award_xp(&mut self, action: &str, amount: u32) -> Result<u32, String> {
        self.user_profile.current_xp += amount;
        self.user_profile.total_xp += amount;
        self.user_profile.last_active = Utc::now();
        
        // Check for level up
        let new_level = self.check_level_up();
        if new_level > self.user_profile.current_level {
            self.user_profile.current_level = new_level;
            self.on_level_up(new_level)?;
        }
        
        // Check for achievements
        self.check_achievements()?;
        
        Ok(self.user_profile.current_xp)
    }
    
    /// Award XP for emergency intervention
    pub fn award_emergency_intervention(&mut self, emergency_type: &str, response_time: u32) -> Result<u32, String> {
        self.user_profile.emergency_interventions += 1;
        
        // Base XP for intervention
        let base_xp = 200;
        
        // Bonus for fast response
        let time_bonus = if response_time < 30 { 100 } else if response_time < 60 { 50 } else { 0 };
        
        // Bonus for critical emergencies
        let emergency_bonus = match emergency_type {
            "drowning" | "heart_attack" | "stroke" | "choking" | "allergic" => 100,
            "bleeding" | "unconscious" | "poisoning" | "trauma" => 75,
            _ => 50,
        };
        
        let total_xp = base_xp + time_bonus + emergency_bonus;
        
        self.award_xp("emergency_intervention", total_xp)
    }
    
    /// Award XP for safety feature usage
    pub fn award_safety_feature(&mut self, feature: &str) -> Result<u32, String> {
        let xp = match feature {
            "silent_sos" => 50,
            "crash_detection" => 75,
            "trusted_network" => 25,
            "location_sharing" => 25,
            _ => 10,
        };
        
        self.award_xp("safety_feature", xp)
    }
    
    /// Award XP for learning
    pub fn award_learning(&mut self, module: &str) -> Result<u32, String> {
        let xp = match module {
            "cpr" => 100,
            "heimlich" => 75,
            "first_aid" => 50,
            "aed" => 75,
            "emergency_protocols" => 25,
            _ => 25,
        };
        
        self.award_xp("learning", xp)
    }
    
    /// Award XP for community features
    pub fn award_community(&mut self, action: &str) -> Result<u32, String> {
        let xp = match action {
            "add_trusted_contact" => 25,
            "network_verification" => 50,
            "community_help" => 100,
            "share_knowledge" => 75,
            _ => 10,
        };
        
        self.award_xp("community", xp)
    }
    
    /// Get user profile
    pub fn get_user_profile(&self) -> &UserProfile {
        &self.user_profile
    }
    
    /// Get achievements
    pub fn get_achievements(&self) -> &HashMap<String, Achievement> {
        &self.achievements
    }
    
    /// Get earned achievements
    pub fn get_earned_achievements(&self) -> Vec<&Achievement> {
        self.achievements.values()
            .filter(|a| a.earned_at.is_some())
            .collect()
    }
    
    /// Get available achievements
    pub fn get_available_achievements(&self) -> Vec<&Achievement> {
        self.achievements.values()
            .filter(|a| a.earned_at.is_none())
            .collect()
    }
    
    /// Get hero levels
    pub fn get_hero_levels(&self) -> &Vec<HeroLevel> {
        &self.hero_levels
    }
    
    /// Get current hero level info
    pub fn get_current_hero_level(&self) -> Option<&HeroLevel> {
        self.hero_levels.iter().find(|l| l.level == self.user_profile.current_level)
    }
    
    /// Get next hero level info
    pub fn get_next_hero_level(&self) -> Option<&HeroLevel> {
        self.hero_levels.iter().find(|l| l.level == self.user_profile.current_level + 1)
    }
    
    /// Get progress to next level
    pub fn get_level_progress(&self) -> (u32, u32) {
        let current_level_xp = self.get_current_hero_level()
            .map(|l| l.xp_required)
            .unwrap_or(0);
        
        let next_level_xp = self.get_next_hero_level()
            .map(|l| l.xp_required)
            .unwrap_or(current_level_xp);
        
        let progress = self.user_profile.current_xp - current_level_xp;
        let required = next_level_xp - current_level_xp;
        
        (progress, required)
    }
    
    fn initialize_achievements(&mut self) {
        let achievements = vec![
            Achievement {
                id: "first_responder".to_string(),
                name: "First Responder".to_string(),
                description: "Complete your first emergency intervention".to_string(),
                xp_reward: 150,
                category: AchievementCategory::EmergencyResponse,
                requirements: vec![AchievementRequirement::EmergencyInterventions(1)],
                earned_at: None,
                icon: "🆘".to_string(),
            },
            Achievement {
                id: "safety_guardian".to_string(),
                name: "Safety Guardian".to_string(),
                description: "Set up all safety features".to_string(),
                xp_reward: 200,
                category: AchievementCategory::SafetyFeatures,
                requirements: vec![AchievementRequirement::FeaturesMastered(5)],
                earned_at: None,
                icon: "🛡️".to_string(),
            },
            Achievement {
                id: "emergency_expert".to_string(),
                name: "Emergency Expert".to_string(),
                description: "Complete 10 emergency interventions".to_string(),
                xp_reward: 300,
                category: AchievementCategory::EmergencyResponse,
                requirements: vec![AchievementRequirement::EmergencyInterventions(10)],
                earned_at: None,
                icon: "🏥".to_string(),
            },
            Achievement {
                id: "network_builder".to_string(),
                name: "Network Builder".to_string(),
                description: "Add 5 trusted contacts".to_string(),
                xp_reward: 100,
                category: AchievementCategory::Community,
                requirements: vec![AchievementRequirement::TrustedContacts(5)],
                earned_at: None,
                icon: "👥".to_string(),
            },
            Achievement {
                id: "feature_master".to_string(),
                name: "Feature Master".to_string(),
                description: "Master all app features".to_string(),
                xp_reward: 150,
                category: AchievementCategory::Mastery,
                requirements: vec![AchievementRequirement::FeaturesMastered(10)],
                earned_at: None,
                icon: "⭐".to_string(),
            },
            Achievement {
                id: "quick_responder".to_string(),
                name: "Quick Responder".to_string(),
                description: "Respond to emergency in under 30 seconds".to_string(),
                xp_reward: 100,
                category: AchievementCategory::EmergencyResponse,
                requirements: vec![AchievementRequirement::ResponseTime(30)],
                earned_at: None,
                icon: "⚡".to_string(),
            },
            Achievement {
                id: "learning_champion".to_string(),
                name: "Learning Champion".to_string(),
                description: "Complete all learning modules".to_string(),
                xp_reward: 200,
                category: AchievementCategory::Learning,
                requirements: vec![AchievementRequirement::LearningModules(5)],
                earned_at: None,
                icon: "📚".to_string(),
            },
            Achievement {
                id: "community_hero".to_string(),
                name: "Community Hero".to_string(),
                description: "Help 5 people in your network".to_string(),
                xp_reward: 250,
                category: AchievementCategory::Community,
                requirements: vec![AchievementRequirement::EmergencyInterventions(5)],
                earned_at: None,
                icon: "🦸".to_string(),
            },
        ];
        
        for achievement in achievements {
            self.achievements.insert(achievement.id.clone(), achievement);
        }
    }
    
    fn initialize_hero_levels(&mut self) {
        self.hero_levels = vec![
            HeroLevel {
                level: 1,
                name: "Novice".to_string(),
                xp_required: 0,
                rewards: LevelRewards {
                    bonk_tokens: 0,
                    skr_tokens: 0,
                    title: "Novice Responder".to_string(),
                    badge: "🆕".to_string(),
                },
                description: "Just getting started with emergency response".to_string(),
            },
            HeroLevel {
                level: 2,
                name: "Apprentice".to_string(),
                xp_required: 100,
                rewards: LevelRewards {
                    bonk_tokens: 50,
                    skr_tokens: 25,
                    title: "Apprentice Responder".to_string(),
                    badge: "📖".to_string(),
                },
                description: "Learning the basics of emergency response".to_string(),
            },
            HeroLevel {
                level: 3,
                name: "Responder".to_string(),
                xp_required: 300,
                rewards: LevelRewards {
                    bonk_tokens: 100,
                    skr_tokens: 50,
                    title: "Emergency Responder".to_string(),
                    badge: "🆘".to_string(),
                },
                description: "Ready to respond to emergencies".to_string(),
            },
            HeroLevel {
                level: 4,
                name: "Guardian".to_string(),
                xp_required: 600,
                rewards: LevelRewards {
                    bonk_tokens: 150,
                    skr_tokens: 75,
                    title: "Safety Guardian".to_string(),
                    badge: "🛡️".to_string(),
                },
                description: "Protecting your community".to_string(),
            },
            HeroLevel {
                level: 5,
                name: "Expert".to_string(),
                xp_required: 1000,
                rewards: LevelRewards {
                    bonk_tokens: 250,
                    skr_tokens: 125,
                    title: "Emergency Expert".to_string(),
                    badge: "🏥".to_string(),
                },
                description: "Highly skilled in emergency response".to_string(),
            },
            HeroLevel {
                level: 6,
                name: "Master".to_string(),
                xp_required: 1500,
                rewards: LevelRewards {
                    bonk_tokens: 350,
                    skr_tokens: 175,
                    title: "Emergency Master".to_string(),
                    badge: "🎯".to_string(),
                },
                description: "Master of emergency protocols".to_string(),
            },
            HeroLevel {
                level: 7,
                name: "Champion".to_string(),
                xp_required: 2200,
                rewards: LevelRewards {
                    bonk_tokens: 500,
                    skr_tokens: 250,
                    title: "Emergency Champion".to_string(),
                    badge: "🏆".to_string(),
                },
                description: "Champion of emergency response".to_string(),
            },
            HeroLevel {
                level: 8,
                name: "Legend".to_string(),
                xp_required: 3000,
                rewards: LevelRewards {
                    bonk_tokens: 750,
                    skr_tokens: 375,
                    title: "Emergency Legend".to_string(),
                    badge: "👑".to_string(),
                },
                description: "Legendary emergency responder".to_string(),
            },
        ];
    }
    
    fn initialize_xp_rewards(&mut self) {
        self.xp_rewards.insert("app_setup".to_string(), 50);
        self.xp_rewards.insert("microphone_permission".to_string(), 25);
        self.xp_rewards.insert("location_permission".to_string(), 25);
        self.xp_rewards.insert("contacts_permission".to_string(), 25);
        self.xp_rewards.insert("emergency_contacts_setup".to_string(), 25);
        self.xp_rewards.insert("voice_recognition_calibration".to_string(), 25);
        self.xp_rewards.insert("noise_filtering_setup".to_string(), 25);
        self.xp_rewards.insert("adaptive_training_enabled".to_string(), 25);
        self.xp_rewards.insert("hybrid_mode_configuration".to_string(), 25);
        self.xp_rewards.insert("offline_database_setup".to_string(), 25);
    }
    
    fn check_level_up(&self) -> u32 {
        for level in &self.hero_levels {
            if self.user_profile.current_xp >= level.xp_required {
                continue;
            } else {
                return level.level - 1;
            }
        }
        self.hero_levels.last().map(|l| l.level).unwrap_or(1)
    }
    
    fn on_level_up(&mut self, new_level: u32) -> Result<(), String> {
        if let Some(level_info) = self.hero_levels.iter().find(|l| l.level == new_level) {
            // Award level rewards
            // In real implementation, this would award BONK/SKR tokens
            Ok(())
        } else {
            Err("Invalid level".to_string())
        }
    }
    
    fn check_achievements(&mut self) -> Result<(), String> {
        let mut achievements_to_award = Vec::new();
        
        for achievement in self.achievements.values() {
            if achievement.earned_at.is_some() {
                continue; // Already earned
            }
            
            if self.has_met_requirements(achievement) {
                achievements_to_award.push(achievement.id.clone());
            }
        }
        
        // Now award the achievements
        for achievement_id in achievements_to_award {
            let xp_reward = if let Some(achievement) = self.achievements.get(&achievement_id) {
                achievement.xp_reward
            } else {
                continue;
            };
            
            if let Some(achievement) = self.achievements.get_mut(&achievement_id) {
                achievement.earned_at = Some(Utc::now());
                self.user_profile.achievements_earned.push(achievement_id.clone());
            }
            
            // Award XP for achievement
            self.award_xp("achievement", xp_reward)?;
        }
        
        Ok(())
    }
    
    fn has_met_requirements(&self, achievement: &Achievement) -> bool {
        for requirement in &achievement.requirements {
            match requirement {
                AchievementRequirement::EmergencyInterventions(count) => {
                    if self.user_profile.emergency_interventions < *count {
                        return false;
                    }
                },
                AchievementRequirement::TrustedContacts(count) => {
                    if self.user_profile.trusted_contacts < *count {
                        return false;
                    }
                },
                AchievementRequirement::FeaturesMastered(count) => {
                    if self.user_profile.features_mastered < *count {
                        return false;
                    }
                },
                AchievementRequirement::LearningModules(count) => {
                    // This would need to track learning modules separately
                    return false;
                },
                AchievementRequirement::ResponseTime(_) => {
                    // This would need to track response times separately
                    return false;
                },
                AchievementRequirement::StreakDays(_) => {
                    // This would need to track daily streaks separately
                    return false;
                },
            }
        }
        
        true
    }
}
