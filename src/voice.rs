use crate::error::AppResult;
use crate::{AppError, types::*, config::VoiceConfig};
use crate::noise_filter::{NoiseFilter, NoiseFilterType};
use chrono::Utc;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tracing::{info, warn, error};
// use vosk::{Model, Recognizer}; // Temporarily disabled for demo
use std::collections::HashMap;

use rand::Rng;
use sha2::{Sha256, Digest};

/// Voice trigger system for detecting emergency phrases
#[derive(Debug)]
pub struct VoiceTrigger {
    config: VoiceConfig,
    noise_filter: NoiseFilter,
    // model: Option<Model>, // Temporarily disabled
    // recognizer: Option<Recognizer>, // Temporarily disabled
    is_listening: Arc<Mutex<bool>>,
    trigger_sender: mpsc::Sender<VoiceTriggerResult>,
    emergency_phrase_map: HashMap<String, EmergencyType>,
}

impl VoiceTrigger {
    pub fn new(config: &VoiceConfig) -> AppResult<Self> {
        info!("Initializing voice trigger system (demo mode)");
        let mut emergency_phrase_map = HashMap::new();
        // Populate emergency_phrase_map
        emergency_phrase_map.insert("drowning help".to_string(), EmergencyType::Drowning);
        emergency_phrase_map.insert("heart attack help".to_string(), EmergencyType::HeartAttack);
        emergency_phrase_map.insert("stroke help".to_string(), EmergencyType::Stroke);
        emergency_phrase_map.insert("choking help".to_string(), EmergencyType::Choking);
        emergency_phrase_map.insert("bleeding help".to_string(), EmergencyType::Bleeding);
        emergency_phrase_map.insert("unconscious help".to_string(), EmergencyType::Unconscious);
        emergency_phrase_map.insert("seizure help".to_string(), EmergencyType::Seizure);
        emergency_phrase_map.insert("poisoning help".to_string(), EmergencyType::Poisoning);
        emergency_phrase_map.insert("overdose help".to_string(), EmergencyType::Poisoning);
        emergency_phrase_map.insert("burns help".to_string(), EmergencyType::SevereBurns);
        emergency_phrase_map.insert("diabetic emergency help".to_string(), EmergencyType::DiabeticEmergency);
        emergency_phrase_map.insert("diabetes help".to_string(), EmergencyType::DiabeticEmergency);
        emergency_phrase_map.insert("allergic reaction help".to_string(), EmergencyType::AllergicReaction);
        emergency_phrase_map.insert("trauma help".to_string(), EmergencyType::Trauma);
        
        // Direct action phrases - more specific to avoid ambiguity
        emergency_phrase_map.insert("cpr".to_string(), EmergencyType::Unconscious);
        emergency_phrase_map.insert("cardiopulmonary".to_string(), EmergencyType::Unconscious);
        emergency_phrase_map.insert("heimlich".to_string(), EmergencyType::Choking);
        emergency_phrase_map.insert("aed".to_string(), EmergencyType::Unconscious);
        emergency_phrase_map.insert("defibrillator".to_string(), EmergencyType::Unconscious);
        emergency_phrase_map.insert("tourniquet".to_string(), EmergencyType::Bleeding);
        emergency_phrase_map.insert("epipen".to_string(), EmergencyType::AllergicReaction);
        emergency_phrase_map.insert("rescue breathing".to_string(), EmergencyType::Unconscious);
        emergency_phrase_map.insert("first aid".to_string(), EmergencyType::Trauma);
        
        // Specific shock types to avoid ambiguity
        emergency_phrase_map.insert("medical shock".to_string(), EmergencyType::AllergicReaction);
        emergency_phrase_map.insert("anaphylactic shock".to_string(), EmergencyType::AllergicReaction);
        emergency_phrase_map.insert("electric shock".to_string(), EmergencyType::Unconscious);
        emergency_phrase_map.insert("electrocution".to_string(), EmergencyType::Unconscious);
        
        // Specific pressure types
        emergency_phrase_map.insert("direct pressure".to_string(), EmergencyType::Bleeding);
        emergency_phrase_map.insert("blood pressure".to_string(), EmergencyType::Trauma);
        emergency_phrase_map.insert("stop bleeding".to_string(), EmergencyType::Bleeding);
        
        // Specific breathing issues
        emergency_phrase_map.insert("not breathing".to_string(), EmergencyType::Unconscious);
        emergency_phrase_map.insert("no breathing".to_string(), EmergencyType::Unconscious);
        emergency_phrase_map.insert("breathing problems".to_string(), EmergencyType::Choking);
        emergency_phrase_map.insert("difficulty breathing".to_string(), EmergencyType::Choking);

        let (trigger_sender, _trigger_receiver) = mpsc::channel(100);
        let is_listening = Arc::new(Mutex::new(false));

        info!("Voice trigger system initialized (demo mode - no Vosk model)");
        Ok(Self {
            config: config.clone(),
            noise_filter: NoiseFilter::new(NoiseFilterType::RNNoise, 0.8), // Initialize with RNNoise filter
            // model: None, // Temporarily disabled
            // recognizer: None, // Temporarily disabled
            is_listening,
            trigger_sender,
            emergency_phrase_map,
        })
    }

    pub async fn start_listening(&mut self) -> AppResult<()> {
        info!("Starting voice listening (demo mode)");
        let mut is_listening = self.is_listening.lock().unwrap();
        if *is_listening {
            warn!("Voice listening already active");
            return Ok(());
        }
        *is_listening = true;
        drop(is_listening);

        let config = self.config.clone();
        let emergency_phrase_map = self.emergency_phrase_map.clone();
        let trigger_sender = self.trigger_sender.clone();
        let is_listening = self.is_listening.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::listening_loop(
                config,
                emergency_phrase_map,
                trigger_sender,
                is_listening,
            ).await {
                error!("Voice listening loop error: {}", e);
            }
        });
        info!("Voice listening started successfully (demo mode)");
        Ok(())
    }

    pub async fn stop_listening(&self) -> AppResult<()> {
        info!("Stopping voice listening");
        let mut is_listening = self.is_listening.lock().unwrap();
        *is_listening = false;
        info!("Voice listening stopped");
        Ok(())
    }

    async fn listening_loop(
        config: VoiceConfig,
        emergency_phrase_map: HashMap<String, EmergencyType>,
        trigger_sender: mpsc::Sender<VoiceTriggerResult>,
        is_listening: Arc<Mutex<bool>>,
    ) -> AppResult<()> {
        info!("Voice listening loop started (demo mode)");
        let audio_buffer = vec![0i16; config.buffer_size];

        // Create noise filter for processing
        let noise_filter = NoiseFilter::new(NoiseFilterType::RNNoise, 0.8);

        // For demo purposes, we'll simulate voice detection
        loop {
            {
                let listening = is_listening.lock().unwrap();
                if !*listening {
                    break;
                }
            }

            // Simulate audio processing
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            // Simulate raw audio data (in real app, this would come from microphone)
            let simulated_raw_audio = vec![0.1f32; 480]; // 480 samples for RNNoise
            
            // Apply noise filtering (this is where the magic happens!)
            match noise_filter.process_audio(&simulated_raw_audio).await {
                Ok(filtered_audio) => {
                    info!("Audio processed with noise filtering ({} samples)", filtered_audio.len());
                    
                    // In real implementation, this filtered audio would go to voice recognition
                    // For demo, we'll simulate detection on the filtered audio
                }
                Err(e) => {
                    warn!("Noise filtering failed: {}", e);
                    // Continue without filtering
                }
            }

            // Check for emergency phrases (simulated for demo)
            for (phrase, emergency_type) in &emergency_phrase_map {
                // In real implementation, this would process actual audio
                // For demo, we'll simulate detection
                if Self::simulate_phrase_detection(phrase).await {
                    info!("Emergency phrase detected: {}", phrase);

                    let trigger = VoiceTriggerResult {
                        emergency_type: emergency_type.clone(),
                        confidence: 0.95,
                        timestamp: Utc::now(),
                        audio_hash: Self::generate_audio_hash(&audio_buffer),
                    };

                    if let Err(e) = trigger_sender.send(trigger).await {
                        error!("Failed to send voice trigger: {}", e);
                    }

                    // Wait before allowing another detection
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
        info!("Voice listening loop ended");
        Ok(())
    }

    async fn simulate_phrase_detection(_phrase: &str) -> bool {
        // Simulate voice detection for demo
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // For demo, randomly trigger (1% chance)
        let mut rng = rand::thread_rng();
        rng.gen_bool(0.01)
    }

    pub async fn test_trigger(&self, phrase: &str) -> AppResult<Option<VoiceTriggerResult>> {
        info!("Testing voice trigger with phrase: {}", phrase);

        let emergency_type = self.emergency_phrase_map.get(phrase)
            .cloned()
            .ok_or_else(|| AppError::Voice(format!("Unknown phrase: {}", phrase)))?;

        let trigger = VoiceTriggerResult {
            emergency_type: emergency_type.clone(),
            confidence: 0.95,
            timestamp: Utc::now(),
            audio_hash: Self::generate_audio_hash(&vec![0; 1024]),
        };

        info!("Test trigger created for: {:?}", emergency_type);
        Ok(Some(trigger))
    }

    fn generate_audio_hash(audio_data: &[i16]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(audio_data.iter().map(|&x| x.to_le_bytes()).flatten().collect::<Vec<u8>>());
        format!("{:x}", hasher.finalize())
    }

    pub fn is_listening(&self) -> bool {
        *self.is_listening.lock().unwrap()
    }

    pub fn get_emergency_phrases(&self) -> Vec<String> {
        self.emergency_phrase_map.keys().cloned().collect()
    }
}

impl Drop for VoiceTrigger {
    fn drop(&mut self) {
        if let Ok(mut is_listening) = self.is_listening.lock() {
            *is_listening = false;
        }
        info!("Voice trigger system dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::VoiceConfig;
    use crate::types::VoiceTriggerResult;

    #[tokio::test]
    async fn test_voice_trigger_creation() {
        let config = VoiceConfig::default();
        let voice_trigger = VoiceTrigger::new(&config);
        assert!(voice_trigger.is_ok());
    }

    #[tokio::test]
    async fn test_emergency_phrase_detection() {
        let config = VoiceConfig::default();
        let voice_trigger = VoiceTrigger::new(&config).unwrap();

        // Test drowning detection
        let result = voice_trigger.test_trigger("drowning help").await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().emergency_type, EmergencyType::Drowning);

        // Test non-emergency phrase
        let result = voice_trigger.test_trigger("hello world").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_confidence_threshold() {
        let config = VoiceConfig::default();
        let voice_trigger = VoiceTrigger::new(&config).unwrap();
        assert_eq!(voice_trigger.config.confidence_threshold, 0.8); // Access config directly
    }

    #[tokio::test]
    async fn test_listening_status() {
        let config = VoiceConfig::default();
        let mut voice_trigger = VoiceTrigger::new(&config).unwrap();

        // Initially not listening
        assert!(!voice_trigger.is_listening());

        // Start listening
        voice_trigger.start_listening().await.unwrap();
        assert!(voice_trigger.is_listening());

        // Stop listening
        voice_trigger.stop_listening().await.unwrap();
        assert!(!voice_trigger.is_listening());
    }
} 