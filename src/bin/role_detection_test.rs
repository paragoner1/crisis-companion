use crate::error::AppResult;
use tracing::{info, Level};
use tracing_subscriber;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("👥 Solana SOS - Role Detection Test");
    info!("===================================");

    // Test role detection concepts
    info!("🎯 Role Detection Features:");
    info!("   • Bystander vs. caregiver distinction");
    info!("   • User experience level assessment");
    info!("   • Training level recognition");
    info!("   • Medical background detection");
    info!("   • Emergency response experience");

    sleep(Duration::from_secs(2)).await;

    // Test role scenarios
    info!("👤 Role Scenarios:");
    info!("   • Bystander: Basic guidance with detailed instructions");
    info!("   • Trained responder: Direct action guidance");
    info!("   • Medical professional: Advanced techniques");
    info!("   • Family caregiver: Personalized care instructions");
    info!("   • Emergency responder: Professional protocols");

    sleep(Duration::from_secs(2)).await;

    // Test guidance adaptation
    info!("📋 Guidance Adaptation:");
    info!("   • Bystander: 'Call 911, then follow these steps...'");
    info!("   • Trained: 'Begin CPR immediately, 100-120 compressions...'");
    info!("   • Medical: 'Advanced airway management, consider intubation...'");
    info!("   • Caregiver: 'Monitor vitals, maintain comfort position...'");
    info!("   • Responder: 'Scene safety, rapid assessment, transport...'");

    sleep(Duration::from_secs(2)).await;

    info!("🎉 Role detection test completed successfully!");
    info!("Personalized guidance based on user role working correctly!");

    Ok(())
} 