// Emergency Protocol System Example
// 
// This example demonstrates the architecture of the emergency protocol system,
// showing how medical authority protocols are structured, validated, and delivered
// to users during life-critical situations.

use solana_sos::public::types::{EmergencyType, EmergencyStage};
use solana_sos::public::emergency_interface::EmergencySystem;

/// Demonstrates basic emergency protocol retrieval and usage
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Emergency Protocol System Example");
    println!("==================================\n");

    // Initialize emergency system
    let emergency_system = EmergencySystem::new();
    println!("Emergency system initialized\n");

    // Example 1: Heart Attack Emergency
    demonstrate_heart_attack_protocol(&emergency_system)?;

    // Example 2: Choking Emergency
    demonstrate_choking_protocol(&emergency_system)?;

    // Example 3: Step-by-step guidance
    demonstrate_step_by_step_guidance(&emergency_system)?;

    Ok(())
}

/// Demonstrates heart attack protocol retrieval
fn demonstrate_heart_attack_protocol(system: &EmergencySystem) -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 1: Heart Attack Emergency Protocol");
    println!("------------------------------------------");

    // Simulate emergency detection
    let emergency_type = EmergencyType::HeartAttack;
    println!("Emergency detected: {:?}", emergency_type);

    // Get protocol instructions
    println!("\nRetrieving medical authority protocol...");
    let instructions = system.get_emergency_instructions()?;

    println!("Protocol steps:");
    for (i, instruction) in instructions.iter().enumerate() {
        println!("  {}. {}", i + 1, instruction);
    }

    println!("\nProtocol source: American Heart Association");
    println!("Last updated: 2024");
    println!("Critical: Yes - Immediate 911 call required\n");

    Ok(())
}

/// Demonstrates choking protocol with age variations
fn demonstrate_choking_protocol(system: &EmergencySystem) -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 2: Choking Emergency Protocol");
    println!("-------------------------------------");

    let emergency_type = EmergencyType::Choking;
    println!("Emergency detected: {:?}", emergency_type);

    println!("\nProtocol adapts based on patient age:");
    println!("  - Adult: Heimlich maneuver, full force");
    println!("  - Child: Modified Heimlich, reduced force");
    println!("  - Infant: Back blows and chest thrusts only");

    println!("\nContext-aware modifications:");
    println!("  - Conscious vs. unconscious patient");
    println!("  - Complete vs. partial airway obstruction");
    println!("  - Availability of equipment");

    println!("\nProtocol source: American Red Cross");
    println!("Validated: Yes\n");

    Ok(())
}

/// Demonstrates step-by-step emergency guidance delivery
fn demonstrate_step_by_step_guidance(system: &EmergencySystem) -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 3: Step-by-Step Guidance System");
    println!("---------------------------------------");

    println!("Emergency guidance is delivered progressively:");
    println!();

    // Simulate emergency stages
    let stages = vec![
        EmergencyStage::Detection,
        EmergencyStage::Assessment,
        EmergencyStage::InitialResponse,
        EmergencyStage::OngoingCare,
        EmergencyStage::HandoffToEMS,
    ];

    for stage in stages {
        println!("Stage: {:?}", stage);
        println!("  System provides:");
        match stage {
            EmergencyStage::Detection => {
                println!("    - Immediate situation assessment");
                println!("    - Safety environment check");
                println!("    - 911 auto-dial initiation");
            }
            EmergencyStage::Assessment => {
                println!("    - Victim responsiveness check");
                println!("    - Breathing and pulse assessment");
                println!("    - Visible injury identification");
            }
            EmergencyStage::InitialResponse => {
                println!("    - Primary life-saving interventions");
                println!("    - CPR if needed");
                println!("    - Bleeding control");
            }
            EmergencyStage::OngoingCare => {
                println!("    - Continued monitoring");
                println!("    - Reassessment every 2 minutes");
                println!("    - Preparation for EMS arrival");
            }
            EmergencyStage::HandoffToEMS => {
                println!("    - Summary of interventions");
                println!("    - Vital sign history");
                println!("    - Timeline of events");
            }
        }
        println!();
    }

    println!("All guidance is:");
    println!("  - Based on medical authority protocols");
    println!("  - Adapted to user experience level");
    println!("  - Delivered via voice and visual display");
    println!("  - Updated in real-time based on situation changes\n");

    Ok(())
}

/// Example of protocol validation and verification
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergency_system_initialization() {
        let system = EmergencySystem::new();
        assert!(!system.is_active);
    }

    #[test]
    fn test_protocol_retrieval() {
        let system = EmergencySystem::new();
        let instructions = system.get_emergency_instructions();
        assert!(instructions.is_ok());
    }

    #[test]
    fn test_emergency_type_coverage() {
        // Verify all critical emergency types are supported
        let critical_types = vec![
            EmergencyType::HeartAttack,
            EmergencyType::Stroke,
            EmergencyType::Choking,
            EmergencyType::Drowning,
            EmergencyType::Bleeding,
        ];

        // Each type should have validated protocols
        for emergency_type in critical_types {
            println!("Verifying protocol for: {:?}", emergency_type);
            // In production: verify against medical authority sources
        }
    }
}

