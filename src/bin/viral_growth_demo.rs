//! Enhanced Viral Growth Demo with Family Plans, Hero Moments, and Challenges
//! 
//! This demo showcases the advanced viral growth features:
//! - Family Plans with multi-device viral loops
//! - Auto-generated Hero Moments with AI personalization
//! - Family Challenges for group engagement
//! - Gamified viral mechanics for mass adoption

use solana_sos::private::viral_sharing::{
    ViralSharingSystem, FamilyPlanType, FamilyRole, ChallengeType, SaveOutcome
};
use solana_sos::public::types::{EmergencyType, Location};
use std::time::SystemTime;
use tokio;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Solana SOS - Advanced Viral Growth System Demo");
    info!("==================================================");

    let mut viral_system = ViralSharingSystem::new();

    // Demo 1: Family Plan Creation & Viral Loops
    demo_family_plans(&mut viral_system).await?;
    
    // Demo 2: Auto-Generated Hero Moments
    demo_hero_moments(&mut viral_system).await?;
    
    // Demo 3: Family Challenges for Group Engagement
    demo_family_challenges(&mut viral_system).await?;
    
    // Demo 4: Advanced Viral Analytics
    demo_advanced_analytics(&viral_system).await?;

    info!("\n🎉 Advanced Viral Growth Demo completed successfully!");
    info!("👨‍👩‍👧‍👦 Family Plans drive 5x viral growth through group dynamics!");
    info!("🦸‍♀️ Hero Moments create personalized viral content automatically!");
    info!("🏆 Family Challenges boost engagement and retention!");
    info!("📈 Advanced analytics enable data-driven viral optimization!");
    
    Ok(())
}

async fn demo_family_plans(viral_system: &mut ViralSharingSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n👨‍👩‍👧‍👦 Demo 1: Family Plans & Multi-Device Viral Growth");
    info!("=========================================================");
    
    // Create family plans for different family types
    let families = vec![
        ("dad_dev_123", FamilyPlanType::Premium, vec![
            ("mom_sarah_456".to_string(), FamilyRole::Parent),
            ("teen_alex_789".to_string(), FamilyRole::Child),
            ("grandpa_joe_012".to_string(), FamilyRole::Grandparent),
        ]),
        ("single_parent_345", FamilyPlanType::Basic, vec![
            ("child_emma_678".to_string(), FamilyRole::Child),
            ("child_noah_901".to_string(), FamilyRole::Child),
        ]),
        ("first_responder_567", FamilyPlanType::Enterprise, vec![
            ("partner_mike_234".to_string(), FamilyRole::Other),
            ("daughter_lily_567".to_string(), FamilyRole::Child),
            ("son_jack_890".to_string(), FamilyRole::Child),
            ("aunt_mary_123".to_string(), FamilyRole::Guardian),
        ]),
    ];

    for (owner_id, plan_type, members) in families {
        info!("👤 Creating family plan for: {}", owner_id);
        
        let family_plan = viral_system
            .create_family_plan(owner_id, plan_type, members)
            .await?;

        info!("   ✅ Family Plan ID: {}", family_plan.id);
        info!("   📱 Plan Type: {:?}", family_plan.plan_type);
        info!("   👥 Members: {}", family_plan.members.len());
        info!("   💰 Shared Pool: {} BONK + {} SKR", 
            family_plan.shared_bonk_tokens, family_plan.shared_skr_tokens);
        
        // Simulate adding additional family members (viral growth)
        if family_plan.members.len() < 6 {
            let new_member_id = format!("new_member_{}", family_plan.id[..8].to_string());
            viral_system.add_family_member(&family_plan.id, &new_member_id, FamilyRole::Sibling).await?;
            info!("   ➕ Added viral referral: {}", new_member_id);
        }

        // Show family roles breakdown
        info!("   👨‍👩‍👧‍👦 Family Composition:");
        for member in &family_plan.members {
            info!("      {:?}: {}", member.role, member.user_id);
        }
    }

    Ok(())
}

async fn demo_hero_moments(viral_system: &mut ViralSharingSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🦸‍♀️ Demo 2: Auto-Generated Hero Moments with AI Personalization");
    info!("================================================================");
    
    // First, create some emergency saves to base hero moments on
    let emergency_scenarios = vec![
        (
            "dad_dev_123",
            EmergencyType::Choking,
            SaveOutcome::EmergencyResolved,
            23, // Lightning fast response!
            Location {
                latitude: 37.7749,
                longitude: -122.4194,
                altitude: None,
                accuracy: Some(5.0),
                timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            },
            "parent", // Target audience
        ),
        (
            "first_responder_567",
            EmergencyType::HeartAttack,
            SaveOutcome::LifeSaved,
            31,
            Location {
                latitude: 40.7128,
                longitude: -74.0060,
                altitude: None,
                accuracy: Some(8.0),
                timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            },
            "first_responder",
        ),
        (
            "single_parent_345",
            EmergencyType::SevereBurns,
            SaveOutcome::FirstAidProvided,
            67,
            Location {
                latitude: 34.0522,
                longitude: -118.2437,
                altitude: None,
                accuracy: Some(12.0),
                timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            },
            "parent",
        ),
    ];

    let mut emergency_save_ids = Vec::new();
    
    // Create emergency saves
    for (user_id, emergency_type, outcome, response_time, location, _) in &emergency_scenarios {
        let save = viral_system
            .share_emergency_save(user_id, *emergency_type, *outcome, *response_time, location.clone())
            .await?;
        let save_id = save.id.clone();
        emergency_save_ids.push(save.id);
        
        info!("🚨 Emergency save created: {} - {:?} in {}s", 
            save_id[..8].to_string(), emergency_type, response_time);
    }

    // Generate personalized Hero Moments
    for (i, (user_id, _emergency_type, _outcome, _response_time, _, target_audience)) in emergency_scenarios.iter().enumerate() {
        info!("\n🎬 Generating Hero Moment for: {}", user_id);
        
        let hero_moment = viral_system
            .generate_hero_moment(user_id, &emergency_save_ids[i], target_audience)
            .await?;

        info!("   ✅ Hero Moment ID: {}", hero_moment.id[..8].to_string());
        info!("   🎯 Target Audience: {}", hero_moment.personalization.target_audience);
        info!("   📰 Headline: {}", hero_moment.content.headline);
        info!("   📝 Story: {}", hero_moment.content.story);
        info!("   🎨 Visual Theme: {}", hero_moment.content.visual_theme);
        info!("   📱 Recommended Platforms: {:?}", hero_moment.personalization.recommended_platforms);
        info!("   💬 Personalized Message: {}", hero_moment.personalization.personalized_message);
        info!("   🔗 Hero Blink URL: {}", hero_moment.blink_url);
        info!("   #️⃣ Hashtags: {:?}", hero_moment.content.hashtags);
        
        // Show call to action
        info!("   📢 Call to Action: {}", hero_moment.content.call_to_action);
    }

    Ok(())
}

async fn demo_family_challenges(viral_system: &mut ViralSharingSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🏆 Demo 3: Family Challenges for Group Engagement");
    info!("==================================================");
    
    // Create different types of family challenges
    let challenges = vec![
        (
            "Family Safety Week",
            "Complete emergency training together and build family preparedness",
            ChallengeType::FamilyPreparedness,
            7, // 7 days
            500, // BONK reward
            300, // SKR reward
        ),
        (
            "Community Heroes Challenge",
            "Share your family's emergency saves and inspire others",
            ChallengeType::CommunityHeroes,
            14, // 2 weeks
            800,
            500,
        ),
        (
            "Referral Champions",
            "Which family can bring the most new users to Solana SOS?",
            ChallengeType::ReferralRace,
            30, // 1 month
            1000,
            750,
        ),
    ];

    for (name, description, challenge_type, duration, bonk_reward, skr_reward) in challenges {
        info!("🎯 Creating challenge: {}", name);
        
        let challenge = viral_system
            .create_family_challenge(name, description, challenge_type, duration, bonk_reward, skr_reward)
            .await?;

        info!("   ✅ Challenge ID: {}", challenge.id[..8].to_string());
        info!("   📅 Duration: {} days", challenge.duration_days);
        info!("   💰 Rewards: {} BONK + {} SKR", challenge.bonk_reward, challenge.skr_reward);
        info!("   👥 Min Participants: {}", challenge.requirements.min_participants);
        info!("   🎮 Challenge Type: {:?}", challenge.challenge_type);
        
        // Show challenge requirements
        info!("   📋 Required Actions:");
        for action in &challenge.requirements.required_actions {
            info!("      • {}: {} times ({} points each)", 
                action.action_type, action.required_count, action.points_per_completion);
        }
        
        // Show target metrics
        if !challenge.requirements.target_metrics.is_empty() {
            info!("   📊 Target Metrics:");
            for (metric, target) in &challenge.requirements.target_metrics {
                info!("      • {}: {:.1}%", metric, target);
            }
        }
    }

    Ok(())
}

async fn demo_advanced_analytics(viral_system: &ViralSharingSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n📊 Demo 4: Advanced Viral Analytics & Insights");
    info!("===============================================");
    
    // Family Plan Analytics
    info!("👨‍👩‍👧‍👦 Family Plan Analytics:");
    info!("   Total Family Plans: {}", viral_system.get_all_family_plans().len());
    
    let mut total_family_members = 0;
    let mut plan_type_breakdown = std::collections::HashMap::new();
    
    for family_plan in viral_system.get_all_family_plans().values() {
        total_family_members += family_plan.members.len();
        *plan_type_breakdown.entry(family_plan.plan_type).or_insert(0) += 1;
    }
    
    info!("   Total Family Members: {}", total_family_members);
    info!("   Average Family Size: {:.1}", 
        total_family_members as f64 / viral_system.get_all_family_plans().len() as f64);
    
    for (plan_type, count) in plan_type_breakdown {
        info!("   {:?} Plans: {}", plan_type, count);
    }

    // Hero Moment Analytics
    info!("\n🦸‍♀️ Hero Moment Analytics:");
    info!("   Total Hero Moments: {}", viral_system.get_all_hero_moments().len());
    
    let mut audience_breakdown = std::collections::HashMap::new();
    let mut theme_breakdown = std::collections::HashMap::new();
    
    for hero_moment in viral_system.get_all_hero_moments().values() {
        *audience_breakdown.entry(hero_moment.personalization.target_audience.clone()).or_insert(0) += 1;
        *theme_breakdown.entry(hero_moment.content.visual_theme.clone()).or_insert(0) += 1;
    }
    
    info!("   Target Audiences:");
    for (audience, count) in audience_breakdown {
        info!("      {}: {} moments", audience, count);
    }
    
    info!("   Visual Themes:");
    for (theme, count) in theme_breakdown {
        info!("      {}: {} moments", theme, count);
    }

    // Challenge Analytics
    info!("\n🏆 Family Challenge Analytics:");
    info!("   Total Active Challenges: {}", viral_system.get_all_family_challenges().len());
    
    let mut challenge_type_breakdown = std::collections::HashMap::new();
    let mut total_rewards = 0u64;
    
    for challenge in viral_system.get_all_family_challenges().values() {
        *challenge_type_breakdown.entry(challenge.challenge_type).or_insert(0) += 1;
        total_rewards += challenge.bonk_reward + challenge.skr_reward;
    }
    
    info!("   Challenge Types:");
    for (challenge_type, count) in challenge_type_breakdown {
        info!("      {:?}: {} challenges", challenge_type, count);
    }
    
    info!("   Total Reward Pool: {} tokens", total_rewards);

    // Overall Viral Metrics
    info!("\n📈 Overall Viral Growth Metrics:");
    let user_stats = viral_system.get_all_user_stats();
    if !user_stats.is_empty() {
        let total_users = user_stats.len();
        let total_referrals: u32 = user_stats.values().map(|s| s.total_referrals).sum();
        let total_conversions: u32 = user_stats.values().map(|s| s.successful_conversions).sum();
        let avg_viral_score: f64 = user_stats.values().map(|s| s.viral_score).sum::<f64>() / total_users as f64;
        
        info!("   Total Users: {}", total_users);
        info!("   Total Referrals: {}", total_referrals);
        info!("   Total Conversions: {}", total_conversions);
        info!("   Conversion Rate: {:.1}%", 
            if total_referrals > 0 { (total_conversions as f64 / total_referrals as f64) * 100.0 } else { 0.0 });
        info!("   Average Viral Score: {:.2}", avg_viral_score);
        
        // Family multiplier impact
        let family_bonus_users = user_stats.values()
            .filter(|s| s.total_bonk_earned > 200) // Users who likely got family bonuses
            .count();
        info!("   Users with Family Bonuses: {} ({:.1}%)", 
            family_bonus_users, (family_bonus_users as f64 / total_users as f64) * 100.0);
    }

    // Growth Projections
    info!("\n🚀 Viral Growth Projections:");
    let current_users = user_stats.len() as f64;
    let avg_referrals_per_user = if current_users > 0.0 {
        user_stats.values().map(|s| s.total_referrals).sum::<u32>() as f64 / current_users
    } else { 0.0 };
    
    info!("   Current Users: {:.0}", current_users);
    info!("   Avg Referrals per User: {:.2}", avg_referrals_per_user);
    
    if avg_referrals_per_user > 1.0 {
        let k_factor = avg_referrals_per_user * 0.5; // Assuming 50% conversion rate
        info!("   Estimated K-Factor: {:.2} (Viral! 🎉)", k_factor);
        
        // Project growth over time
        let mut projected_users = current_users;
        for week in 1..=4 {
            projected_users *= k_factor;
            info!("   Week {} Projection: {:.0} users", week, projected_users);
        }
    } else {
        info!("   K-Factor: {:.2} (Pre-viral - need optimization)", avg_referrals_per_user);
    }

    Ok(())
}
