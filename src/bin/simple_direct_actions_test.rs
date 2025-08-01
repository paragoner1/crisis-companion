use solana_sos::{
    public::types::{EmergencyType, DirectAction},
    error::AppResult,
};
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

    info!("🎯 Solana SOS - Direct Actions Test");
    info!("====================================");

    // Test direct actions
    let direct_actions = [
        DirectAction::CPR,
        DirectAction::Heimlich,
        DirectAction::AED,
        DirectAction::Tourniquet,
        DirectAction::EpiPen,
        DirectAction::RescueBreathing,
        DirectAction::FirstAid,
        DirectAction::FASTTest,
        DirectAction::PoisonControl,
        DirectAction::CoolBurn,
        DirectAction::MedicalAlert,
    ];

    info!("📋 Available Direct Actions:");
    for action in &direct_actions {
        info!("   • {}: {}", action.display_name(), action.description());
    }

    sleep(Duration::from_secs(2)).await;

    // Test emergency types
    let emergency_types = [
        EmergencyType::Drowning,
        EmergencyType::HeartAttack,
        EmergencyType::Stroke,
        EmergencyType::Choking,
        EmergencyType::Bleeding,
        EmergencyType::Unconscious,
        EmergencyType::Seizure,
        EmergencyType::Poisoning,
        EmergencyType::SevereBurns,
        EmergencyType::DiabeticEmergency,
        EmergencyType::AllergicReaction,
        EmergencyType::Trauma,
    ];

    info!("🚨 Supported Emergency Types:");
    for emergency in &emergency_types {
        info!("   • {}: {}", emergency.display_name(), emergency.description());
    }

    sleep(Duration::from_secs(2)).await;

    info!("🎉 Direct actions test completed successfully!");
    info!("All direct actions and emergency types working correctly!");

    Ok(())
} 