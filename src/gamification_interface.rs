//! Gamification Interface
//! 
//! This module provides advanced gamification features for user engagement and skill development.
//! Implementation details are hidden to protect proprietary algorithms.

use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Achievement types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementType {
    /// First emergency response
    FirstResponse,
    /// Multiple emergency responses
    EmergencyResponder,
    /// Training completion
    TrainingCompletion,
    /// Skill mastery
    SkillMastery,
    /// Community help
    CommunityHelper,
    /// Life saved
    LifeSaved,
    /// Quick response
    QuickResponse,
    /// Perfect assessment
    PerfectAssessment,
    /// Certification earned
    CertificationEarned,
    /// Streak maintained
    StreakMaintained,
    /// Mentor status
    MentorStatus,
}

/// Achievement levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

/// Achievement data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    /// Achievement type
    pub achievement_type: AchievementType,
    /// Achievement level
    pub level: AchievementLevel,
    /// Achievement name
    pub name: String,
    /// Achievement description
    pub description: String,
    /// XP reward
    pub xp_reward: u32,
    /// Token reward
    pub token_reward: u32,
    /// Unlocked date
    pub unlocked_date: Option<String>,
    /// Progress towards next level
    pub progress: f32, // 0.0-1.0
}

/// User profile for gamification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User level
    pub level: u32,
    /// Current XP
    pub xp: u32,
    /// XP to next level
    pub xp_to_next: u32,
    /// Total tokens earned
    pub tokens: u32,
    /// Emergency responses completed
    pub emergency_responses: u32,
    /// Training modules completed
    pub training_completed: u32,
    /// Skills mastered
    pub skills_mastered: u32,
    /// Lives potentially saved
    pub lives_saved: u32,
    /// Current streak (days)
    pub current_streak: u32,
    /// Longest streak (days)
    pub longest_streak: u32,
    /// Achievements unlocked
    pub achievements: Vec<Achievement>,
    /// Rank in community
    pub community_rank: String,
    /// Mentor status
    pub is_mentor: bool,
}

/// Leaderboard entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    /// User identifier
    pub user_id: String,
    /// User name
    pub name: String,
    /// Total XP
    pub xp: u32,
    /// Level
    pub level: u32,
    /// Lives saved
    pub lives_saved: u32,
    /// Rank
    pub rank: u32,
}

/// Advanced gamification interface
pub struct GamificationInterface {
    /// User profiles
    profiles: HashMap<String, UserProfile>,
    /// Achievement definitions
    achievements: HashMap<AchievementType, Vec<Achievement>>,
    /// Leaderboard data
    leaderboard: Vec<LeaderboardEntry>,
    /// XP multipliers
    xp_multipliers: HashMap<String, f32>,
    /// Daily challenges
    daily_challenges: Vec<String>,
}

impl GamificationInterface {
    pub fn new() -> Self {
        let mut achievements = HashMap::new();
        
        // Initialize achievements
        achievements.insert(AchievementType::FirstResponse, vec![
            Achievement {
                achievement_type: AchievementType::FirstResponse,
                level: AchievementLevel::Bronze,
                name: "First Responder".to_string(),
                description: "Complete your first emergency response".to_string(),
                xp_reward: 100,
                token_reward: 10,
                unlocked_date: None,
                progress: 0.0,
            },
        ]);
        
        achievements.insert(AchievementType::EmergencyResponder, vec![
            Achievement {
                achievement_type: AchievementType::EmergencyResponder,
                level: AchievementLevel::Bronze,
                name: "Emergency Responder".to_string(),
                description: "Complete 10 emergency responses".to_string(),
                xp_reward: 500,
                token_reward: 50,
                unlocked_date: None,
                progress: 0.0,
            },
            Achievement {
                achievement_type: AchievementType::EmergencyResponder,
                level: AchievementLevel::Silver,
                name: "Emergency Expert".to_string(),
                description: "Complete 50 emergency responses".to_string(),
                xp_reward: 1000,
                token_reward: 100,
                unlocked_date: None,
                progress: 0.0,
            },
        ]);
        
        achievements.insert(AchievementType::LifeSaved, vec![
            Achievement {
                achievement_type: AchievementType::LifeSaved,
                level: AchievementLevel::Gold,
                name: "Life Saver".to_string(),
                description: "Potentially save a life through emergency response".to_string(),
                xp_reward: 2000,
                token_reward: 200,
                unlocked_date: None,
                progress: 0.0,
            },
        ]);
        
        achievements.insert(AchievementType::TrainingCompletion, vec![
            Achievement {
                achievement_type: AchievementType::TrainingCompletion,
                level: AchievementLevel::Bronze,
                name: "Trainee".to_string(),
                description: "Complete your first training module".to_string(),
                xp_reward: 200,
                token_reward: 20,
                unlocked_date: None,
                progress: 0.0,
            },
        ]);
        
        achievements.insert(AchievementType::SkillMastery, vec![
            Achievement {
                achievement_type: AchievementType::SkillMastery,
                level: AchievementLevel::Silver,
                name: "Skill Master".to_string(),
                description: "Master 10 different skills".to_string(),
                xp_reward: 1500,
                token_reward: 150,
                unlocked_date: None,
                progress: 0.0,
            },
        ]);
        
        achievements.insert(AchievementType::StreakMaintained, vec![
            Achievement {
                achievement_type: AchievementType::StreakMaintained,
                level: AchievementLevel::Bronze,
                name: "Consistent Helper".to_string(),
                description: "Maintain a 7-day streak".to_string(),
                xp_reward: 300,
                token_reward: 30,
                unlocked_date: None,
                progress: 0.0,
            },
        ]);
        
        achievements.insert(AchievementType::MentorStatus, vec![
            Achievement {
                achievement_type: AchievementType::MentorStatus,
                level: AchievementLevel::Platinum,
                name: "Community Mentor".to_string(),
                description: "Help 10 other users improve their skills".to_string(),
                xp_reward: 3000,
                token_reward: 300,
                unlocked_date: None,
                progress: 0.0,
            },
        ]);
        
        Self {
            profiles: HashMap::new(),
            achievements,
            leaderboard: Vec::new(),
            xp_multipliers: HashMap::new(),
            daily_challenges: vec![
                "Complete 3 emergency assessments".to_string(),
                "Practice CPR for 10 minutes".to_string(),
                "Review first aid procedures".to_string(),
                "Help someone with emergency training".to_string(),
                "Update emergency contacts".to_string(),
            ],
        }
    }
    
    /// Create or get user profile
    pub async fn get_user_profile(&mut self, user_id: &str) -> UserProfile {
        if let Some(profile) = self.profiles.get(user_id) {
            profile.clone()
        } else {
            let new_profile = UserProfile {
                level: 1,
                xp: 0,
                xp_to_next: 100,
                tokens: 0,
                emergency_responses: 0,
                training_completed: 0,
                skills_mastered: 0,
                lives_saved: 0,
                current_streak: 0,
                longest_streak: 0,
                achievements: Vec::new(),
                community_rank: "Newcomer".to_string(),
                is_mentor: false,
            };
            self.profiles.insert(user_id.to_string(), new_profile.clone());
            new_profile
        }
    }
    
    /// Award XP for an action
    pub async fn award_xp(&mut self, user_id: &str, action: &str, base_xp: u32) -> AppResult<UserProfile> {
        let profile = self.get_user_profile(user_id).await;
        let mut updated_profile = profile.clone();
        
        // Apply XP multiplier if available
        let multiplier = self.xp_multipliers.get(action).unwrap_or(&1.0);
        let final_xp = (base_xp as f32 * multiplier) as u32;
        
        updated_profile.xp += final_xp;
        
        // Check for level up
        while updated_profile.xp >= updated_profile.xp_to_next {
            updated_profile.level += 1;
            updated_profile.xp -= updated_profile.xp_to_next;
            updated_profile.xp_to_next = self.calculate_xp_for_level(updated_profile.level);
        }
        
        self.profiles.insert(user_id.to_string(), updated_profile.clone());
        Ok(updated_profile)
    }
    
    /// Award tokens
    pub async fn award_tokens(&mut self, user_id: &str, amount: u32) -> AppResult<UserProfile> {
        let mut profile = self.get_user_profile(user_id).await;
        profile.tokens += amount;
        self.profiles.insert(user_id.to_string(), profile.clone());
        Ok(profile)
    }
    
    /// Record emergency response
    pub async fn record_emergency_response(&mut self, user_id: &str, emergency_type: &str, success: bool) -> AppResult<UserProfile> {
        let mut profile = self.get_user_profile(user_id).await;
        
        profile.emergency_responses += 1;
        if success {
            profile.lives_saved += 1;
        }
        
        // Award XP
        let base_xp = if success { 200 } else { 50 };
        let updated_profile = self.award_xp(user_id, "emergency_response", base_xp).await?;
        
        // Check for achievements
        self.check_achievements(user_id).await?;
        
        Ok(updated_profile)
    }
    
    /// Record training completion
    pub async fn record_training_completion(&mut self, user_id: &str, module: &str) -> AppResult<UserProfile> {
        let mut profile = self.get_user_profile(user_id).await;
        profile.training_completed += 1;
        
        // Award XP
        let updated_profile = self.award_xp(user_id, "training_completion", 150).await?;
        
        // Check for achievements
        self.check_achievements(user_id).await?;
        
        Ok(updated_profile)
    }
    
    /// Record skill mastery
    pub async fn record_skill_mastery(&mut self, user_id: &str, skill: &str) -> AppResult<UserProfile> {
        let mut profile = self.get_user_profile(user_id).await;
        profile.skills_mastered += 1;
        
        // Award XP
        let updated_profile = self.award_xp(user_id, "skill_mastery", 100).await?;
        
        // Check for achievements
        self.check_achievements(user_id).await?;
        
        Ok(updated_profile)
    }
    
    /// Update streak
    pub async fn update_streak(&mut self, user_id: &str, active_today: bool) -> AppResult<UserProfile> {
        let mut profile = self.get_user_profile(user_id).await;
        
        if active_today {
            profile.current_streak += 1;
            if profile.current_streak > profile.longest_streak {
                profile.longest_streak = profile.current_streak;
            }
        } else {
            profile.current_streak = 0;
        }
        
        self.profiles.insert(user_id.to_string(), profile.clone());
        
        // Check for streak achievements
        self.check_achievements(user_id).await?;
        
        Ok(profile)
    }
    
    /// Check and award achievements
    async fn check_achievements(&mut self, user_id: &str) -> AppResult<()> {
        let profile = self.get_user_profile(user_id).await;
        
        // Check FirstResponse achievement
        if profile.emergency_responses >= 1 {
            self.award_achievement(user_id, AchievementType::FirstResponse, AchievementLevel::Bronze).await?;
        }
        
        // Check EmergencyResponder achievements
        if profile.emergency_responses >= 10 {
            self.award_achievement(user_id, AchievementType::EmergencyResponder, AchievementLevel::Bronze).await?;
        }
        if profile.emergency_responses >= 50 {
            self.award_achievement(user_id, AchievementType::EmergencyResponder, AchievementLevel::Silver).await?;
        }
        
        // Check LifeSaved achievement
        if profile.lives_saved >= 1 {
            self.award_achievement(user_id, AchievementType::LifeSaved, AchievementLevel::Gold).await?;
        }
        
        // Check TrainingCompletion achievement
        if profile.training_completed >= 1 {
            self.award_achievement(user_id, AchievementType::TrainingCompletion, AchievementLevel::Bronze).await?;
        }
        
        // Check SkillMastery achievement
        if profile.skills_mastered >= 10 {
            self.award_achievement(user_id, AchievementType::SkillMastery, AchievementLevel::Silver).await?;
        }
        
        // Check StreakMaintained achievement
        if profile.current_streak >= 7 {
            self.award_achievement(user_id, AchievementType::StreakMaintained, AchievementLevel::Bronze).await?;
        }
        
        Ok(())
    }
    
    /// Award achievement
    async fn award_achievement(&mut self, user_id: &str, achievement_type: AchievementType, level: AchievementLevel) -> AppResult<()> {
        let mut profile = self.get_user_profile(user_id).await;
        
        // Check if achievement already unlocked
        let already_unlocked = profile.achievements.iter().any(|a| 
            a.achievement_type == achievement_type && a.level == level
        );
        
        if !already_unlocked {
            if let Some(achievements) = self.achievements.get(&achievement_type) {
                if let Some(achievement) = achievements.iter().find(|a| a.level == level) {
                    let mut new_achievement = achievement.clone();
                    new_achievement.unlocked_date = Some("Today".to_string());
                    new_achievement.progress = 1.0;
                    
                    profile.achievements.push(new_achievement.clone());
                    
                    // Award XP and tokens
                    profile.xp += achievement.xp_reward;
                    profile.tokens += achievement.token_reward;
                    
                    self.profiles.insert(user_id.to_string(), profile);
                }
            }
        }
        
        Ok(())
    }
    
    /// Get leaderboard
    pub async fn get_leaderboard(&self) -> Vec<LeaderboardEntry> {
        let mut entries: Vec<LeaderboardEntry> = self.profiles.iter().map(|(user_id, profile)| {
            LeaderboardEntry {
                user_id: user_id.clone(),
                name: format!("User {}", user_id),
                xp: profile.xp,
                level: profile.level,
                lives_saved: profile.lives_saved,
                rank: 0, // Will be set after sorting
            }
        }).collect();
        
        // Sort by XP (descending)
        entries.sort_by(|a, b| b.xp.cmp(&a.xp));
        
        // Set ranks
        for (index, entry) in entries.iter_mut().enumerate() {
            entry.rank = index as u32 + 1;
        }
        
        entries
    }
    
    /// Get daily challenges
    pub async fn get_daily_challenges(&self) -> Vec<String> {
        self.daily_challenges.clone()
    }
    
    /// Calculate XP required for a level
    fn calculate_xp_for_level(&self, level: u32) -> u32 {
        // Exponential growth: each level requires more XP
        100 * level * level
    }
    
    /// Get gamification statistics
    pub fn get_gamification_stats(&self) -> HashMap<String, f32> {
        let mut stats = HashMap::new();
        
        let total_users = self.profiles.len();
        let total_xp: u32 = self.profiles.values().map(|p| p.xp).sum();
        let total_lives_saved: u32 = self.profiles.values().map(|p| p.lives_saved).sum();
        let total_achievements: usize = self.profiles.values().map(|p| p.achievements.len()).sum();
        
        stats.insert("total_users".to_string(), total_users as f32);
        stats.insert("total_xp".to_string(), total_xp as f32);
        stats.insert("total_lives_saved".to_string(), total_lives_saved as f32);
        stats.insert("total_achievements".to_string(), total_achievements as f32);
        stats.insert("average_xp_per_user".to_string(), total_xp as f32 / total_users.max(1) as f32);
        stats.insert("average_lives_saved_per_user".to_string(), total_lives_saved as f32 / total_users.max(1) as f32);
        
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_gamification_interface() {
        let mut gamification = GamificationInterface::new();
        
        // Test user profile creation
        let profile = gamification.get_user_profile("test_user").await;
        assert_eq!(profile.level, 1);
        assert_eq!(profile.xp, 0);
        
        // Test XP award
        let updated_profile = gamification.award_xp("test_user", "emergency_response", 100).await.unwrap();
        assert_eq!(updated_profile.xp, 100);
        
        // Test emergency response recording
        let response_profile = gamification.record_emergency_response("test_user", "drowning", true).await.unwrap();
        assert_eq!(response_profile.emergency_responses, 1);
        assert_eq!(response_profile.lives_saved, 1);
        
        // Test training completion
        let training_profile = gamification.record_training_completion("test_user", "CPR").await.unwrap();
        assert_eq!(training_profile.training_completed, 1);
        
        // Test skill mastery
        let skill_profile = gamification.record_skill_mastery("test_user", "CPR").await.unwrap();
        assert_eq!(skill_profile.skills_mastered, 1);
        
        // Test leaderboard
        let leaderboard = gamification.get_leaderboard().await;
        assert!(!leaderboard.is_empty());
        
        // Test statistics
        let stats = gamification.get_gamification_stats();
        assert!(stats.contains_key("total_users"));
        assert!(stats.contains_key("total_xp"));
    }
} 