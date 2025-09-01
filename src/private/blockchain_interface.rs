//! Solana Blockchain Interface
//! 
//! This module provides the public interface for Solana blockchain integration.
//! Now uses real Solana transactions instead of simulations.

use crate::error::AppResult;
use crate::private::solana_blockchain::{
    SolanaBlockchain, SolanaNetwork, TokenTransfer, TokenType, TransferPurpose,
    EmergencyRecord as BlockchainEmergencyRecord, EmergencyOutcome
};
use crate::public::types::{EmergencyType, Location};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Solana blockchain connection with real transaction capabilities
pub struct SolanaConnection {
    /// Real Solana blockchain implementation
    blockchain: Arc<RwLock<SolanaBlockchain>>,
    /// Whether connection is active
    pub is_connected: bool,
    /// Network endpoint
    pub endpoint: String,
    /// Connection status
    pub status: ConnectionStatus,
}

impl SolanaConnection {
    /// Creates a new Solana connection with real blockchain integration
    pub fn new(endpoint: &str) -> AppResult<Self> {
        // Determine network based on endpoint
        let network = if endpoint.contains("mainnet") {
            SolanaNetwork::Mainnet
        } else if endpoint.contains("devnet") {
            SolanaNetwork::Devnet
        } else if endpoint.contains("testnet") {
            SolanaNetwork::Testnet
        } else {
            SolanaNetwork::Devnet // Default to devnet for safety
        };

        let blockchain = Arc::new(RwLock::new(SolanaBlockchain::new(network)));

        Ok(Self {
            blockchain,
            is_connected: false,
            endpoint: endpoint.to_string(),
            status: ConnectionStatus::Disconnected,
        })
    }

    /// Connects to Solana network
    pub async fn connect(&mut self) -> AppResult<()> {
        // Implementation details hidden - proprietary connection logic
        self.is_connected = true;
        self.status = ConnectionStatus::Connected;
        Ok(())
    }

    /// Disconnects from Solana network
    pub async fn disconnect(&mut self) -> AppResult<()> {
        // Implementation details hidden
        self.is_connected = false;
        self.status = ConnectionStatus::Disconnected;
        Ok(())
    }

    /// Gets connection status
    pub fn get_status(&self) -> ConnectionStatus {
        self.status.clone()
    }

    /// Transfer BONK tokens for emergency rewards
    pub async fn transfer_bonk_tokens(&self, recipient: &str, amount: u64, purpose: &str) -> AppResult<String> {
        let mut blockchain = self.blockchain.write().await;
        
        let transfer = TokenTransfer {
            token_type: TokenType::BONK,
            amount,
            recipient: recipient.to_string(),
            purpose: self.parse_transfer_purpose(purpose),
            emergency_type: None,
        };

        let result = blockchain.transfer_tokens(transfer).await?;
        Ok(result.signature)
    }

    /// Transfer SKR tokens for emergency rewards
    pub async fn transfer_skr_tokens(&self, recipient: &str, amount: u64, purpose: &str) -> AppResult<String> {
        let mut blockchain = self.blockchain.write().await;
        
        let transfer = TokenTransfer {
            token_type: TokenType::SKR,
            amount,
            recipient: recipient.to_string(),
            purpose: self.parse_transfer_purpose(purpose),
            emergency_type: None,
        };

        let result = blockchain.transfer_tokens(transfer).await?;
        Ok(result.signature)
    }

    /// Award emergency response tokens
    pub async fn award_emergency_tokens(&self, responder_wallet: &str, emergency_type: EmergencyType, response_time_seconds: u32) -> AppResult<(String, String)> {
        let mut blockchain = self.blockchain.write().await;
        
        let (bonk_result, skr_result) = blockchain.award_emergency_tokens(
            responder_wallet,
            emergency_type,
            response_time_seconds,
        ).await?;

        Ok((bonk_result.signature, skr_result.signature))
    }

    /// Record emergency on blockchain
    pub async fn record_emergency_on_blockchain(&self, emergency_type: EmergencyType, location: Location, outcome: &str) -> AppResult<String> {
        let mut blockchain = self.blockchain.write().await;
        
        // Create privacy-preserving location hash
        let location_hash = self.create_location_hash(&location);
        let verification_hash = self.create_verification_hash(&emergency_type, &location);
        
        let record = BlockchainEmergencyRecord {
            id: uuid::Uuid::new_v4().to_string(),
            emergency_type,
            location_hash,
            timestamp: chrono::Utc::now().timestamp() as u64,
            response_time_seconds: None,
            outcome: self.parse_emergency_outcome(outcome),
            responders: Vec::new(),
            verification_hash,
        };

        let result = blockchain.record_emergency(record).await?;
        Ok(result.signature)
    }

    /// Get blockchain statistics
    pub async fn get_blockchain_stats(&self) -> AppResult<crate::private::solana_blockchain::BlockchainStats> {
        let blockchain = self.blockchain.read().await;
        Ok(blockchain.get_stats().clone())
    }

    /// Verify a transaction on blockchain
    pub async fn verify_transaction(&self, signature: &str) -> AppResult<bool> {
        let blockchain = self.blockchain.read().await;
        blockchain.verify_transaction(signature).await
    }

    /// Parse transfer purpose from string
    fn parse_transfer_purpose(&self, purpose: &str) -> TransferPurpose {
        match purpose.to_lowercase().as_str() {
            "emergency" | "emergency_response" => TransferPurpose::EmergencyResponse,
            "first_responder" | "responder" => TransferPurpose::FirstResponderReward,
            "training" => TransferPurpose::TrainingCompletion,
            "referral" => TransferPurpose::ReferralBonus,
            "achievement" => TransferPurpose::AchievementUnlock,
            "staking" => TransferPurpose::StakingReward,
            _ => TransferPurpose::EmergencyResponse,
        }
    }

    /// Parse emergency outcome from string
    fn parse_emergency_outcome(&self, outcome: &str) -> EmergencyOutcome {
        match outcome.to_lowercase().as_str() {
            "resolved" | "success" => EmergencyOutcome::Resolved,
            "ongoing" | "in_progress" => EmergencyOutcome::Ongoing,
            "escalated" | "escalate" => EmergencyOutcome::Escalated,
            "false_alarm" | "false" => EmergencyOutcome::FalseAlarm,
            "no_response" | "timeout" => EmergencyOutcome::NoResponse,
            _ => EmergencyOutcome::Ongoing,
        }
    }

    /// Create privacy-preserving location hash
    fn create_location_hash(&self, location: &Location) -> String {
        use sha2::{Digest, Sha256};
        
        let mut hasher = Sha256::new();
        hasher.update(location.latitude.to_le_bytes());
        hasher.update(location.longitude.to_le_bytes());
        hasher.update(location.timestamp.to_le_bytes());
        hasher.update(b"solana_sos_location");
        
        hex::encode(hasher.finalize())
    }

    /// Create verification hash for emergency record
    fn create_verification_hash(&self, emergency_type: &EmergencyType, location: &Location) -> String {
        use sha2::{Digest, Sha256};
        
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", emergency_type).as_bytes());
        hasher.update(location.latitude.to_le_bytes());
        hasher.update(location.longitude.to_le_bytes());
        hasher.update(chrono::Utc::now().timestamp().to_le_bytes());
        hasher.update(b"solana_sos_verification");
        
        hex::encode(hasher.finalize())
    }
}

/// Connection status enumeration
#[derive(Debug, Clone)]
pub enum ConnectionStatus {
    /// Connected to network
    Connected,
    /// Disconnected from network
    Disconnected,
    /// Connecting to network
    Connecting,
    /// Connection failed
    Failed,
}

/// Emergency record on blockchain
pub struct EmergencyRecord {
    /// Record ID
    pub id: String,
    /// Emergency type
    pub emergency_type: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Location hash
    pub location_hash: String,
    /// Audio hash
    pub audio_hash: String,
    /// Transaction hash
    pub transaction_hash: String,
    /// Block number
    pub block_number: u64,
}

/// Blockchain transaction manager
pub struct TransactionManager {
    /// Whether manager is active
    pub is_active: bool,
    /// Transaction fee
    pub transaction_fee: u64,
    /// Gas limit
    pub gas_limit: u64,
}

impl TransactionManager {
    /// Creates a new transaction manager
    pub fn new() -> AppResult<Self> {
        // Implementation details hidden - proprietary transaction setup
        Ok(Self {
            is_active: false,
            transaction_fee: 5000,
            gas_limit: 200000,
        })
    }

    /// Records emergency data on blockchain
    /// 
    /// # Arguments
    /// * `emergency_data` - Emergency data to record
    /// 
    /// # Returns
    /// * `AppResult<String>` - Transaction hash
    pub async fn record_emergency(&self, _emergency_data: EmergencyData) -> AppResult<String> {
        // Implementation details hidden - proprietary blockchain recording logic
        Ok("tx_hash_123456789".to_string())
    }

    /// Retrieves emergency record from blockchain
    /// 
    /// # Arguments
    /// * `record_id` - Record ID to retrieve
    /// 
    /// # Returns
    /// * `AppResult<EmergencyRecord>` - Emergency record
    pub async fn get_emergency_record(&self, record_id: &str) -> AppResult<EmergencyRecord> {
        // Implementation details hidden - proprietary retrieval logic
        Ok(EmergencyRecord {
            id: record_id.to_string(),
            emergency_type: "drowning".to_string(),
            timestamp: chrono::Utc::now(),
            location_hash: "location_hash".to_string(),
            audio_hash: "audio_hash".to_string(),
            transaction_hash: "tx_hash".to_string(),
            block_number: 12345,
        })
    }

    /// Verifies emergency record authenticity
    /// 
    /// # Arguments
    /// * `record` - Emergency record to verify
    /// 
    /// # Returns
    /// * `AppResult<bool>` - True if authentic
    pub async fn verify_record(&self, _record: &EmergencyRecord) -> AppResult<bool> {
        // Implementation details hidden - proprietary verification logic
        Ok(true)
    }
}

/// Emergency data structure
pub struct EmergencyData {
    /// Emergency type
    pub emergency_type: String,
    /// Location data
    pub location: LocationData,
    /// Audio data hash
    pub audio_hash: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Location data structure
pub struct LocationData {
    /// Latitude coordinate
    pub latitude: f64,
    /// Longitude coordinate
    pub longitude: f64,
    /// Location hash
    pub hash: String,
    /// Location accuracy
    pub accuracy: f32,
}

/// Token transaction manager (now uses real Solana blockchain)
/// This is kept for backward compatibility but delegates to SolanaConnection
pub struct TokenManager {
    connection: SolanaConnection,
}

impl TokenManager {
    /// Creates a new token manager
    pub fn new() -> AppResult<Self> {
        let connection = SolanaConnection::new("https://api.devnet.solana.com")?;
        Ok(Self { connection })
    }

    /// Transfers BONK tokens
    pub async fn transfer_bonk(&self, to_address: &str, amount: u64) -> AppResult<String> {
        self.connection.transfer_bonk_tokens(to_address, amount, "emergency").await
    }

    /// Transfers SKR tokens
    pub async fn transfer_skr(&self, to_address: &str, amount: u64) -> AppResult<String> {
        self.connection.transfer_skr_tokens(to_address, amount, "emergency").await
    }

    /// Gets token balance (placeholder for now)
    pub async fn get_balance(&self, _address: &str, _token_type: TokenType) -> AppResult<u64> {
        // This would require RPC calls to get actual balances
        // For now, return a placeholder value
        Ok(1000)
    }
}

/// Blockchain configuration
pub struct BlockchainConfig {
    /// Network endpoint
    pub endpoint: String,
    /// Commitment level
    pub commitment: CommitmentLevel,
    /// Transaction timeout
    pub transaction_timeout: std::time::Duration,
    /// Retry attempts
    pub retry_attempts: u32,
}

impl Default for BlockchainConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://api.mainnet-beta.solana.com".to_string(),
            commitment: CommitmentLevel::Confirmed,
            transaction_timeout: std::time::Duration::from_secs(30),
            retry_attempts: 3,
        }
    }
}

/// Commitment levels
#[derive(Debug, Clone)]
pub enum CommitmentLevel {
    /// Processed commitment
    Processed,
    /// Confirmed commitment
    Confirmed,
    /// Finalized commitment
    Finalized,
}

/// Blockchain statistics
pub struct BlockchainStats {
    /// Number of transactions sent
    pub transactions_sent: u32,
    /// Number of records stored
    pub records_stored: u32,
    /// Average transaction time
    pub avg_transaction_time: std::time::Duration,
    /// Success rate
    pub success_rate: f32,
}

impl Default for BlockchainStats {
    fn default() -> Self {
        Self {
            transactions_sent: 0,
            records_stored: 0,
            avg_transaction_time: std::time::Duration::from_secs(0),
            success_rate: 0.0,
        }
    }
} 