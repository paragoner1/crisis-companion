use solana_sos::{
    public::types::EmergencyType,
    error::AppResult,
};
use tracing::info;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🏆 Solana SOS - SOS Hero Gamification System Demo");
    info!("================================================");

    // Simulate comprehensive user actions and gamification
    let _user_id = "user_123";
    
    // App Setup & Configuration Rewards
    info!("🎯 User completes app setup...");
    info!("✅ +50 XP for app setup");
    info!("✅ +25 XP for microphone permission");
    info!("✅ +25 XP for location permission");
    info!("✅ +25 XP for contacts permission");
    info!("✅ +25 XP for emergency contacts setup");
    
    // Learning & Training Rewards
    info!("📚 User completes CPR learning module...");
    info!("✅ +100 XP for CPR certification");
    info!("✅ +50 XP for Heimlich maneuver training");
    info!("✅ +50 XP for AED usage training");
    info!("✅ +50 XP for first aid basics");
    info!("✅ +25 XP for emergency response protocols");
    
    // Feature Usage Rewards
    info!("🔧 User maximizes app features...");
    info!("✅ +25 XP for voice recognition calibration");
    info!("✅ +25 XP for noise filtering setup");
    info!("✅ +25 XP for adaptive training enabled");
    info!("✅ +25 XP for hybrid mode configuration");
    info!("✅ +25 XP for offline database setup");
    
    // Safety Features Rewards
    info!("🛡️ User configures safety features...");
    info!("✅ +50 XP for silent SOS setup");
    info!("✅ +50 XP for crash detection enabled");
    info!("✅ +50 XP for trusted network configuration");
    info!("✅ +25 XP for location sharing preferences");
    info!("✅ +25 XP for notification preferences");
    
    // Emergency Response Rewards
    info!("🚨 User performs emergency intervention...");
    info!("✅ +200 XP for successful intervention");
    info!("✅ +100 XP for quick response time (<30s)");
    info!("✅ +50 XP for proper emergency type detection");
    info!("✅ +50 XP for context-aware guidance usage");
    info!("✅ +25 XP for location sharing during emergency");
    
    // Network & Social Rewards
    info!("👥 User expands trusted network...");
    info!("✅ +75 XP for adding 3 trusted contacts");
    info!("✅ +25 XP for each additional contact (up to 10)");
    info!("✅ +50 XP for network verification");
    info!("✅ +25 XP for emergency contact testing");
    
    // Achievement & Progression Rewards
    info!("🏅 User unlocks achievements...");
    info!("✅ +150 XP for 'First Responder' achievement");
    info!("✅ +200 XP for 'Safety Guardian' achievement");
    info!("✅ +300 XP for 'Emergency Expert' achievement");
    info!("✅ +100 XP for 'Network Builder' achievement");
    info!("✅ +150 XP for 'Feature Master' achievement");
    
    // Token Rewards (BONK/SKR)
    info!("🪙 Token rewards earned...");
    info!("✅ +100 BONK for app setup completion");
    info!("✅ +250 BONK for learning module completion");
    info!("✅ +500 BONK for successful emergency intervention");
    info!("✅ +150 BONK for trusted network expansion");
    info!("✅ +200 SKR for achievement milestones");
    info!("✅ +100 SKR for feature mastery");
    info!("✅ +300 SKR for community contribution");

    sleep(Duration::from_secs(2)).await;

    info!("📊 User Statistics:");
    info!("   Level: Expert (Level 8)");
    info!("   XP: 2,475/3,000");
    info!("   Achievements: 8/20");
    info!("   Interventions: 3");
    info!("   Trusted Contacts: 7");
    info!("   Features Mastered: 12/15");
    info!("   BONK Tokens: 1,000");
    info!("   SKR Tokens: 600");

    sleep(Duration::from_secs(2)).await;

    info!("🏆 Leaderboard:");
    info!("   1. user_123 - Level Expert - 2,475 XP - 1,000 BONK");
    info!("   2. user_456 - Level Responder - 1,200 XP - 500 BONK");
    info!("   3. user_789 - Level Novice - 800 XP - 200 BONK");

    sleep(Duration::from_secs(2)).await;

    // Feature Proficiency Rewards
    info!("🎯 Feature Proficiency Rewards:");
    info!("   • Voice Recognition: 95% accuracy (+50 XP)");
    info!("   • Context Analysis: 90% accuracy (+50 XP)");
    info!("   • Emergency Detection: 98% accuracy (+75 XP)");
    info!("   • Location Sharing: 100% reliability (+25 XP)");
    info!("   • Offline Functionality: 100% uptime (+50 XP)");
    info!("   • Hybrid Mode: Seamless switching (+50 XP)");

    sleep(Duration::from_secs(2)).await;

    println!("\n🎯 Solana SOS - SOS Hero Gamification Summary:");
    println!("================================================");
    println!("✅ 10 Hero Levels: Novice → Legend");
    println!("✅ XP Rewards: Setup, Learning, Interventions, Features");
    println!("✅ BONK/SKR Token Integration: 1,000 BONK, 600 SKR earned");
    println!("✅ 20+ Achievements to Unlock");
    println!("✅ Feature Proficiency Tracking");
    println!("✅ Viral Growth: Trusted Network Drives Adoption");
    println!("✅ Community Leaderboards");
    println!("✅ Hero Profile Progression");
    println!("✅ Maximum Feature Usage Rewards");

    info!("🎉 Gamification demo completed successfully!");
    info!("Solana SOS creates a powerful growth engine through comprehensive SOS Hero gamification!");

    Ok(())
} 