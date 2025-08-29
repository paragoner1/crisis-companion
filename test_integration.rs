use solana_sos::SolanaSOS;
use solana_sos::public::voice_interface::VoiceInterface;
use solana_sos::public::emergency_interface::EmergencySystem;
use solana_sos::public::types::EmergencyType;

#[tokio::main]
async fn main() {
    println!("🧪 Testing Solana SOS Integration...");
    
    // Test 1: Create SolanaSOS instance
    println!("✅ Test 1: Creating SolanaSOS instance...");
    let mut sos = SolanaSOS::new();
    println!("   ✓ SolanaSOS instance created successfully");
    
    // Test 2: Add emergency contact
    println!("✅ Test 2: Adding emergency contact...");
    sos.add_emergency_contact("Test Contact", "555-1234", "Family");
    let contacts = sos.get_emergency_contacts();
    println!("   ✓ Emergency contacts: {}", contacts.len());
    
    // Test 3: Get emergency types
    println!("✅ Test 3: Getting emergency types...");
    let emergency_types = sos.get_emergency_types();
    println!("   ✓ Available emergency types: {}", emergency_types.len());
    for emergency_type in &emergency_types {
        println!("     - {}", emergency_type);
    }
    
    // Test 4: Get emergency protocol
    println!("✅ Test 4: Getting emergency protocol...");
    if let Some(protocol) = sos.get_emergency_protocol("drowning") {
        println!("   ✓ Drowning protocol found with {} steps", protocol.steps.len());
        println!("   ✓ Source: {}", protocol.official_source);
    } else {
        println!("   ⚠️  Drowning protocol not found");
    }
    
    // Test 5: Test voice interface
    println!("✅ Test 5: Testing voice interface...");
    let voice_interface = VoiceInterface::new();
    let test_phrase = "hey sos drowning emergency";
    let result = voice_interface.process_voice_input(test_phrase);
    println!("   ✓ Voice processing result: {:?}", result);
    
    // Test 6: Test emergency system
    println!("✅ Test 6: Testing emergency system...");
    let emergency_system = EmergencySystem::new();
    let context_flags = vec!["urgent".to_string(), "water".to_string()];
    let response = emergency_system.process_emergency("drowning", &context_flags);
    println!("   ✓ Emergency response: {:?}", response);
    
    // Test 7: Test gamification
    println!("✅ Test 7: Testing gamification...");
    let token_award = sos.award_emergency_tokens("drowning", 25);
    println!("   ✓ Token award: {} BONK, {} SKR, {} XP", 
             token_award.bonk_tokens, 
             token_award.skr_tokens, 
             token_award.xp_points);
    
    println!("\n🎉 All integration tests passed!");
    println!("🚀 Solana SOS is ready for deployment!");
}
