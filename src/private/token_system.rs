//! Token System Module
//! 
//! Implements BONK and SKR token rewards, blockchain integration,
//! and token economics for the SOS Hero gamification system.

use crate::error::AppResult;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TokenSystem {
    user_wallet: UserWallet,
    token_economics: TokenEconomics,
    reward_history: Vec<TokenReward>,
    blockchain_integration: BlockchainIntegration,
}

#[derive(Debug, Clone)]
pub struct UserWallet {
    pub user_id: String,
    pub bonk_balance: u64,
    pub skr_balance: u64,
    pub wallet_address: String,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TokenEconomics {
    pub bonk_rewards: HashMap<String, u64>,
    pub skr_rewards: HashMap<String, u64>,
    pub inflation_rate: f64,
    pub max_supply_bonk: u64,
    pub max_supply_skr: u64,
}

#[derive(Debug, Clone)]
pub struct TokenReward {
    pub id: String,
    pub user_id: String,
    pub token_type: TokenType,
    pub amount: u64,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
    pub transaction_hash: Option<String>,
    pub status: RewardStatus,
}

#[derive(Debug, Clone)]
pub struct BlockchainIntegration {
    pub solana_connection: String,
    pub program_id: String,
    pub wallet_connected: bool,
    pub last_sync: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    BONK,
    SKR,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RewardStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl TokenSystem {
    pub fn new(user_id: String) -> Self {
        let mut system = TokenSystem {
            user_wallet: UserWallet {
                user_id: user_id.clone(),
                bonk_balance: 0,
                skr_balance: 0,
                wallet_address: "".to_string(),
                last_updated: Utc::now(),
            },
            token_economics: TokenEconomics {
                bonk_rewards: HashMap::new(),
                skr_rewards: HashMap::new(),
                inflation_rate: 0.05, // 5% annual inflation
                max_supply_bonk: 1_000_000_000, // 1 billion BONK
                max_supply_skr: 100_000_000, // 100 million SKR
            },
            reward_history: Vec::new(),
            blockchain_integration: BlockchainIntegration {
                solana_connection: "https://api.mainnet-beta.solana.com".to_string(),
                program_id: "SoS111111111111111111111111111111111111111111".to_string(),
                wallet_connected: false,
                last_sync: Utc::now(),
            },
        };
        
        system.initialize_token_economics();
        
        system
    }
    
    /// Award tokens for an action
    pub fn award_tokens(&mut self, action: &str, emergency_type: Option<&str>, response_time: Option<u32>) -> Result<TokenReward, String> {
        let (bonk_amount, skr_amount) = self.calculate_token_reward(action, emergency_type, response_time);
        
        // Create BONK reward
        if bonk_amount > 0 {
            let bonk_reward = TokenReward {
                id: uuid::Uuid::new_v4().to_string().to_string(),
                user_id: self.user_wallet.user_id.clone(),
                token_type: TokenType::BONK,
                amount: bonk_amount,
                reason: format!("{} reward", action),
                timestamp: Utc::now(),
                transaction_hash: None,
                status: RewardStatus::Pending,
            };
            
            self.reward_history.push(bonk_reward.clone());
            self.user_wallet.bonk_balance += bonk_amount;
        }
        
        // Create SKR reward
        if skr_amount > 0 {
            let skr_reward = TokenReward {
                id: uuid::Uuid::new_v4().to_string().to_string(),
                user_id: self.user_wallet.user_id.clone(),
                token_type: TokenType::SKR,
                amount: skr_amount,
                reason: format!("{} reward", action),
                timestamp: Utc::now(),
                transaction_hash: None,
                status: RewardStatus::Pending,
            };
            
            self.reward_history.push(skr_reward.clone());
            self.user_wallet.skr_balance += skr_amount;
        }
        
        // Update wallet timestamp
        self.user_wallet.last_updated = Utc::now();
        
        // Return the larger reward (or BONK if equal)
        if bonk_amount >= skr_amount {
            Ok(TokenReward {
                id: uuid::Uuid::new_v4().to_string().to_string(),
                user_id: self.user_wallet.user_id.clone(),
                token_type: TokenType::BONK,
                amount: bonk_amount,
                reason: format!("{} reward", action),
                timestamp: Utc::now(),
                transaction_hash: None,
                status: RewardStatus::Pending,
            })
        } else {
            Ok(TokenReward {
                id: uuid::Uuid::new_v4().to_string().to_string(),
                user_id: self.user_wallet.user_id.clone(),
                token_type: TokenType::SKR,
                amount: skr_amount,
                reason: format!("{} reward", action),
                timestamp: Utc::now(),
                transaction_hash: None,
                status: RewardStatus::Pending,
            })
        }
    }
    
    /// Award tokens for emergency intervention
    pub fn award_emergency_intervention(&mut self, emergency_type: &str, response_time: u32) -> Result<TokenReward, String> {
        let base_bonk = self.token_economics.bonk_rewards.get("emergency_intervention")
            .cloned()
            .unwrap_or(500);
        
        let base_skr = self.token_economics.skr_rewards.get("emergency_intervention")
            .cloned()
            .unwrap_or(250);
        
        // Time bonus for fast response
        let time_bonus_bonk = if response_time < 30 { 200 } else if response_time < 60 { 100 } else { 0 };
        let time_bonus_skr = if response_time < 30 { 100 } else if response_time < 60 { 50 } else { 0 };
        
        // Emergency type bonus
        let emergency_bonus_bonk = match emergency_type {
            "drowning" | "heart_attack" | "stroke" | "choking" | "allergic" => 300,
            "bleeding" | "unconscious" | "poisoning" | "trauma" => 200,
            _ => 100,
        };
        
        let emergency_bonus_skr = match emergency_type {
            "drowning" | "heart_attack" | "stroke" | "choking" | "allergic" => 150,
            "bleeding" | "unconscious" | "poisoning" | "trauma" => 100,
            _ => 50,
        };
        
        let total_bonk = base_bonk + time_bonus_bonk + emergency_bonus_bonk;
        let total_skr = base_skr + time_bonus_skr + emergency_bonus_skr;
        
        self.award_tokens("emergency_intervention", Some(emergency_type), Some(response_time))
    }
    
    /// Award tokens for achievement
    pub fn award_achievement(&mut self, achievement_id: &str) -> Result<TokenReward, String> {
        let bonk_amount = self.token_economics.bonk_rewards.get(achievement_id)
            .cloned()
            .unwrap_or(100);
        
        let skr_amount = self.token_economics.skr_rewards.get(achievement_id)
            .cloned()
            .unwrap_or(50);
        
        self.award_tokens("achievement", None, None)
    }
    
    /// Award tokens for level up
    pub fn award_level_up(&mut self, level: u32) -> Result<TokenReward, String> {
        let bonk_amount = level * 100; // 100 BONK per level
        let skr_amount = level * 50;   // 50 SKR per level
        
        self.award_tokens("level_up", None, None)
    }
    
    /// Get user wallet
    pub fn get_user_wallet(&self) -> &UserWallet {
        &self.user_wallet
    }
    
    /// Get reward history
    pub fn get_reward_history(&self) -> &Vec<TokenReward> {
        &self.reward_history
    }
    
    /// Get token economics
    pub fn get_token_economics(&self) -> &TokenEconomics {
        &self.token_economics
    }
    
    /// Connect wallet
    pub fn connect_wallet(&mut self, wallet_address: String) -> Result<(), String> {
        self.user_wallet.wallet_address = wallet_address.clone();
        self.blockchain_integration.wallet_connected = true;
        self.blockchain_integration.last_sync = Utc::now();
        
        // In real implementation, this would sync with blockchain
        Ok(())
    }
    
    /// Sync with blockchain
    pub async fn sync_with_blockchain(&mut self) -> Result<(), String> {
        if !self.blockchain_integration.wallet_connected {
            return Err("Wallet not connected".to_string());
        }
        
        // In real implementation, this would query the blockchain
        // For now, we'll simulate the sync
        
        self.blockchain_integration.last_sync = Utc::now();
        
        Ok(())
    }
    
    /// Transfer tokens
    pub async fn transfer_tokens(&mut self, recipient: &str, token_type: TokenType, amount: u64) -> Result<String, String> {
        if !self.blockchain_integration.wallet_connected {
            return Err("Wallet not connected".to_string());
        }
        
        // Check balance
        match token_type {
            TokenType::BONK => {
                if self.user_wallet.bonk_balance < amount {
                    return Err("Insufficient BONK balance".to_string());
                }
                self.user_wallet.bonk_balance -= amount;
            },
            TokenType::SKR => {
                if self.user_wallet.skr_balance < amount {
                    return Err("Insufficient SKR balance".to_string());
                }
                self.user_wallet.skr_balance -= amount;
            },
        }
        
        // In real implementation, this would create a blockchain transaction
        let transaction_hash = format!("tx_{}", uuid::Uuid::new_v4().to_string().to_string().replace("-", ""));
        
        // Update reward history
        let transfer_reward = TokenReward {
            id: uuid::Uuid::new_v4().to_string().to_string(),
            user_id: self.user_wallet.user_id.clone(),
            token_type,
            amount,
            reason: format!("Transfer to {}", recipient),
            timestamp: Utc::now(),
            transaction_hash: Some(transaction_hash.clone()),
            status: RewardStatus::Completed,
        };
        
        self.reward_history.push(transfer_reward);
        
        Ok(transaction_hash)
    }
    
    fn calculate_token_reward(&self, action: &str, emergency_type: Option<&str>, response_time: Option<u32>) -> (u64, u64) {
        let base_bonk = self.token_economics.bonk_rewards.get(action)
            .cloned()
            .unwrap_or(50);
        
        let base_skr = self.token_economics.skr_rewards.get(action)
            .cloned()
            .unwrap_or(25);
        
        let mut total_bonk = base_bonk;
        let mut total_skr = base_skr;
        
        // Emergency type bonus
        if let Some(emergency_type) = emergency_type {
            let emergency_bonus = match emergency_type {
                "drowning" | "heart_attack" | "stroke" | "choking" | "allergic" => 100,
                "bleeding" | "unconscious" | "poisoning" | "trauma" => 75,
                _ => 50,
            };
            total_bonk += emergency_bonus;
            total_skr += emergency_bonus / 2;
        }
        
        // Response time bonus
        if let Some(response_time) = response_time {
            if response_time < 30 {
                total_bonk += 100;
                total_skr += 50;
            } else if response_time < 60 {
                total_bonk += 50;
                total_skr += 25;
            }
        }
        
        (total_bonk, total_skr)
    }
    
    fn initialize_token_economics(&mut self) {
        // BONK rewards
        self.token_economics.bonk_rewards.insert("app_setup".to_string(), 100);
        self.token_economics.bonk_rewards.insert("emergency_intervention".to_string(), 500);
        self.token_economics.bonk_rewards.insert("achievement".to_string(), 200);
        self.token_economics.bonk_rewards.insert("level_up".to_string(), 100);
        self.token_economics.bonk_rewards.insert("safety_feature".to_string(), 50);
        self.token_economics.bonk_rewards.insert("learning".to_string(), 75);
        self.token_economics.bonk_rewards.insert("community".to_string(), 100);
        self.token_economics.bonk_rewards.insert("trusted_network".to_string(), 150);
        
        // SKR rewards
        self.token_economics.skr_rewards.insert("app_setup".to_string(), 50);
        self.token_economics.skr_rewards.insert("emergency_intervention".to_string(), 250);
        self.token_economics.skr_rewards.insert("achievement".to_string(), 100);
        self.token_economics.skr_rewards.insert("level_up".to_string(), 50);
        self.token_economics.skr_rewards.insert("safety_feature".to_string(), 25);
        self.token_economics.skr_rewards.insert("learning".to_string(), 40);
        self.token_economics.skr_rewards.insert("community".to_string(), 50);
        self.token_economics.skr_rewards.insert("trusted_network".to_string(), 75);
        
        // Achievement-specific rewards
        self.token_economics.bonk_rewards.insert("first_responder".to_string(), 150);
        self.token_economics.bonk_rewards.insert("safety_guardian".to_string(), 200);
        self.token_economics.bonk_rewards.insert("emergency_expert".to_string(), 300);
        self.token_economics.bonk_rewards.insert("network_builder".to_string(), 100);
        self.token_economics.bonk_rewards.insert("feature_master".to_string(), 150);
        self.token_economics.bonk_rewards.insert("quick_responder".to_string(), 100);
        self.token_economics.bonk_rewards.insert("learning_champion".to_string(), 200);
        self.token_economics.bonk_rewards.insert("community_hero".to_string(), 250);
        
        self.token_economics.skr_rewards.insert("first_responder".to_string(), 75);
        self.token_economics.skr_rewards.insert("safety_guardian".to_string(), 100);
        self.token_economics.skr_rewards.insert("emergency_expert".to_string(), 150);
        self.token_economics.skr_rewards.insert("network_builder".to_string(), 50);
        self.token_economics.skr_rewards.insert("feature_master".to_string(), 75);
        self.token_economics.skr_rewards.insert("quick_responder".to_string(), 50);
        self.token_economics.skr_rewards.insert("learning_champion".to_string(), 100);
        self.token_economics.skr_rewards.insert("community_hero".to_string(), 125);
    }
}
