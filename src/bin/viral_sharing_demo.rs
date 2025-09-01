//! Viral Sharing System Demo
//! 
//! This demo showcases the Solana Blinks integration for viral growth:
//! - Referral campaigns with token rewards
//! - Emergency save sharing (privacy-preserving)
//! - Achievement sharing for social proof
//! - Viral leaderboards and statistics

use solana_sos::private::viral_sharing::{
    ViralSharingSystem, Achievement, AchievementCategory, AchievementRarity, SaveOutcome
};
use solana_sos::public::types::{EmergencyType, Location};
use std::time::SystemTime;
use tokio;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Solana SOS - Viral Sharing System Demo");
    info!("==========================================");

    let mut viral_system = ViralSharingSystem::new();

    // Demo 1: Create App Download Referral Campaign
    demo_referral_campaigns(&mut viral_system).await?;
    
    // Demo 2: Share Emergency Saves
    demo_emergency_save_sharing(&mut viral_system).await?;
    
    // Demo 3: Share Achievements
    demo_achievement_sharing(&mut viral_system).await?;
    
    // Demo 4: Process Viral Conversions
    demo_viral_conversions(&mut viral_system).await?;
    
    // Demo 5: Viral Analytics and Leaderboards
    demo_viral_analytics(&viral_system).await?;

    info!("\n🎉 Viral Sharing Demo completed successfully!");
    info!("🔗 Solana Blinks enable seamless viral growth with token incentives!");
    info!("🏆 Privacy-preserving sharing drives mass adoption!");
    
    Ok(())
}

async fn demo_referral_campaigns(viral_system: &mut ViralSharingSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🔗 Demo 1: App Download Referral Campaigns");
    info!("============================================");
    
    // Create referral campaigns for different users
    let users = vec![
        ("alice_123", "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM"),
        ("bob_456", "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU"),
        ("carol_789", "4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi"),
    ];

    for (user_id, wallet) in users {
        info!("👤 Creating referral campaign for user: {}", user_id);
        
        let campaign = viral_system
            .create_app_download_campaign(user_id, wallet)
            .await?;

        info!("   ✅ Campaign ID: {}", campaign.id);
        info!("   🔗 Blink URL: {}", campaign.blink_url);
        info!("   💰 Rewards: {} BONK + {} SKR per conversion", 
            campaign.bonk_reward, campaign.skr_reward);
        info!("   📝 Title: {}", campaign.title);
        info!("   📄 Description: {}", campaign.description);
        
        // Simulate some interactions
        viral_system.track_blink_interaction(&campaign.id, "view").await?;
        viral_system.track_blink_interaction(&campaign.id, "view").await?;
        viral_system.track_blink_interaction(&campaign.id, "click").await?;
        
        info!("   📊 Simulated: 2 views, 1 click");
    }

    Ok(())
}

async fn demo_emergency_save_sharing(viral_system: &mut ViralSharingSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🆘 Demo 2: Emergency Save Sharing (Privacy-Preserving)");
    info!("======================================================");
    
    // Simulate different emergency saves
    let emergency_scenarios = vec![
        (
            "alice_123",
            EmergencyType::HeartAttack,
            SaveOutcome::LifeSaved,
            35, // 35 seconds response time
            Location {
                latitude: 37.7749,
                longitude: -122.4194,
                altitude: None,
                accuracy: Some(10.0),
                timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            },
        ),
        (
            "bob_456",
            EmergencyType::Choking,
            SaveOutcome::EmergencyResolved,
            28, // 28 seconds - lightning fast!
            Location {
                latitude: 40.7128,
                longitude: -74.0060,
                altitude: None,
                accuracy: Some(5.0),
                timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            },
        ),
        (
            "carol_789",
            EmergencyType::SevereBurns,
            SaveOutcome::FirstAidProvided,
            95, // 95 seconds
            Location {
                latitude: 34.0522,
                longitude: -118.2437,
                altitude: None,
                accuracy: Some(15.0),
                timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            },
        ),
    ];

    for (user_id, emergency_type, outcome, response_time, location) in emergency_scenarios {
        info!("🚨 Sharing emergency save by user: {}", user_id);
        
        let save = viral_system
            .share_emergency_save(user_id, emergency_type, outcome, response_time, location)
            .await?;

        info!("   📋 Emergency: {:?}", save.emergency_type);
        info!("   ✅ Outcome: {:?}", save.outcome);
        info!("   ⏱️  Response Time: {} seconds", save.response_time_seconds);
        info!("   🔗 Blink URL: {}", save.blink_url);
        info!("   📖 Story: {}", save.shareable_context.story_title);
        info!("   📝 Description: {}", save.shareable_context.description);
        info!("   💥 Impact: {}", save.shareable_context.impact);
        info!("   🎯 Call to Action: {}", save.shareable_context.call_to_action);
        info!("   🔒 Privacy: Location hashed to {}", save.location_hash);
    }

    Ok(())
}

async fn demo_achievement_sharing(viral_system: &mut ViralSharingSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🏆 Demo 3: Achievement Sharing");
    info!("===============================");
    
    // Create sample achievements
    let achievements = vec![
        Achievement {
            id: "first_responder".to_string(),
            name: "First Responder Hero".to_string(),
            description: "Successfully responded to your first emergency".to_string(),
            category: AchievementCategory::Emergency,
            rarity: AchievementRarity::Uncommon,
            icon_url: "https://solanasos.com/achievements/first_responder.png".to_string(),
            xp_reward: 200,
            token_reward: 100,
        },
        Achievement {
            id: "lightning_response".to_string(),
            name: "Lightning Response".to_string(),
            description: "Responded to an emergency in under 30 seconds".to_string(),
            category: AchievementCategory::Emergency,
            rarity: AchievementRarity::Rare,
            icon_url: "https://solanasos.com/achievements/lightning.png".to_string(),
            xp_reward: 300,
            token_reward: 200,
        },
        Achievement {
            id: "life_saver".to_string(),
            name: "Life Saver".to_string(),
            description: "Your actions directly saved someone's life".to_string(),
            category: AchievementCategory::Emergency,
            rarity: AchievementRarity::Epic,
            icon_url: "https://solanasos.com/achievements/life_saver.png".to_string(),
            xp_reward: 500,
            token_reward: 500,
        },
    ];

    let users = ["alice_123", "bob_456", "carol_789"];
    
    for (i, achievement) in achievements.iter().enumerate() {
        let user_id = users[i % users.len()];
        
        info!("🎖️ User {} unlocked achievement: {}", user_id, achievement.name);
        
        let share = viral_system
            .share_achievement(user_id, achievement.clone())
            .await?;

        info!("   🏆 Achievement: {}", share.achievement.name);
        info!("   📝 Description: {}", share.achievement.description);
        info!("   🎨 Category: {:?}", share.achievement.category);
        info!("   ✨ Rarity: {:?}", share.achievement.rarity);
        info!("   🎯 XP Reward: {}", share.achievement.xp_reward);
        info!("   💰 Token Reward: {}", share.achievement.token_reward);
        info!("   🔗 Blink URL: {}", share.blink_url);
    }

    Ok(())
}

async fn demo_viral_conversions(viral_system: &mut ViralSharingSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n📈 Demo 4: Viral Conversion Processing");
    info!("======================================");
    
    // Get campaign IDs for conversion simulation
    let campaign_ids: Vec<String> = viral_system
        .get_all_campaigns()
        .keys()
        .cloned()
        .collect();

    if campaign_ids.is_empty() {
        warn!("No campaigns found for conversion demo");
        return Ok(());
    }

    // Simulate conversions for each campaign
    for (i, campaign_id) in campaign_ids.iter().enumerate() {
        let new_user_id = format!("new_user_{}", i + 1);
        
        info!("💫 Processing referral conversion for campaign: {}", campaign_id);
        info!("   👤 New user: {}", new_user_id);
        
        viral_system
            .process_referral_conversion(campaign_id, &new_user_id)
            .await?;

        // Get campaign metrics
        if let Some(metrics) = viral_system.get_campaign_metrics(campaign_id) {
            info!("   📊 Campaign Metrics:");
            info!("      Views: {}", metrics.views);
            info!("      Clicks: {}", metrics.clicks);
            info!("      Conversions: {}", metrics.conversions);
            info!("      Conversion Rate: {:.1}%", metrics.conversion_rate);
            info!("      Click-through Rate: {:.1}%", metrics.click_through_rate);
            info!("      Total Rewards: {} tokens", metrics.total_rewards_distributed);
        }
    }

    Ok(())
}

async fn demo_viral_analytics(viral_system: &ViralSharingSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n📊 Demo 5: Viral Analytics & Leaderboards");
    info!("==========================================");
    
    // Display user statistics
    info!("👥 User Viral Statistics:");
    for (user_id, stats) in viral_system.get_all_user_stats() {
        info!("   🧑‍💼 User: {}", user_id);
        info!("      Total Referrals: {}", stats.total_referrals);
        info!("      Successful Conversions: {}", stats.successful_conversions);
        info!("      BONK Earned: {}", stats.total_bonk_earned);
        info!("      SKR Earned: {}", stats.total_skr_earned);
        info!("      Saves Shared: {}", stats.saves_shared);
        info!("      Achievements Shared: {}", stats.achievements_shared);
        info!("      Viral Score: {:.2}", stats.viral_score);
    }

    // Display leaderboard
    info!("\n🏆 Viral Growth Leaderboard (Top 5):");
    let leaderboard = viral_system.get_viral_leaderboard(5);
    
    for (rank, stats) in leaderboard.iter().enumerate() {
        let medal = match rank {
            0 => "🥇",
            1 => "🥈",
            2 => "🥉",
            _ => "🏅",
        };
        
        info!("   {} #{} - {} (Score: {:.2})", 
            medal, rank + 1, stats.user_id, stats.viral_score);
        info!("        Conversions: {}, Tokens: {} BONK + {} SKR", 
            stats.successful_conversions, stats.total_bonk_earned, stats.total_skr_earned);
    }

    // Display emergency saves summary
    info!("\n🆘 Emergency Saves Summary:");
    info!("   Total Saves Shared: {}", viral_system.get_all_emergency_saves().len());
    
    let mut save_types = std::collections::HashMap::new();
    for save in viral_system.get_all_emergency_saves().values() {
        *save_types.entry(save.emergency_type).or_insert(0) += 1;
    }
    
    for (emergency_type, count) in save_types {
        info!("   {:?}: {} saves", emergency_type, count);
    }

    // Display achievement shares summary
    info!("\n🏆 Achievement Shares Summary:");
    info!("   Total Achievements Shared: {}", viral_system.get_all_achievement_shares().len());
    
    let mut achievement_rarities = std::collections::HashMap::new();
    for share in viral_system.get_all_achievement_shares().values() {
        *achievement_rarities.entry(share.achievement.rarity).or_insert(0) += 1;
    }
    
    for (rarity, count) in achievement_rarities {
        info!("   {:?}: {} achievements", rarity, count);
    }

    Ok(())
}
