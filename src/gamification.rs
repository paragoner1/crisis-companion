use crate::{
    types::*,
    error::AppError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use uuid::Uuid;
use tracing::{info, warn, error};
use chrono::{DateTime, Utc};

/// Hero level system for SOS Hero app
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum HeroLevel {
    Novice = 1,
    Trainee = 2,
    Responder = 3,
    Guardian = 4,
    Protector = 5,
    Defender = 6,
    Sentinel = 7,
    Champion = 8,
    GuardianAngel = 9,
    Legend = 10,
}

impl HeroLevel {
    pub fn from_experience(exp: u32) -> Self {
        match exp {
            0..=99 => HeroLevel::Novice,
            100..=299 => HeroLevel::Trainee,
            300..=599 => HeroLevel::Responder,
            600..=999 => HeroLevel::Guardian,
            1000..=1499 => HeroLevel::Protector,
            1500..=2099 => HeroLevel::Defender,
            2100..=2799 => HeroLevel::Sentinel,
            2800..=3599 => HeroLevel::Champion,
            3600..=4499 => HeroLevel::GuardianAngel,
            4500.. => HeroLevel::Legend,
        }
    }
    
    pub fn experience_required(&self) -> u32 {
        match self {
            HeroLevel::Novice => 0,
            HeroLevel::Trainee => 100,
            HeroLevel::Responder => 300,
            HeroLevel::Guardian => 600,
            HeroLevel::Protector => 1000,
            HeroLevel::Defender => 1500,
            HeroLevel::Sentinel => 2100,
            HeroLevel::Champion => 2800,
            HeroLevel::GuardianAngel => 3600,
            HeroLevel::Legend => 4500,
        }
    }
    
    pub fn title(&self) -> &'static str {
        match self {
            HeroLevel::Novice => "Novice Hero",
            HeroLevel::Trainee => "Trainee Hero",
            HeroLevel::Responder => "Emergency Responder",
            HeroLevel::Guardian => "Safety Guardian",
            HeroLevel::Protector => "Life Protector",
            HeroLevel::Defender => "Community Defender",
            HeroLevel::Sentinel => "Safety Sentinel",
            HeroLevel::Champion => "Emergency Champion",
            HeroLevel::GuardianAngel => "Guardian Angel",
            HeroLevel::Legend => "Life-Saving Legend",
        }
    }
    
    pub fn badge(&self) -> &'static str {
        match self {
            HeroLevel::Novice => "🆕",
            HeroLevel::Trainee => "📚",
            HeroLevel::Responder => "🚨",
            HeroLevel::Guardian => "🛡️",
            HeroLevel::Protector => "💪",
            HeroLevel::Defender => "🏘️",
            HeroLevel::Sentinel => "👁️",
            HeroLevel::Champion => "🏆",
            HeroLevel::GuardianAngel => "👼",
            HeroLevel::Legend => "⭐",
        }
    }
}

/// Achievement types for SOS Hero
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementType {
    // Setup Achievements
    AppSetup,
    PermissionsGranted,
    TrustedNetworkCreated,
    EmergencyContactsAdded,
    LocationEnabled,
    
    // Learning Achievements
    CPRCertified,
    FirstAidTrained,
    AEDTrained,
    HeimlichLearned,
    StrokeRecognition,
    DrowningRescue,
    SeizureResponse,
    AllergicReaction,
    BurnTreatment,
    DiabeticEmergency,
    PoisoningResponse,
    TraumaAssessment,
    
    // Intervention Achievements
    FirstIntervention,
    MultipleInterventions,
    SuccessfulRescue,
    CommunityHelper,
    EmergencyCoordinator,
    LifeSaver,
    
    // Network Growth Achievements
    InvitedFriends,
    NetworkExpansion,
    CommunityBuilder,
    SafetyAmbassador,
    
    // Special Achievements
    OfflineHero,
    QuickResponder,
    CalmUnderPressure,
    TeamPlayer,
    KnowledgeSharer,
}

/// Achievement data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: Uuid,
    pub achievement_type: AchievementType,
    pub name: String,
    pub description: String,
    pub experience_reward: u32,
    pub token_reward: u32,
    pub badge: String,
    pub unlocked_at: Option<DateTime<Utc>>,
    pub is_unlocked: bool,
}

/// Reward system for SOS Hero
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub experience_points: u32,
    pub bonk_tokens: u32,
    pub skr_tokens: u32,
    pub hero_badges: Vec<String>,
    pub unlocked_at: Option<DateTime<Utc>>,
}

/// User progress and gamification data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroProfile {
    pub user_id: Uuid,
    pub hero_level: HeroLevel,
    pub experience_points: u32,
    pub total_interventions: u32,
    pub successful_rescues: u32,
    pub learning_modules_completed: u32,
    pub network_size: u32,
    pub achievements: Vec<Achievement>,
    pub rewards: Vec<Reward>,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

/// Gamification manager for SOS Hero
#[derive(Debug)]
pub struct GamificationManager {
    hero_profiles: Arc<RwLock<HashMap<Uuid, HeroProfile>>>,
    achievements: Vec<Achievement>,
    rewards: Vec<Reward>,
}

impl GamificationManager {
    pub fn new() -> Self {
        let achievements = Self::initialize_achievements();
        let rewards = Self::initialize_rewards();
        
        Self {
            hero_profiles: Arc::new(RwLock::new(HashMap::new())),
            achievements,
            rewards,
        }
    }
    
    /// Initialize all achievements
    fn initialize_achievements() -> Vec<Achievement> {
        vec![
            // Setup Achievements
            Achievement {
                id: Uuid::new_v4(),
                achievement_type: AchievementType::AppSetup,
                name: "First Steps".to_string(),
                description: "Complete initial app setup".to_string(),
                experience_reward: 50,
                token_reward: 100,
                badge: "🆕".to_string(),
                unlocked_at: None,
                is_unlocked: false,
            },
            Achievement {
                id: Uuid::new_v4(),
                achievement_type: AchievementType::PermissionsGranted,
                name: "Trusted Guardian".to_string(),
                description: "Grant all necessary permissions".to_string(),
                experience_reward: 75,
                token_reward: 150,
                badge: "🔐".to_string(),
                unlocked_at: None,
                is_unlocked: false,
            },
            Achievement {
                id: Uuid::new_v4(),
                achievement_type: AchievementType::TrustedNetworkCreated,
                name: "Safety Network".to_string(),
                description: "Create your trusted network".to_string(),
                experience_reward: 100,
                token_reward: 200,
                badge: "👥".to_string(),
                unlocked_at: None,
                is_unlocked: false,
            },
            
            // Learning Achievements
            Achievement {
                id: Uuid::new_v4(),
                achievement_type: AchievementType::CPRCertified,
                name: "CPR Hero".to_string(),
                description: "Complete CPR training module".to_string(),
                experience_reward: 200,
                token_reward: 500,
                badge: "💓".to_string(),
                unlocked_at: None,
                is_unlocked: false,
            },
            Achievement {
                id: Uuid::new_v4(),
                achievement_type: AchievementType::FirstAidTrained,
                name: "First Aid Expert".to_string(),
                description: "Complete first aid training".to_string(),
                experience_reward: 150,
                token_reward: 300,
                badge: "🩹".to_string(),
                unlocked_at: None,
                is_unlocked: false,
            },
            Achievement {
                id: Uuid::new_v4(),
                achievement_type: AchievementType::AEDTrained,
                name: "AED Master".to_string(),
                description: "Complete AED training module".to_string(),
                experience_reward: 175,
                token_reward: 400,
                badge: "⚡".to_string(),
                unlocked_at: None,
                is_unlocked: false,
            },
            
            // Intervention Achievements
            Achievement {
                id: Uuid::new_v4(),
                achievement_type: AchievementType::FirstIntervention,
                name: "First Response".to_string(),
                description: "Complete your first emergency intervention".to_string(),
                experience_reward: 300,
                token_reward: 750,
                badge: "🚨".to_string(),
                unlocked_at: None,
                is_unlocked: false,
            },
            Achievement {
                id: Uuid::new_v4(),
                achievement_type: AchievementType::SuccessfulRescue,
                name: "Life Saver".to_string(),
                description: "Successfully save a life".to_string(),
                experience_reward: 1000,
                token_reward: 2500,
                badge: "💎".to_string(),
                unlocked_at: None,
                is_unlocked: false,
            },
            
            // Network Growth Achievements
            Achievement {
                id: Uuid::new_v4(),
                achievement_type: AchievementType::InvitedFriends,
                name: "Community Builder".to_string(),
                description: "Invite 5 friends to SOS Hero".to_string(),
                experience_reward: 250,
                token_reward: 600,
                badge: "🌱".to_string(),
                unlocked_at: None,
                is_unlocked: false,
            },
            Achievement {
                id: Uuid::new_v4(),
                achievement_type: AchievementType::SafetyAmbassador,
                name: "Safety Ambassador".to_string(),
                description: "Help 10 people join the safety network".to_string(),
                experience_reward: 500,
                token_reward: 1200,
                badge: "🏆".to_string(),
                unlocked_at: None,
                is_unlocked: false,
            },
        ]
    }
    
    /// Initialize all rewards
    fn initialize_rewards() -> Vec<Reward> {
        vec![
            Reward {
                id: Uuid::new_v4(),
                name: "Setup Completion".to_string(),
                description: "Complete initial app setup".to_string(),
                experience_points: 100,
                bonk_tokens: 200,
                skr_tokens: 50,
                hero_badges: vec!["🆕".to_string()],
                unlocked_at: None,
            },
            Reward {
                id: Uuid::new_v4(),
                name: "CPR Certification".to_string(),
                description: "Complete CPR training".to_string(),
                experience_points: 200,
                bonk_tokens: 500,
                skr_tokens: 100,
                hero_badges: vec!["💓".to_string(), "📚".to_string()],
                unlocked_at: None,
            },
            Reward {
                id: Uuid::new_v4(),
                name: "First Intervention".to_string(),
                description: "Complete your first emergency response".to_string(),
                experience_points: 300,
                bonk_tokens: 750,
                skr_tokens: 150,
                hero_badges: vec!["🚨".to_string(), "💪".to_string()],
                unlocked_at: None,
            },
            Reward {
                id: Uuid::new_v4(),
                name: "Network Growth".to_string(),
                description: "Expand your safety network".to_string(),
                experience_points: 150,
                bonk_tokens: 300,
                skr_tokens: 75,
                hero_badges: vec!["👥".to_string(), "🌱".to_string()],
                unlocked_at: None,
            },
        ]
    }
    
    /// Create or get hero profile
    pub async fn get_or_create_profile(&self, user_id: Uuid) -> Result<HeroProfile, AppError> {
        let mut profiles = self.hero_profiles.write().await;
        
        if let Some(profile) = profiles.get(&user_id) {
            Ok(profile.clone())
        } else {
            let new_profile = HeroProfile {
                user_id,
                hero_level: HeroLevel::Novice,
                experience_points: 0,
                total_interventions: 0,
                successful_rescues: 0,
                learning_modules_completed: 0,
                network_size: 0,
                achievements: Vec::new(),
                rewards: Vec::new(),
                created_at: Utc::now(),
                last_active: Utc::now(),
            };
            
            profiles.insert(user_id, new_profile.clone());
            Ok(new_profile)
        }
    }
    
    /// Award experience points
    pub async fn award_experience(&self, user_id: Uuid, points: u32) -> Result<HeroLevel, AppError> {
        let mut profiles = self.hero_profiles.write().await;
        
        if let Some(profile) = profiles.get_mut(&user_id) {
            profile.experience_points += points;
            profile.last_active = Utc::now();
            
            let new_level = HeroLevel::from_experience(profile.experience_points);
            let level_up = new_level > profile.hero_level;
            
            if level_up {
                info!("🎉 Hero level up! {} -> {}", profile.hero_level.title(), new_level.title());
            }
            
            profile.hero_level = new_level.clone();
            Ok(new_level)
        } else {
            Err(AppError::Gamification("User profile not found".to_string()))
        }
    }
    
    /// Complete learning module
    pub async fn complete_learning_module(&self, user_id: Uuid, module_type: &str) -> Result<Reward, AppError> {
        let mut profiles = self.hero_profiles.write().await;
        
        if let Some(profile) = profiles.get_mut(&user_id) {
            profile.learning_modules_completed += 1;
            
            // Award experience and tokens
            let experience_reward = 200;
            let bonk_reward = 500;
            let skr_reward = 100;
            
            profile.experience_points += experience_reward;
            profile.last_active = Utc::now();
            
            // Create reward
            let reward = Reward {
                id: Uuid::new_v4(),
                name: format!("{} Master", module_type),
                description: format!("Completed {} training", module_type),
                experience_points: experience_reward,
                bonk_tokens: bonk_reward,
                skr_tokens: skr_reward,
                hero_badges: vec!["📚".to_string(), "🎓".to_string()],
                unlocked_at: Some(Utc::now()),
            };
            
            profile.rewards.push(reward.clone());
            
            info!("🎓 Learning module completed: {} (+{} XP, +{} BONK, +{} SKR)", 
                  module_type, experience_reward, bonk_reward, skr_reward);
            
            Ok(reward)
        } else {
            Err(AppError::Gamification("User profile not found".to_string()))
        }
    }
    
    /// Record emergency intervention
    pub async fn record_intervention(&self, user_id: Uuid, emergency_type: EmergencyType, was_successful: bool) -> Result<Reward, AppError> {
        let mut profiles = self.hero_profiles.write().await;
        
        if let Some(profile) = profiles.get_mut(&user_id) {
            profile.total_interventions += 1;
            
            if was_successful {
                profile.successful_rescues += 1;
            }
            
            // Award experience and tokens
            let experience_reward = if was_successful { 500 } else { 100 };
            let bonk_reward = if was_successful { 1000 } else { 200 };
            let skr_reward = if was_successful { 250 } else { 50 };
            
            profile.experience_points += experience_reward;
            profile.last_active = Utc::now();
            
            // Create reward
            let reward = Reward {
                id: Uuid::new_v4(),
                name: if was_successful { "Life Saver".to_string() } else { "Emergency Responder".to_string() },
                description: format!("Responded to {:?} emergency", emergency_type),
                experience_points: experience_reward,
                bonk_tokens: bonk_reward,
                skr_tokens: skr_reward,
                hero_badges: if was_successful { 
                    vec!["💎".to_string(), "🏆".to_string()] 
                } else { 
                    vec!["🚨".to_string(), "💪".to_string()] 
                },
                unlocked_at: Some(Utc::now()),
            };
            
            profile.rewards.push(reward.clone());
            
            info!("🚨 Emergency intervention recorded: {:?} ({}) (+{} XP, +{} BONK, +{} SKR)", 
                  emergency_type, if was_successful { "SUCCESS" } else { "RESPONDED" }, 
                  experience_reward, bonk_reward, skr_reward);
            
            Ok(reward)
        } else {
            Err(AppError::Gamification("User profile not found".to_string()))
        }
    }
    
    /// Expand trusted network
    pub async fn expand_network(&self, user_id: Uuid, new_contacts: u32) -> Result<Reward, AppError> {
        let mut profiles = self.hero_profiles.write().await;
        
        if let Some(profile) = profiles.get_mut(&user_id) {
            profile.network_size += new_contacts;
            
            // Award experience and tokens
            let experience_reward = new_contacts * 50;
            let bonk_reward = new_contacts * 100;
            let skr_reward = new_contacts * 25;
            
            profile.experience_points += experience_reward;
            profile.last_active = Utc::now();
            
            // Create reward
            let reward = Reward {
                id: Uuid::new_v4(),
                name: "Network Growth".to_string(),
                description: format!("Added {} contacts to safety network", new_contacts),
                experience_points: experience_reward,
                bonk_tokens: bonk_reward,
                skr_tokens: skr_reward,
                hero_badges: vec!["👥".to_string(), "🌱".to_string()],
                unlocked_at: Some(Utc::now()),
            };
            
            profile.rewards.push(reward.clone());
            
            info!("👥 Network expanded: +{} contacts (+{} XP, +{} BONK, +{} SKR)", 
                  new_contacts, experience_reward, bonk_reward, skr_reward);
            
            Ok(reward)
        } else {
            Err(AppError::Gamification("User profile not found".to_string()))
        }
    }
    
    /// Get leaderboard
    pub async fn get_leaderboard(&self) -> Result<Vec<HeroProfile>, AppError> {
        let profiles = self.hero_profiles.read().await;
        let mut leaderboard: Vec<HeroProfile> = profiles.values().cloned().collect();
        
        // Sort by experience points (descending)
        leaderboard.sort_by(|a, b| b.experience_points.cmp(&a.experience_points));
        
        Ok(leaderboard)
    }
    
    /// Get user statistics
    pub async fn get_user_stats(&self, user_id: Uuid) -> Result<HeroProfile, AppError> {
        let profiles = self.hero_profiles.read().await;
        
        profiles.get(&user_id)
            .cloned()
            .ok_or_else(|| AppError::Gamification("User profile not found".to_string()))
    }
} 