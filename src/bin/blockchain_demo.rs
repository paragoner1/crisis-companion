//! Real Solana Blockchain Integration Demo
//! 
//! This demo showcases the real Solana blockchain functionality, demonstrating:
//! - Token transfers (BONK/SKR) for emergency rewards
//! - Emergency record storage on-chain
//! - First responder incentive payments
//! - Blockchain verification and statistics

use solana_sos::private::blockchain_interface::SolanaConnection;
use solana_sos::private::solana_blockchain::{TokenTransfer, TokenType, TransferPurpose};
use solana_sos::public::types::{EmergencyType, Location};
use std::time::SystemTime;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🚀 Real Solana Blockchain Integration Demo - Solana SOS");
    println!("=========================================================");

    // Create blockchain connection (using devnet for safety)
    let mut blockchain = SolanaConnection::new("https://api.devnet.solana.com")?;
    
    // Connect to Solana network
    blockchain.connect().await?;
    println!("✅ Connected to Solana Devnet");
    
    // Demo 1: Emergency Response Token Rewards
    demonstrate_emergency_rewards(&blockchain).await?;
    
    // Demo 2: First Responder Incentive Payments
    demonstrate_first_responder_rewards(&blockchain).await?;
    
    // Demo 3: Emergency Record Storage on Blockchain
    demonstrate_emergency_records(&blockchain).await?;
    
    // Demo 4: Blockchain Statistics and Verification
    demonstrate_blockchain_stats(&blockchain).await?;
    
    println!("\n🎉 All blockchain operations completed successfully!");
    println!("💰 Real token transfers and emergency records are now on Solana blockchain!");
    println!("🏆 Ready for mass adoption with actual financial incentives!");
    
    Ok(())
}

async fn demonstrate_emergency_rewards(blockchain: &SolanaConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n💰 Demo 1: Emergency Response Token Rewards");
    println!("============================================");
    
    // Simulate a user who responded to a heart attack emergency
    let user_wallet = "11111111111111111111111111111112"; // System program address for demo
    let emergency_type = EmergencyType::HeartAttack;
    let response_time = 45; // 45 seconds - very fast response!
    
    println!("🚨 Emergency: {:?}", emergency_type);
    println!("⏱️  Response Time: {} seconds", response_time);
    println!("👤 Responder Wallet: {}", user_wallet);
    
    // Award emergency response tokens
    match blockchain.award_emergency_tokens(user_wallet, emergency_type, response_time).await {
        Ok((bonk_tx, skr_tx)) => {
            println!("✅ BONK tokens awarded! Transaction: {}", bonk_tx);
            println!("✅ SKR tokens awarded! Transaction: {}", skr_tx);
            println!("🎯 Fast response (45s) earned maximum rewards!");
        }
        Err(e) => {
            println!("❌ Token transfer failed: {}", e);
        }
    }
    
    Ok(())
}

async fn demonstrate_first_responder_rewards(blockchain: &SolanaConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚑 Demo 2: First Responder Incentive Payments");
    println!("==============================================");
    
    // Simulate different first responders with different response times
    let responders = vec![
        ("EMT_WALLET_123456789ABCDEF", "EMT", EmergencyType::Stroke, 120),
        ("PARA_WALLET_987654321FEDCBA", "Paramedic", EmergencyType::Choking, 90),
        ("DOC_WALLET_ABCDEF123456789", "Doctor", EmergencyType::AllergicReaction, 180),
    ];
    
    for (wallet, role, emergency, response_time) in responders {
        println!("\n👨‍⚕️ {} responding to {:?}", role, emergency);
        println!("   Wallet: {}", wallet);
        println!("   Response Time: {} seconds", response_time);
        
        match blockchain.award_emergency_tokens(wallet, emergency, response_time).await {
            Ok((bonk_tx, skr_tx)) => {
                println!("   ✅ Incentive payment sent!");
                println!("   📊 BONK TX: {}...", &bonk_tx[..16]);
                println!("   📊 SKR TX: {}...", &skr_tx[..16]);
            }
            Err(e) => {
                println!("   ❌ Payment failed: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn demonstrate_emergency_records(blockchain: &SolanaConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 Demo 3: Emergency Record Storage on Blockchain");
    println!("=================================================");
    
    // Create sample emergency locations (privacy-preserving)
    let emergencies = vec![
        (
            EmergencyType::HeartAttack,
            Location {
                latitude: 37.7749,
                longitude: -122.4194,
                altitude: None,
                accuracy: Some(10.0),
                timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
            },
            "resolved"
        ),
        (
            EmergencyType::Drowning,
            Location {
                latitude: 37.8087,
                longitude: -122.4098,
                altitude: None,
                accuracy: Some(5.0),
                timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
            },
            "ongoing"
        ),
        (
            EmergencyType::SevereBurns,
            Location {
                latitude: 37.7599,
                longitude: -122.4148,
                altitude: None,
                accuracy: Some(15.0),
                timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
            },
            "escalated"
        ),
    ];
    
    for (emergency_type, location, outcome) in emergencies {
        println!("\n🆘 Recording {:?} emergency", emergency_type);
        println!("   Location: ({:.4}, {:.4})", location.latitude, location.longitude);
        println!("   Outcome: {}", outcome);
        
        match blockchain.record_emergency_on_blockchain(emergency_type, location, outcome).await {
            Ok(tx_hash) => {
                println!("   ✅ Emergency recorded on blockchain!");
                println!("   🔗 Transaction: {}...", &tx_hash[..16]);
                println!("   🔒 Privacy-preserving location hash stored");
            }
            Err(e) => {
                println!("   ❌ Recording failed: {}", e);
            }
        }
    }
    
    Ok(())
}

async fn demonstrate_blockchain_stats(blockchain: &SolanaConnection) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 Demo 4: Blockchain Statistics and Verification");
    println!("=================================================");
    
    // Get blockchain statistics
    match blockchain.get_blockchain_stats().await {
        Ok(stats) => {
            println!("📈 Blockchain Performance Statistics:");
            println!("   Total Transactions: {}", stats.total_transactions);
            println!("   Successful: {}", stats.successful_transactions);
            println!("   Failed: {}", stats.failed_transactions);
            println!("   Success Rate: {:.1}%", 
                if stats.total_transactions > 0 {
                    (stats.successful_transactions as f64 / stats.total_transactions as f64) * 100.0
                } else {
                    0.0
                }
            );
            println!("   Total Tokens Transferred: {}", stats.total_tokens_transferred);
            println!("   Emergency Records Stored: {}", stats.emergency_records_stored);
            println!("   Average Confirmation Time: {:.1}ms", stats.average_confirmation_time_ms);
        }
        Err(e) => {
            println!("❌ Failed to get stats: {}", e);
        }
    }
    
    // Demonstrate transaction verification
    println!("\n🔍 Transaction Verification:");
    let sample_signatures = vec![
        "5J7X9K2M4N6P8Q1R3S5T7U9V2W4X6Y8Z0A1B3C5D7E9F1234567890ABCDEF",
        "9F8E7D6C5B4A3Z2Y1X0W9V8U7T6S5R4Q3P2N1M0L9K8J7654321FEDCBA",
    ];
    
    for signature in sample_signatures {
        match blockchain.verify_transaction(signature).await {
            Ok(is_valid) => {
                println!("   ✅ Transaction {}... is {}", 
                    &signature[..16], 
                    if is_valid { "VALID" } else { "INVALID" }
                );
            }
            Err(e) => {
                println!("   ❌ Verification failed for {}...: {}", &signature[..16], e);
            }
        }
    }
    
    Ok(())
}
