// Blockchain Rewards System Example
//
// This example demonstrates the Solana blockchain integration for token rewards,
// showing how BONK and SKR tokens incentivize emergency preparedness training
// while maintaining user privacy and decentralization.

use std::collections::HashMap;

/// Represents a training completion event
#[derive(Debug, Clone)]
struct TrainingCompletion {
    user_id: String,
    module: TrainingModule,
    score: u8,
    completion_time_seconds: u32,
    timestamp: u64,
}

/// Available training modules
#[derive(Debug, Clone, PartialEq)]
enum TrainingModule {
    CPR,
    FirstAid,
    Heimlich,
    AED,
    Drowning,
    StrokeRecognition,
    Bleeding,
    BurnCare,
}

/// Token reward calculation result
#[derive(Debug)]
struct TokenReward {
    bonk_tokens: u64,
    skr_tokens: u64,
    reason: String,
    multiplier: f32,
}

/// Demonstrates blockchain reward system
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Blockchain Rewards System Example");
    println!("==================================\n");

    // Example 1: Basic reward calculation
    demonstrate_reward_calculation()?;

    // Example 2: Reward multipliers and bonuses
    demonstrate_reward_modifiers()?;

    // Example 3: Blockchain transaction flow
    demonstrate_transaction_flow()?;

    // Example 4: Decentralization and transparency
    demonstrate_blockchain_benefits()?;

    Ok(())
}

/// Demonstrates basic token reward calculation
fn demonstrate_reward_calculation() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 1: Token Reward Calculation");
    println!("-----------------------------------");

    let training_completion = TrainingCompletion {
        user_id: "user123".to_string(),
        module: TrainingModule::CPR,
        score: 95,
        completion_time_seconds: 300,
        timestamp: 1699564800,
    };

    println!("Training completed:");
    println!("  Module: {:?}", training_completion.module);
    println!("  Score: {}%", training_completion.score);
    println!("  Time: {}s", training_completion.completion_time_seconds);

    // Calculate base reward
    let base_reward = calculate_base_reward(&training_completion.module);
    println!("\nBase reward: {} BONK", base_reward);

    // Apply performance bonus
    let performance_multiplier = calculate_performance_multiplier(training_completion.score);
    println!("Performance multiplier: {:.2}x", performance_multiplier);

    // Calculate final reward
    let final_bonk = (base_reward as f32 * performance_multiplier) as u64;
    let final_skr = final_bonk / 2;

    println!("\nFinal rewards:");
    println!("  BONK tokens: {}", final_bonk);
    println!("  SKR tokens: {}", final_skr);
    println!("  Reason: Training completion with high score\n");

    Ok(())
}

/// Demonstrates reward multipliers and bonuses
fn demonstrate_reward_modifiers() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 2: Reward Multipliers and Bonuses");
    println!("-----------------------------------------");

    println!("Reward multipliers are applied for:");
    println!();

    println!("1. Performance Bonuses:");
    println!("   - 90-100% score: 1.5x multiplier");
    println!("   - 80-89% score: 1.2x multiplier");
    println!("   - 70-79% score: 1.0x multiplier");
    println!("   - Below 70%: 0.5x multiplier");

    println!();

    println!("2. Speed Bonuses:");
    println!("   - Completed in under 5 minutes: +50 BONK");
    println!("   - Completed in under 10 minutes: +25 BONK");

    println!();

    println!("3. Streak Bonuses:");
    println!("   - 3 modules in a row: +100 BONK");
    println!("   - 7 modules in a row: +500 BONK");
    println!("   - All modules completed: +2000 BONK");

    println!();

    println!("4. Emergency Response Rewards:");
    println!("   - Verified emergency assistance: +1000 BONK");
    println!("   - Successful intervention: +5000 BONK");
    println!("   - Life saved (verified): +10000 BONK");

    println!();

    // Example with multiple modifiers
    let modules_completed = vec![
        TrainingModule::CPR,
        TrainingModule::FirstAid,
        TrainingModule::Heimlich,
    ];

    println!("Example: User completed {} modules in sequence", modules_completed.len());
    let base_total = modules_completed.len() as u64 * 100;
    let streak_bonus = 100;
    let total = base_total + streak_bonus;

    println!("  Base rewards: {} BONK", base_total);
    println!("  Streak bonus: {} BONK", streak_bonus);
    println!("  Total earned: {} BONK\n", total);

    Ok(())
}

/// Demonstrates blockchain transaction flow
fn demonstrate_transaction_flow() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 3: Blockchain Transaction Flow");
    println!("--------------------------------------");

    println!("Reward distribution process:");
    println!();

    println!("Step 1: Training Verification");
    println!("  - User completes training module");
    println!("  - System validates completion criteria");
    println!("  - Score and time recorded locally");

    println!();

    println!("Step 2: Reward Calculation");
    println!("  - Base reward determined by module type");
    println!("  - Performance multipliers applied");
    println!("  - Bonuses calculated (speed, streak, etc.)");
    println!("  - Total BONK and SKR computed");

    println!();

    println!("Step 3: Transaction Building");
    println!("  - Solana transaction created locally");
    println!("  - Reward amounts encoded");
    println!("  - User wallet address included");
    println!("  - No personal data in transaction");

    println!();

    println!("Step 4: Signature and Submission");
    println!("  - Transaction signed via Mobile Wallet Adapter");
    println!("  - User approves transaction (gas fees minimal)");
    println!("  - Submitted to Solana network");
    println!("  - Confirmation within seconds");

    println!();

    println!("Step 5: Verification");
    println!("  - Blockchain confirms transaction");
    println!("  - Tokens appear in user wallet");
    println!("  - On-chain record created (transparent)");
    println!("  - Local cache updated");

    println!();

    println!("Privacy guarantees:");
    println!("  - Only wallet address and reward amount on-chain");
    println!("  - No training scores or times recorded publicly");
    println!("  - No linkage to personal identity");
    println!("  - Complete transaction transparency\n");

    Ok(())
}

/// Demonstrates blockchain benefits for emergency preparedness
fn demonstrate_blockchain_benefits() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 4: Blockchain Benefits");
    println!("------------------------------");

    println!("Solana blockchain provides:");
    println!();

    println!("1. Decentralization:");
    println!("   - No central authority controls rewards");
    println!("   - Transparent reward distribution");
    println!("   - Immutable training records");
    println!("   - Censorship-resistant");

    println!();

    println!("2. Transparency:");
    println!("   - All transactions publicly verifiable");
    println!("   - Reward calculations auditable");
    println!("   - Token supply visible");
    println!("   - Fair distribution provable");

    println!();

    println!("3. Performance:");
    println!("   - Sub-second transaction confirmation");
    println!("   - Minimal gas fees (fractions of a cent)");
    println!("   - High throughput for scaling");
    println!("   - Mobile-optimized via Solana Mobile Stack");

    println!();

    println!("4. Incentive Alignment:");
    println!("   - Rewards drive emergency preparedness");
    println!("   - Gamification encourages training");
    println!("   - Token value tied to app success");
    println!("   - Community benefits from participation");

    println!();

    println!("5. Integration:");
    println!("   - Mobile Wallet Adapter for seamless UX");
    println!("   - Compatible with Solana ecosystem");
    println!("   - Tokens tradeable on DEXs");
    println!("   - Future DeFi integration possible");

    println!();

    Ok(())
}

/// Calculate base reward for training module
fn calculate_base_reward(module: &TrainingModule) -> u64 {
    match module {
        TrainingModule::CPR => 150,
        TrainingModule::FirstAid => 100,
        TrainingModule::Heimlich => 100,
        TrainingModule::AED => 125,
        TrainingModule::Drowning => 150,
        TrainingModule::StrokeRecognition => 125,
        TrainingModule::Bleeding => 100,
        TrainingModule::BurnCare => 100,
    }
}

/// Calculate performance multiplier based on score
fn calculate_performance_multiplier(score: u8) -> f32 {
    match score {
        90..=100 => 1.5,
        80..=89 => 1.2,
        70..=79 => 1.0,
        _ => 0.5,
    }
}

/// Example test cases for reward calculation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_reward_calculation() {
        assert_eq!(calculate_base_reward(&TrainingModule::CPR), 150);
        assert_eq!(calculate_base_reward(&TrainingModule::FirstAid), 100);
    }

    #[test]
    fn test_performance_multiplier() {
        assert_eq!(calculate_performance_multiplier(95), 1.5);
        assert_eq!(calculate_performance_multiplier(85), 1.2);
        assert_eq!(calculate_performance_multiplier(75), 1.0);
        assert_eq!(calculate_performance_multiplier(65), 0.5);
    }

    #[test]
    fn test_reward_fairness() {
        // High score should earn more
        let high_score_reward = 150.0 * calculate_performance_multiplier(95);
        let low_score_reward = 150.0 * calculate_performance_multiplier(65);

        assert!(high_score_reward > low_score_reward);
    }
}

