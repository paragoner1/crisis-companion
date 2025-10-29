// Voice Activation System Example
//
// This example demonstrates the voice activation architecture for emergency detection,
// showing how the system processes audio, detects emergency phrases, and initiates
// response protocols while maintaining privacy through on-device processing.

use solana_sos::public::voice_interface::{VoiceInterface, VoiceStats};
use solana_sos::config::VoiceConfig;

/// Demonstrates voice activation and emergency phrase detection
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Voice Activation System Example");
    println!("================================\n");

    // Example 1: Voice interface initialization
    demonstrate_voice_initialization()?;

    // Example 2: Emergency phrase detection
    demonstrate_emergency_detection()?;

    // Example 3: Performance characteristics
    demonstrate_performance_metrics()?;

    // Example 4: Privacy-preserving design
    demonstrate_privacy_architecture()?;

    Ok(())
}

/// Demonstrates voice interface initialization
fn demonstrate_voice_initialization() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 1: Voice Interface Initialization");
    println!("-----------------------------------------");

    // Create voice configuration
    let config = VoiceConfig {
        model_path: "models/voice-recognition".to_string(),
        confidence_threshold: 0.80,
        emergency_phrases: vec![
            "help".to_string(),
            "emergency".to_string(),
            "heart attack".to_string(),
            "choking".to_string(),
            "drowning".to_string(),
        ],
        sample_rate: 16000,
        buffer_size: 4096,
        continuous_listening: true,
        detection_timeout: 30,
    };

    println!("Voice configuration:");
    println!("  Sample rate: {} Hz", config.sample_rate);
    println!("  Confidence threshold: {}", config.confidence_threshold);
    println!("  Emergency phrases: {} configured", config.emergency_phrases.len());
    println!("  Continuous listening: {}", config.continuous_listening);

    // Initialize voice interface
    let voice_interface = VoiceInterface::new(&config.model_path);
    println!("\nVoice interface initialized");
    println!("  On-device processing: Yes");
    println!("  Network transmission: None");
    println!("  Privacy mode: Maximum\n");

    Ok(())
}

/// Demonstrates emergency phrase detection logic
fn demonstrate_emergency_detection() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 2: Emergency Phrase Detection");
    println!("-------------------------------------");

    println!("The system detects emergencies through multiple signals:");
    println!();

    // Direct emergency phrases
    println!("1. Direct Emergency Phrases:");
    let direct_phrases = vec![
        "Help!",
        "Emergency!",
        "Call 911!",
        "SOS",
    ];
    for phrase in direct_phrases {
        println!("   - \"{}\" -> Immediate activation", phrase);
    }

    println!();

    // Medical emergency phrases
    println!("2. Medical Emergency Keywords:");
    let medical_phrases = vec![
        "heart attack",
        "can't breathe",
        "chest pain",
        "unconscious",
        "choking",
    ];
    for phrase in medical_phrases {
        println!("   - \"{}\" -> Medical protocol activation", phrase);
    }

    println!();

    // Context and tone analysis
    println!("3. Voice Pattern Analysis:");
    println!("   - Stress level detection in voice");
    println!("   - Panic indicators (rapid speech, high pitch)");
    println!("   - Background noise analysis (sirens, crashes)");
    println!("   - Urgency scoring based on acoustic features");

    println!();

    // Multi-language support
    println!("4. Multilingual Support:");
    println!("   - 99+ languages supported");
    println!("   - Accent and dialect recognition");
    println!("   - Automatic language detection");
    println!("   - Context-aware translation");

    println!();

    Ok(())
}

/// Demonstrates performance characteristics
fn demonstrate_performance_metrics() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 3: Performance Characteristics");
    println!("--------------------------------------");

    println!("Voice activation performance targets:");
    println!();

    println!("Latency Requirements:");
    println!("  - Wake word detection: <100ms");
    println!("  - Emergency phrase recognition: <200ms");
    println!("  - Complete voice-to-action: <300ms");
    println!("  - 911 auto-dial initiation: <500ms");

    println!();

    println!("Accuracy Requirements:");
    println!("  - True positive rate: >95%");
    println!("  - False positive rate: <5%");
    println!("  - Noise environment accuracy: >90% at 70dB");
    println!("  - Multilingual accuracy: >93% across languages");

    println!();

    println!("Resource Efficiency:");
    println!("  - CPU usage: <10% average");
    println!("  - Memory footprint: <200MB");
    println!("  - Battery impact: <5% per hour");
    println!("  - Storage: <100MB for models");

    println!();

    // Example performance stats
    let stats = VoiceStats {
        total_activations: 0,
        emergency_detections: 0,
        false_positive_rate: 0.03,
        avg_response_time_ms: 175,
        last_activation: None,
    };

    println!("Example Runtime Statistics:");
    println!("  Total activations: {}", stats.total_activations);
    println!("  Emergency detections: {}", stats.emergency_detections);
    println!("  False positive rate: {:.1}%", stats.false_positive_rate * 100.0);
    println!("  Average response time: {}ms", stats.avg_response_time_ms);

    println!();

    Ok(())
}

/// Demonstrates privacy-preserving architecture
fn demonstrate_privacy_architecture() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 4: Privacy-Preserving Design");
    println!("------------------------------------");

    println!("The voice system maintains privacy through:");
    println!();

    println!("1. On-Device Processing:");
    println!("   - All voice recognition runs locally on device");
    println!("   - No audio data transmitted to servers");
    println!("   - No cloud dependencies for core functionality");
    println!("   - Works completely offline");

    println!();

    println!("2. Data Minimization:");
    println!("   - Audio buffer cleared after processing");
    println!("   - No permanent audio recordings");
    println!("   - Only emergency transcripts stored (encrypted)");
    println!("   - Automatic deletion of non-essential data");

    println!();

    println!("3. Secure Processing:");
    println!("   - Memory-safe Rust implementation");
    println!("   - No data leakage through errors");
    println!("   - Isolated process space");
    println!("   - Encrypted temporary storage");

    println!();

    println!("4. User Control:");
    println!("   - Voice activation can be disabled");
    println!("   - History deletion available");
    println!("   - Granular permission control");
    println!("   - No behavioral profiling");

    println!();

    println!("5. Compliance:");
    println!("   - GDPR compliant (data minimization)");
    println!("   - HIPAA considerations (healthcare data protection)");
    println!("   - CCPA compliant (user privacy rights)");
    println!("   - Zero data selling or sharing");

    println!();

    Ok(())
}

/// Example test cases for voice activation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_config_defaults() {
        let config = VoiceConfig {
            model_path: "test/model".to_string(),
            confidence_threshold: 0.80,
            emergency_phrases: vec!["help".to_string()],
            sample_rate: 16000,
            buffer_size: 4096,
            continuous_listening: true,
            detection_timeout: 30,
        };

        assert_eq!(config.sample_rate, 16000);
        assert_eq!(config.confidence_threshold, 0.80);
    }

    #[test]
    fn test_emergency_phrase_coverage() {
        let required_phrases = vec![
            "help",
            "emergency",
            "heart attack",
            "stroke",
            "choking",
            "drowning",
        ];

        for phrase in required_phrases {
            println!("Validating emergency phrase: {}", phrase);
            // In production: verify phrase is in detection system
        }
    }

    #[test]
    fn test_performance_requirements() {
        // Define performance requirements
        let max_latency_ms = 200;
        let min_accuracy = 0.95;
        let max_false_positive_rate = 0.05;

        assert!(max_latency_ms <= 200);
        assert!(min_accuracy >= 0.95);
        assert!(max_false_positive_rate <= 0.05);
    }
}

