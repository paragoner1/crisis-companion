use crisis_companion::{
    adaptive_training::{AdaptiveTrainer, AdaptiveTrainingConfig, TrainingContext},
    types::EmergencyType,
};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🧠 Adaptive Training System Test");
    info!("================================");

    // Create adaptive trainer with default config
    let config = AdaptiveTrainingConfig::default();
    let mut trainer = AdaptiveTrainer::new(config);
    
    info!("✅ Adaptive trainer initialized");

    // Test 1: Simulate user training data
    info!("📊 Training User Model:");
    
    let user_id = "user_123";
    let audio_samples = vec![
        vec![0.1f32; 480], // Simulated audio data
        vec![0.2f32; 480],
        vec![0.15f32; 480],
    ];
    
    let transcriptions = vec![
        "drowning help".to_string(),
        "help drowning".to_string(),
        "drowning emergency".to_string(),
    ];

    // Train user model
    let adaptation_score = trainer.train_user_model(user_id, &audio_samples, &transcriptions).await?;
    info!("✅ User model trained with score: {:.2}", adaptation_score);

    // Test 2: Get personalized phrases
    info!("🎯 Personalized Emergency Phrases:");
    let personalized_phrases = trainer.get_personalized_phrases(user_id).await;
    
    for phrase in &personalized_phrases {
        info!("  • {} → {:?} (success rate: {:.1}%)", 
              phrase.phrase, phrase.emergency_type, phrase.success_rate * 100.0);
        
        info!("    Variations: {:?}", phrase.user_variations);
    }

    // Test 3: Update user profile with new data
    info!("🔄 Updating User Profile:");
    
    let new_audio_sample = vec![0.25f32; 480];
    let new_transcription = "choking help".to_string();
    let context = TrainingContext {
        environment: "noisy".to_string(),
        emotional_state: "stressed".to_string(),
        device_position: "pocket".to_string(),
    };

    trainer.update_user_profile(
        user_id,
        new_audio_sample,
        new_transcription,
        0.85,
        context,
    ).await?;
    
    info!("✅ User profile updated with new training data");

    // Test 4: Simulate continuous learning
    info!("📈 Continuous Learning Simulation:");
    
    for i in 1..=5 {
        let sample_audio = vec![0.1f32 + (i as f32 * 0.05); 480];
        let sample_transcription = format!("emergency help {}", i);
        let sample_context = TrainingContext {
            environment: "quiet".to_string(),
            emotional_state: "calm".to_string(),
            device_position: "handheld".to_string(),
        };

        trainer.update_user_profile(
            user_id,
            sample_audio,
            sample_transcription,
            0.9,
            sample_context,
        ).await?;
        
        info!("  • Added training sample {}", i);
    }

    info!("🎉 Adaptive training test completed!");
    info!("================================");

    Ok(())
} 