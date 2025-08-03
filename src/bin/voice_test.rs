use solana_sos::public::voice_interface::VoiceTrigger;

#[tokio::main]
async fn main() {
    // Construct VoiceTrigger directly
    let voice_trigger = VoiceTrigger {
        wake_word: "hey sos".to_string(),
        confidence_threshold: 0.8,
        emergency_phrases: vec![
            "drowning".to_string(),
            "heart attack".to_string(),
        ],
        direct_actions: vec!["cpr".to_string()],
    };

    println!("VoiceTrigger: {:?}", voice_trigger);
    // Demo: print the wake word and emergency phrases
    println!("Wake word: {}", voice_trigger.wake_word);
    println!("Emergency phrases: {:?}", voice_trigger.emergency_phrases);
    println!("Direct actions: {:?}", voice_trigger.direct_actions);
    // No static methods to call anymore
} 