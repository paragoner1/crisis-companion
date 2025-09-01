//! First Responder Network Demo
//! 
//! This demo showcases the first responder network functionality,
//! demonstrating how emergency broadcasts are sent to nearby qualified responders.

use solana_sos::private::first_responder_network::{
    FirstResponderNetwork, FirstResponder, ResponderType, ResponderStatus, 
    VerificationStatus, ResponseHistory, ResponseDecision
};
use solana_sos::public::types::{EmergencyType, Location};
use std::sync::Arc;
use tokio;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("🚨 First Responder Network Demo - Solana SOS");
    println!("=============================================");

    // Create first responder network
    let network = Arc::new(FirstResponderNetwork::new());
    
    // Register some first responders in San Francisco area
    register_demo_responders(&network).await?;
    
    // Simulate an emergency broadcast
    simulate_emergency_broadcast(&network).await?;
    
    // Demonstrate responder responses
    demonstrate_responder_responses(&network).await?;
    
    println!("\n✅ Demo completed successfully!");
    println!("📡 The first responder network is ready for mass adoption and life-saving functionality!");
    
    Ok(())
}

async fn register_demo_responders(network: &Arc<FirstResponderNetwork>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 Registering Demo First Responders...");
    
    let responders = vec![
        // EMT near downtown SF
        FirstResponder {
            id: "resp_001".to_string(),
            user_id: "user_emt_001".to_string(),
            responder_type: ResponderType::EMT,
            certifications: vec!["CPR".to_string(), "AED".to_string(), "First Aid".to_string()],
            current_location: Location {
                latitude: 37.7749,   // San Francisco
                longitude: -122.4194,
                altitude: None,
                accuracy: Some(10.0),
                timestamp: Utc::now().timestamp() as u64,
            },
            status: ResponderStatus::Available,
            availability_radius_meters: 2000.0,
            emergency_specialties: vec![EmergencyType::HeartAttack, EmergencyType::Stroke],
            verification_status: VerificationStatus::Verified,
            response_history: ResponseHistory {
                total_responses: 25,
                successful_responses: 23,
                average_response_time_seconds: 180.0,
                reliability_score: 0.92,
                last_response: Some(Utc::now()),
            },
            last_active: Utc::now(),
            solana_wallet: Some("EMT1ABC123DEF456".to_string()),
        },
        
        // Paramedic in Mission District
        FirstResponder {
            id: "resp_002".to_string(),
            user_id: "user_paramedic_001".to_string(),
            responder_type: ResponderType::Paramedic,
            certifications: vec!["CPR".to_string(), "AED".to_string(), "Advanced Life Support".to_string()],
            current_location: Location {
                latitude: 37.7599,   // Mission District
                longitude: -122.4148,
                altitude: None,
                accuracy: Some(5.0),
                timestamp: Utc::now().timestamp() as u64,
            },
            status: ResponderStatus::Available,
            availability_radius_meters: 3000.0,
            emergency_specialties: vec![EmergencyType::HeartAttack, EmergencyType::Choking, EmergencyType::Drowning],
            verification_status: VerificationStatus::Verified,
            response_history: ResponseHistory {
                total_responses: 45,
                successful_responses: 44,
                average_response_time_seconds: 120.0,
                reliability_score: 0.98,
                last_response: Some(Utc::now()),
            },
            last_active: Utc::now(),
            solana_wallet: Some("PARA2XYZ789GHI012".to_string()),
        },
        
        // Trained citizen in Castro District
        FirstResponder {
            id: "resp_003".to_string(),
            user_id: "user_citizen_001".to_string(),
            responder_type: ResponderType::TrainedCitizen,
            certifications: vec!["CPR".to_string(), "First Aid".to_string()],
            current_location: Location {
                latitude: 37.7609,   // Castro District
                longitude: -122.4350,
                altitude: None,
                accuracy: Some(15.0),
                timestamp: Utc::now().timestamp() as u64,
            },
            status: ResponderStatus::Available,
            availability_radius_meters: 1000.0,
            emergency_specialties: vec![EmergencyType::Choking, EmergencyType::AllergicReaction],
            verification_status: VerificationStatus::Verified,
            response_history: ResponseHistory {
                total_responses: 8,
                successful_responses: 7,
                average_response_time_seconds: 240.0,
                reliability_score: 0.88,
                last_response: Some(Utc::now()),
            },
            last_active: Utc::now(),
            solana_wallet: Some("CITIZEN3JKL345MNO678".to_string()),
        },
    ];
    
    for responder in responders {
        network.register_responder(responder.clone()).await?;
        println!("✅ Registered {} - {} ({})", 
                 responder.id, 
                 format!("{:?}", responder.responder_type),
                 responder.certifications.join(", "));
    }
    
    Ok(())
}

async fn simulate_emergency_broadcast(network: &Arc<FirstResponderNetwork>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚨 Simulating Emergency Broadcast...");
    
    // Emergency location near Pier 39 (tourist area)
    let emergency_location = Location {
        latitude: 37.8087,   // Pier 39, SF
        longitude: -122.4098,
        altitude: None,
        accuracy: Some(5.0),
        timestamp: Utc::now().timestamp() as u64,
    };
    
    println!("📍 Emergency Location: Pier 39, San Francisco");
    println!("🆘 Emergency Type: Heart Attack");
    println!("⚠️  Severity Level: 9/10 (Critical)");
    
    let broadcast_id = network.broadcast_emergency(
        EmergencyType::HeartAttack,
        emergency_location,
        9, // Critical severity
        "Tourist collapsed with chest pain, not responsive. CPR needed immediately.".to_string(),
        "tourist_emergency_001".to_string(),
    ).await?;
    
    println!("📡 Emergency broadcast sent with ID: {}", broadcast_id);
    
    // Give some time for notifications to be processed
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let active_broadcasts = network.get_active_broadcasts().await;
    println!("📊 Active broadcasts: {}", active_broadcasts.len());
    
    if let Some(broadcast) = active_broadcasts.first() {
        println!("   📋 Broadcast Details:");
        println!("      - Emergency: {:?}", broadcast.emergency_type);
        println!("      - Priority: {:?}", broadcast.priority);
        println!("      - Radius: {:.0}m", broadcast.radius_meters);
        println!("      - Required Certs: {}", broadcast.required_certifications.join(", "));
    }
    
    Ok(())
}

async fn demonstrate_responder_responses(network: &Arc<FirstResponderNetwork>) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📱 Demonstrating Responder Responses...");
    
    let active_broadcasts = network.get_active_broadcasts().await;
    if let Some(broadcast) = active_broadcasts.first() {
        let broadcast_id = &broadcast.id;
        
        // Simulate different responder responses
        println!("👨‍⚕️ EMT (resp_001) accepting emergency...");
        network.handle_responder_response(
            broadcast_id,
            "resp_001",
            ResponseDecision::Accepting,
        ).await?;
        
        println!("🚑 Paramedic (resp_002) accepting emergency...");
        network.handle_responder_response(
            broadcast_id,
            "resp_002", 
            ResponseDecision::Accepting,
        ).await?;
        
        println!("👤 Trained citizen (resp_003) declining (already helping someone else)...");
        network.handle_responder_response(
            broadcast_id,
            "resp_003",
            ResponseDecision::Declining,
        ).await?;
        
        // Check updated broadcast status
        let updated_broadcasts = network.get_active_broadcasts().await;
        if let Some(updated_broadcast) = updated_broadcasts.first() {
            println!("📊 Broadcast Status: {:?}", updated_broadcast.status);
        }
        
        println!("✅ Multiple responders are now en route to the emergency!");
        println!("🏥 Emergency services coordination in progress...");
    }
    
    Ok(())
}
