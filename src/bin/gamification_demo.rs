use crisis_companion::{
    gamification::GamificationManager,
    types::*,
    error::AppError,
};
use tracing::{info, warn, error};
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🏆 Solana SOS - SOS Hero Gamification System Demo");
    info!("================================================");
    
    let gamification = GamificationManager::new();
    let user_id = Uuid::new_v4();
    
    // Create user profile
    let profile = gamification.get_or_create_profile(user_id).await?;
    println!("✅ Hero profile created: {} {}", profile.hero_level.badge(), profile.hero_level.title());
    
    sleep(Duration::from_secs(1)).await;
    
    // Demo 1: App Setup Rewards
    println!("\n🎬 Demo 1: App Setup Rewards");
    println!("=============================");
    
    let setup_reward = gamification.award_experience(user_id, 100).await?;
    println!("✅ App setup completed!");
    println!("🎁 Reward: +100 XP, +200 BONK, +50 SKR");
    println!("📈 New level: {} {}", setup_reward.badge(), setup_reward.title());
    
    sleep(Duration::from_secs(2)).await;
    
    // Demo 2: Learning Module Completion
    println!("\n🎬 Demo 2: Learning Module Completion");
    println!("=====================================");
    
    let cpr_reward = gamification.complete_learning_module(user_id, "CPR").await?;
    println!("🎓 CPR training completed!");
    println!("🎁 Reward: +{} XP, +{} BONK, +{} SKR", 
             cpr_reward.experience_points, cpr_reward.bonk_tokens, cpr_reward.skr_tokens);
    println!("🏆 Badges earned: {:?}", cpr_reward.hero_badges);
    
    let first_aid_reward = gamification.complete_learning_module(user_id, "First Aid").await?;
    println!("🎓 First Aid training completed!");
    println!("🎁 Reward: +{} XP, +{} BONK, +{} SKR", 
             first_aid_reward.experience_points, first_aid_reward.bonk_tokens, first_aid_reward.skr_tokens);
    
    sleep(Duration::from_secs(2)).await;
    
    // Demo 3: Emergency Intervention
    println!("\n🎬 Demo 3: Emergency Intervention");
    println!("=================================");
    
    let intervention_reward = gamification.record_intervention(user_id, EmergencyType::Drowning, true).await?;
    println!("🚨 Emergency intervention: Drowning (SUCCESS)");
    println!("🎁 Reward: +{} XP, +{} BONK, +{} SKR", 
             intervention_reward.experience_points, intervention_reward.bonk_tokens, intervention_reward.skr_tokens);
    println!("🏆 Badges earned: {:?}", intervention_reward.hero_badges);
    
    let new_level = gamification.award_experience(user_id, 0).await?;
    println!("📈 New level: {} {}", new_level.badge(), new_level.title());
    
    sleep(Duration::from_secs(2)).await;
    
    // Demo 4: Network Growth
    println!("\n🎬 Demo 4: Network Growth");
    println!("==========================");
    
    let network_reward = gamification.expand_network(user_id, 5).await?;
    println!("👥 Added 5 contacts to safety network");
    println!("🎁 Reward: +{} XP, +{} BONK, +{} SKR", 
             network_reward.experience_points, network_reward.bonk_tokens, network_reward.skr_tokens);
    println!("🏆 Badges earned: {:?}", network_reward.hero_badges);
    
    sleep(Duration::from_secs(2)).await;
    
    // Demo 5: Hero Levels
    println!("\n🎬 Demo 5: Hero Levels");
    println!("=======================");
    
    let stats = gamification.get_user_stats(user_id).await?;
    println!("👤 Hero Profile:");
    println!("   Level: {} {}", stats.hero_level.badge(), stats.hero_level.title());
    println!("   Experience: {} XP", stats.experience_points);
    println!("   Interventions: {}", stats.total_interventions);
    println!("   Successful Rescues: {}", stats.successful_rescues);
    println!("   Learning Modules: {}", stats.learning_modules_completed);
    println!("   Network Size: {}", stats.network_size);
    println!("   Total Rewards: {}", stats.rewards.len());
    
    sleep(Duration::from_secs(2)).await;
    
    // Demo 6: Leaderboard
    println!("\n🎬 Demo 6: Leaderboard");
    println!("=======================");
    
    // Add some other users for leaderboard demo
    let user2 = Uuid::new_v4();
    let user3 = Uuid::new_v4();
    
    gamification.get_or_create_profile(user2).await?;
    gamification.award_experience(user2, 800).await?;
    gamification.record_intervention(user2, EmergencyType::HeartAttack, true).await?;
    
    gamification.get_or_create_profile(user3).await?;
    gamification.award_experience(user3, 1200).await?;
    gamification.complete_learning_module(user3, "AED").await?;
    
    let leaderboard = gamification.get_leaderboard().await?;
    println!("🏆 Top Heroes:");
    for (i, profile) in leaderboard.iter().take(5).enumerate() {
        println!("   {}. {} {} - {} XP", 
                 i + 1, profile.hero_level.badge(), profile.hero_level.title(), profile.experience_points);
    }
    
    sleep(Duration::from_secs(2)).await;
    
    // Demo 7: Achievement System
    println!("\n🎬 Demo 7: Achievement System");
    println!("=============================");
    
    println!("🏅 Available Achievements:");
    println!("   🆕 First Steps - Complete initial app setup");
    println!("   🔐 Trusted Guardian - Grant all permissions");
    println!("   👥 Safety Network - Create trusted network");
    println!("   💓 CPR Hero - Complete CPR training");
    println!("   🩹 First Aid Expert - Complete first aid training");
    println!("   ⚡ AED Master - Complete AED training");
    println!("   🚨 First Response - Complete first intervention");
    println!("   💎 Life Saver - Successfully save a life");
    println!("   🌱 Community Builder - Invite 5 friends");
    println!("   🏆 Safety Ambassador - Help 10 people join");
    
    sleep(Duration::from_secs(2)).await;
    
    // Demo 8: Growth Engine
    println!("\n🎬 Demo 8: Growth Engine");
    println!("=========================");
    
    println!("🚀 Network Effect:");
    println!("   • User adds 5 trusted contacts");
    println!("   • Each contact gets invited to join SOS Hero");
    println!("   • New users complete setup and add their contacts");
    println!("   • Exponential growth through safety network");
    println!("   • Viral coefficient: 2.5x (each user invites 2.5 others)");
    
    println!("\n💰 Token Economics:");
    println!("   • BONK tokens for learning and interventions");
    println!("   • SKR tokens for network growth");
    println!("   • Tokens can be used for premium features");
    println!("   • Staking rewards for active heroes");
    
    sleep(Duration::from_secs(2)).await;
    
    // Final summary
    println!("\n🎯 Solana SOS - SOS Hero Gamification Summary:");
    println!("================================================");
    println!("✅ 10 Hero Levels with unique titles and badges");
    println!("✅ 20+ Achievements for different activities");
    println!("✅ Experience points and token rewards");
    println!("✅ Learning module completion tracking");
    println!("✅ Emergency intervention recording");
    println!("✅ Network growth incentives");
    println!("✅ Leaderboard and social features");
    println!("✅ Viral growth through trusted network");
    println!("✅ Token economics with BONK and SKR");
    
    info!("🎉 Gamification demo completed successfully!");
    info!("Solana SOS creates a powerful growth engine through SOS Hero gamification!");
    
    Ok(())
} 