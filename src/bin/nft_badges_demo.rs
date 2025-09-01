//! NFT Hero Badges System Demo
//! 
//! This demo showcases the unique Solana-native viral growth mechanism through
//! collectible NFT badges awarded for emergency saves:
//! - Mintable NFT Hero Badges for verified emergency saves
//! - Collectible badge categories with rarity systems
//! - Social sharing and viral mechanics through NFT showcases
//! - Trading marketplace with price analytics
//! - Seasonal challenges and community verification

use solana_sos::private::nft_hero_badges::{
    NFTHeroBadgeSystem, EmergencySaveRecord, SaveOutcome, WitnessVerification,
    BadgeRarity, ListingType
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

    info!("🏆 Solana SOS - NFT Hero Badges System Demo");
    info!("============================================");

    let mut nft_system = NFTHeroBadgeSystem::new();

    // Demo 1: Mint hero badges for emergency saves
    demo_hero_badge_minting(&mut nft_system).await?;
    
    // Demo 2: Social sharing and viral mechanics
    demo_social_sharing_viral(&mut nft_system).await?;
    
    // Demo 3: Badge marketplace and trading
    demo_marketplace_trading(&mut nft_system).await?;
    
    // Demo 4: Collection progress and achievements
    demo_collection_progress(&nft_system).await?;
    
    // Demo 5: Seasonal challenges
    demo_seasonal_challenges(&nft_system).await?;
    
    // Demo 6: Viral analytics and insights
    demo_viral_analytics(&nft_system).await?;

    info!("\n🎉 NFT Hero Badges System Demo completed successfully!");
    info!("🏆 Solana-native collectible badges create powerful viral loops!");
    info!("🌟 Community verification and trading drive engagement!");
    info!("📊 Seasonal challenges and rarity systems boost retention!");
    info!("🚀 Blockchain-verified proof-of-heroism builds trust!");
    
    Ok(())
}

async fn demo_hero_badge_minting(system: &mut NFTHeroBadgeSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🏆 Demo 1: Minting Hero Badges for Emergency Saves");
    info!("==================================================");
    
    // Create realistic emergency save scenarios
    let emergency_saves = vec![
        // Life-saving CPR rescue
        EmergencySaveRecord {
            incident_id: "incident_001".to_string(),
            emergency_type: EmergencyType::HeartAttack,
            location: Location {
                latitude: 40.7128,
                longitude: -74.0060,
                altitude: None,
                accuracy: Some(10.0),
                timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            },
            save_timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            people_saved: 1,
            response_time_seconds: 180, // 3 minutes
            outcome: SaveOutcome::LifeSaved,
            witnesses: vec![
                WitnessVerification {
                    witness_id: "witness_001".to_string(),
                    statement: "Saw the hero perform CPR and save a jogger's life".to_string(),
                    verified_at: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
                    credibility_score: 9.5,
                },
                WitnessVerification {
                    witness_id: "witness_002".to_string(),
                    statement: "Called 911 while hero administered first aid".to_string(),
                    verified_at: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
                    credibility_score: 8.8,
                },
            ],
            official_verification: None,
        },
        
        // Fire rescue heroics
        EmergencySaveRecord {
            incident_id: "incident_002".to_string(),
            emergency_type: EmergencyType::SevereBurns,
            location: Location {
                latitude: 34.0522,
                longitude: -118.2437,
                altitude: None,
                accuracy: Some(15.0),
                timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            },
            save_timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            people_saved: 3,
            response_time_seconds: 120, // 2 minutes
            outcome: SaveOutcome::LifeSaved,
            witnesses: vec![
                WitnessVerification {
                    witness_id: "witness_003".to_string(),
                    statement: "Hero rescued family from burning building".to_string(),
                    verified_at: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
                    credibility_score: 9.8,
                },
            ],
            official_verification: None,
        },

        // Good Samaritan car accident
        EmergencySaveRecord {
            incident_id: "incident_003".to_string(),
            emergency_type: EmergencyType::Trauma,
            location: Location {
                latitude: 41.8781,
                longitude: -87.6298,
                altitude: None,
                accuracy: Some(20.0),
                timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            },
            save_timestamp: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            people_saved: 2,
            response_time_seconds: 240, // 4 minutes
            outcome: SaveOutcome::InjuryPrevented,
            witnesses: vec![
                WitnessVerification {
                    witness_id: "witness_004".to_string(),
                    statement: "Hero pulled victims from overturned car".to_string(),
                    verified_at: SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
                    credibility_score: 9.2,
                },
            ],
            official_verification: None,
        },
    ];

    let users = vec!["hero_sarah", "hero_mike", "hero_alex"];

    for (i, save) in emergency_saves.into_iter().enumerate() {
        let user_id = users[i];
        let emergency_type = save.emergency_type;
        let people_saved = save.people_saved;
        let response_time = save.response_time_seconds;
        
        info!("\n👤 User: {}", user_id);
        info!("   🚨 Emergency: {:?}", emergency_type);
        info!("   👥 People Saved: {}", people_saved);
        info!("   ⏱️  Response Time: {}s", response_time);
        
        // Mint hero badge
        let token_id = system.mint_hero_badge(user_id, save, "spring_lifesaver").await?;
        
        if let Some(badge) = system.hero_badges.get(&token_id) {
            info!("   🏆 Badge Minted: {}", badge.metadata.name);
            info!("   🌟 Rarity: {:?}", badge.metadata.rarity);
            info!("   🎨 Shape: {:?}", badge.metadata.visual_attributes.shape);
            info!("   💎 Token ID: {}", token_id[..16].to_string());
            info!("   ✅ Verification: {:?}", badge.verification);
        }
    }

    info!("\n📊 Minting Summary:");
    info!("   🏆 Total Badges Minted: {}", system.hero_badges.len());
    info!("   👥 Heroes Created: {}", users.len());
    info!("   🌟 Rarity Distribution:");
    
    let mut rarity_counts = std::collections::HashMap::new();
    for badge in system.hero_badges.values() {
        *rarity_counts.entry(badge.metadata.rarity).or_insert(0) += 1;
    }
    
    for (rarity, count) in rarity_counts {
        info!("      {:?}: {} badges", rarity, count);
    }

    Ok(())
}

async fn demo_social_sharing_viral(system: &mut NFTHeroBadgeSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🌟 Demo 2: Social Sharing and Viral Mechanics");
    info!("==============================================");
    
    let platforms = vec!["Instagram", "Twitter", "TikTok", "Facebook", "LinkedIn"];
    let token_ids: Vec<String> = system.hero_badges.keys().cloned().collect();
    
    for (i, token_id) in token_ids.iter().enumerate() {
        let platform = platforms[i % platforms.len()];
        
        if let Some(badge) = system.hero_badges.get(token_id) {
            info!("\n📱 Sharing Badge: {}", badge.metadata.name);
            info!("   👤 Owner: {}", badge.owner_id);
            info!("   🌐 Platform: {}", platform);
            
            let share_url = system.share_badge(token_id, platform).await?;
            
            // Simulate viral engagement
            let (likes, comments, views) = {
                if let Some(updated_badge) = system.hero_badges.get_mut(token_id) {
                    updated_badge.social_stats.likes += 25 + (i as u32 * 15); // Simulate varying popularity
                    updated_badge.social_stats.comments += 5 + (i as u32 * 3);
                    updated_badge.social_stats.views += 150 + (i as u32 * 100);
                    
                    (updated_badge.social_stats.likes, updated_badge.social_stats.comments, updated_badge.social_stats.views)
                } else {
                    (0, 0, 0)
                }
            };
            
            // Calculate viral score separately
            let viral_score = {
                let badge = system.hero_badges.get(token_id).unwrap();
                let social_stats = badge.social_stats.clone();
                system.calculate_viral_score(&social_stats)
            };
            
            // Update viral score
            if let Some(badge) = system.hero_badges.get_mut(token_id) {
                badge.social_stats.viral_score = viral_score;
            }
            
            info!("   📊 Engagement:");
            info!("      👍 Likes: {}", likes);
            info!("      💬 Comments: {}", comments);
            info!("      👀 Views: {}", views);
            info!("      🚀 Viral Score: {:.1}", viral_score);
            info!("   🔗 Share URL: {}", share_url);
        }
    }

    info!("\n🌟 Viral Growth Analysis:");
    let total_shares = system.viral_analytics.total_shares;
    let viral_coefficient = if total_shares > 0 {
        system.hero_badges.values()
            .map(|b| b.social_stats.viral_score)
            .sum::<f64>() / total_shares as f64
    } else {
        0.0
    };
    
    info!("   📈 Total Shares: {}", total_shares);
    info!("   🔄 Viral Coefficient: {:.2}", viral_coefficient);
    info!("   📱 Platform Distribution:");
    
    for (platform, count) in &system.viral_analytics.sharing_platforms {
        info!("      {}: {} shares", platform, count);
    }

    Ok(())
}

async fn demo_marketplace_trading(system: &mut NFTHeroBadgeSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n💰 Demo 3: Badge Marketplace and Trading");
    info!("========================================");
    
    let token_ids: Vec<String> = system.hero_badges.keys().cloned().collect();
    
    // List badges for sale
    info!("\n📋 Listing Badges for Sale:");
    for (_i, token_id) in token_ids.iter().enumerate().take(2) {
        let (price, owner_id, name, rarity) = {
            let badge = system.hero_badges.get(token_id).unwrap();
            let price = match badge.metadata.rarity {
                BadgeRarity::Legendary => 5000,
                BadgeRarity::Epic => 2000,
                BadgeRarity::Rare => 1000,
                BadgeRarity::Uncommon => 500,
                BadgeRarity::Common => 200,
                BadgeRarity::Mythic => 10000,
            };
            (price, badge.owner_id.clone(), badge.metadata.name.clone(), badge.metadata.rarity)
        };
        
        let listing_id = system.list_badge_for_sale(
            token_id,
            &owner_id,
            price,
            ListingType::FixedPrice,
        ).await?;
        
        info!("   🏷️  Listed: {} ({})", name, token_id[..12].to_string());
        info!("      💰 Price: {} BONK", price);
        info!("      🌟 Rarity: {:?}", rarity);
        info!("      📝 Listing ID: {}", listing_id[..12].to_string());
    }

    // Simulate purchases
    info!("\n🛒 Simulating Badge Purchases:");
    let listings: Vec<String> = system.marketplace.listings.keys().cloned().collect();
    
    for (i, listing_id) in listings.iter().enumerate().take(1) {
        let buyer_id = format!("collector_{}", i + 1);
        
        if let Some(listing) = system.marketplace.listings.get(listing_id) {
            info!("   👤 Buyer: {}", buyer_id);
            info!("   🏆 Badge: {}", listing.badge_token_id[..12].to_string());
            info!("   💰 Price: {} BONK", listing.price);
            
            let trade_record = system.purchase_badge(listing_id, &buyer_id).await?;
            
            info!("   ✅ Trade Completed!");
            info!("      🔄 From: {} → To: {}", trade_record.from_user, trade_record.to_user);
            info!("      📅 Trade ID: {}", trade_record.trade_id[..12].to_string());
        }
    }

    // Show marketplace analytics
    info!("\n📊 Marketplace Analytics:");
    let active_listings = system.marketplace.listings.values()
        .filter(|l| l.status == solana_sos::private::nft_hero_badges::ListingStatus::Active)
        .count();
    
    let total_trades = system.marketplace.recent_trades.len();
    let total_volume: u64 = system.marketplace.recent_trades.iter()
        .map(|t| t.price)
        .sum();
    
    info!("   📋 Active Listings: {}", active_listings);
    info!("   🔄 Total Trades: {}", total_trades);
    info!("   💰 Trading Volume: {} BONK", total_volume);
    
    if total_trades > 0 {
        let avg_price = total_volume / total_trades as u64;
        info!("   📈 Average Price: {} BONK", avg_price);
    }

    Ok(())
}

async fn demo_collection_progress(system: &NFTHeroBadgeSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🎯 Demo 4: Collection Progress and Achievements");
    info!("===============================================");
    
    // Show user collections
    for user_id in ["hero_sarah", "hero_mike", "hero_alex", "collector_1"] {
        if let Some(inventory) = system.get_user_collection(user_id) {
            info!("\n👤 User: {}", user_id);
            info!("   🏆 Owned Badges: {}", inventory.owned_badges.len());
            info!("   ⭐ Favorite Badges: {}", inventory.favorite_badges.len());
            
            // Show owned badges details
            for badge_id in &inventory.owned_badges {
                if let Some(badge) = system.hero_badges.get(badge_id) {
                    info!("      • {} ({:?})", badge.metadata.name, badge.metadata.rarity);
                }
            }
            
            // Collection progress
            info!("   📊 Collection Progress:");
            for (collection_id, progress) in &inventory.collection_progress {
                info!("      📚 {}: {:.1}% complete ({}/{})", 
                    collection_id, 
                    progress.completion_percentage,
                    progress.badges_owned.len(),
                    progress.total_badges
                );
            }
            
            // Trading preferences
            info!("   💱 Trading: {} (Min: {} BONK)", 
                if inventory.trading_preferences.trading_enabled { "Enabled" } else { "Disabled" },
                inventory.trading_preferences.minimum_trade_value
            );
        }
    }

    // Show collection statistics
    info!("\n📚 Collection Statistics:");
    for (_collection_id, collection) in &system.collections {
        info!("   🎨 Collection: {}", collection.name);
        info!("      📊 Total Badges: {}", collection.stats.total_badges);
        info!("      🏭 Total Minted: {}", collection.stats.total_minted);
        info!("      ✅ Completion Rate: {:.1}%", collection.stats.completion_rate * 100.0);
        info!("      🎁 Completion Reward: {} BONK", collection.completion_rewards.bonk_reward);
        
        if let Some(popular) = &collection.stats.most_popular_badge {
            info!("      🌟 Most Popular: {}", popular);
        }
    }

    Ok(())
}

async fn demo_seasonal_challenges(system: &NFTHeroBadgeSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🌸 Demo 5: Seasonal Challenges");
    info!("==============================");
    
    for (_challenge_id, challenge) in &system.seasonal_challenges {
        info!("\n🏆 Challenge: {}", challenge.name);
        info!("   📝 Description: {}", challenge.description);
        
        let days_remaining = if challenge.end_date > SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs() {
            (challenge.end_date - SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs()) / (24 * 60 * 60)
        } else {
            0
        };
        
        info!("   ⏰ Days Remaining: {}", days_remaining);
        info!("   🎯 Requirements:");
        info!("      🏆 Min Badges: {}", challenge.requirements.min_badges);
        info!("      📋 Categories: {:?}", challenge.requirements.required_categories);
        info!("      🚨 Emergency Types: {:?}", challenge.requirements.emergency_types);
        
        info!("   🎁 Rewards:");
        info!("      💰 BONK Rewards: {}", challenge.rewards.bonk_rewards);
        info!("      🏅 Exclusive Badges: {:?}", challenge.rewards.exclusive_badges);
        info!("      🏆 Recognition: {:?}", challenge.rewards.recognition);
        
        info!("   📊 Participation:");
        info!("      👥 Participants: {}", challenge.participation.total_participants);
        info!("      ✅ Completion Rate: {:.1}%", challenge.participation.completion_rate * 100.0);
        
        // Show leaderboard
        if !challenge.participation.leaderboard.is_empty() {
            info!("   🏆 Leaderboard:");
            for (i, participant) in challenge.participation.leaderboard.iter().enumerate().take(3) {
                info!("      {}. {} - {} badges (Score: {})", 
                    i + 1, participant.display_name, participant.badges_earned, participant.score);
            }
        }
    }

    Ok(())
}

async fn demo_viral_analytics(system: &NFTHeroBadgeSystem) -> Result<(), Box<dyn std::error::Error>> {
    info!("\n📈 Demo 6: Viral Analytics and Insights");
    info!("=======================================");
    
    let analytics = system.get_viral_analytics();
    
    info!("🚀 VIRAL GROWTH OVERVIEW:");
    info!("   🏆 Total Badges Minted: {}", analytics.total_minted);
    info!("   📤 Total Social Shares: {}", analytics.total_shares);
    info!("   🔄 Viral Coefficient: {:.2}", analytics.viral_coefficient);
    
    // Calculate engagement metrics
    let total_engagement: u32 = system.hero_badges.values()
        .map(|b| b.social_stats.likes + b.social_stats.comments + b.social_stats.shares)
        .sum();
    
    let avg_engagement = if analytics.total_minted > 0 {
        total_engagement as f64 / analytics.total_minted as f64
    } else {
        0.0
    };
    
    info!("   💖 Total Engagement: {}", total_engagement);
    info!("   📊 Avg Engagement per Badge: {:.1}", avg_engagement);

    info!("\n🏆 MOST VIRAL BADGES:");
    let mut viral_badges: Vec<_> = system.hero_badges.values().collect();
    viral_badges.sort_by(|a, b| b.social_stats.viral_score.partial_cmp(&a.social_stats.viral_score).unwrap());
    
    for (i, badge) in viral_badges.iter().enumerate().take(3) {
        info!("   {}. {} (Score: {:.1})", i + 1, badge.metadata.name, badge.social_stats.viral_score);
        info!("      👍 {} likes, 💬 {} comments, 📤 {} shares", 
            badge.social_stats.likes, badge.social_stats.comments, badge.social_stats.shares);
    }

    info!("\n📱 PLATFORM BREAKDOWN:");
    for (platform, shares) in &analytics.sharing_platforms {
        let percentage = if analytics.total_shares > 0 {
            (*shares as f64 / analytics.total_shares as f64) * 100.0
        } else {
            0.0
        };
        info!("   {}: {} shares ({:.1}%)", platform, shares, percentage);
    }

    info!("\n👥 USER ENGAGEMENT:");
    info!("   🟢 Daily Active Collectors: {}", analytics.user_engagement.daily_active_collectors);
    info!("   ⏱️  Avg Session Time: {:.1} minutes", analytics.user_engagement.avg_session_time_minutes);
    info!("   📚 Collection Completion: {:.1}%", analytics.user_engagement.collection_completion_rate * 100.0);
    info!("   💱 Trading Participation: {:.1}%", analytics.user_engagement.trading_participation_rate * 100.0);

    info!("\n🌟 RARITY DISTRIBUTION:");
    let mut rarity_stats = std::collections::HashMap::new();
    for badge in system.hero_badges.values() {
        *rarity_stats.entry(badge.metadata.rarity).or_insert(0) += 1;
    }
    
    for (rarity, count) in rarity_stats {
        let percentage = (count as f64 / analytics.total_minted as f64) * 100.0;
        info!("   {:?}: {} badges ({:.1}%)", rarity, count, percentage);
    }

    info!("\n💰 MARKET INSIGHTS:");
    let market_cap = system.marketplace.price_analytics.market_cap;
    let daily_volume = system.marketplace.trading_volume.daily_volume;
    
    info!("   💎 Market Cap: {} BONK", market_cap);
    info!("   📊 Daily Volume: {} BONK", daily_volume);
    info!("   🔥 Top Traded: {:?}", system.marketplace.trading_volume.top_traded);

    info!("\n🎯 VIRAL GROWTH PROJECTIONS:");
    let viral_coefficient = analytics.viral_coefficient;
    if viral_coefficient > 1.0 {
        info!("   🚀 VIRAL STATUS: Exponential growth! ({:.2}x multiplier)", viral_coefficient);
        info!("   📈 Projected Growth: {}% monthly increase", (viral_coefficient - 1.0) * 100.0);
    } else if viral_coefficient > 0.5 {
        info!("   📈 GROWTH STATUS: Strong organic growth ({:.2}x)", viral_coefficient);
        info!("   🎯 Near viral threshold - optimize sharing mechanics");
    } else {
        info!("   🌱 EARLY STATUS: Building foundation ({:.2}x)", viral_coefficient);
        info!("   💡 Focus on engagement and community building");
    }

    Ok(())
}
