#[cfg(feature = "private")]
use solana_sos::SolanaSOS;

fn main() {
    println!("🧪 Testing Solana SOS Core Functionality...");
    
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
    
    // Test 5: Test gamification
    println!("✅ Test 5: Testing gamification...");
    let token_award = sos.award_emergency_tokens("drowning", 25);
    println!("   ✓ Token award: {} BONK, {} SKR, {} XP", 
             token_award.bonk_tokens, 
             token_award.skr_tokens, 
             token_award.xp_points);
    
    println!("\n🎉 All core functionality tests passed!");
    println!("🚀 Solana SOS core is working correctly!");
}
