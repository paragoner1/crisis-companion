//! Voice Recognition Interface
//! 
//! This module provides the public interface for voice recognition functionality.
//! Implementation details are hidden to protect proprietary algorithms.

use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Voice recognition trigger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceTrigger {
    /// Wake word to activate emergency mode
    pub wake_word: String,
    /// Confidence threshold for wake word detection
    pub confidence_threshold: f32,
    /// Emergency phrases to detect
    pub emergency_phrases: Vec<String>,
    /// Direct action phrases for trained responders
    pub direct_actions: Vec<String>,
}

/// Voice recognition configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    /// Sample rate for audio processing
    pub sample_rate: u32,
    /// Audio buffer size
    pub buffer_size: usize,
    /// Enable noise filtering
    pub enable_noise_filtering: bool,
    /// Enable adaptive model training
    pub enable_adaptive_training: bool,
    /// Voice trigger configuration
    pub trigger: VoiceTrigger,
}

/// Voice recognition statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceStats {
    /// Total voice activations
    pub total_activations: u64,
    /// Successful emergency detections
    pub emergency_detections: u64,
    /// False positive rate
    pub false_positive_rate: f32,
    /// Average response time in milliseconds
    pub avg_response_time_ms: u64,
    /// Last activation timestamp
    pub last_activation: Option<chrono::DateTime<chrono::Utc>>,
}

/// Voice interface for emergency recognition
pub struct VoiceInterface {
    config: VoiceConfig,
    stats: Arc<RwLock<VoiceStats>>,
    model_path: String,
}

impl VoiceInterface {
    /// Create a new voice interface
    pub fn new(model_path: &str) -> Self {
        let config = VoiceConfig {
            sample_rate: 16000,
            buffer_size: 4096,
            enable_noise_filtering: true,
            enable_adaptive_training: true,
            trigger: VoiceTrigger {
                wake_word: "hey sos".to_string(),
                confidence_threshold: 0.8,
                emergency_phrases: vec![
                    "drowning".to_string(),
                    "heart attack".to_string(),
                    "choking".to_string(),
                    "bleeding".to_string(),
                    "unconscious".to_string(),
                    "stroke".to_string(),
                    "seizure".to_string(),
                    "poisoning".to_string(),
                    "burn".to_string(),
                    "diabetic".to_string(),
                    "allergic".to_string(),
                    "trauma".to_string(),
                ],
                direct_actions: vec![
                    "cpr".to_string(),
                    "heimlich".to_string(),
                    "aed".to_string(),
                    "tourniquet".to_string(),
                    "epipen".to_string(),
                    "rescue breathing".to_string(),
                    "first aid".to_string(),
                    "fast test".to_string(),
                    "poison control".to_string(),
                    "cool burn".to_string(),
                    "medical alert".to_string(),
                ],
            },
        };

        let stats = Arc::new(RwLock::new(VoiceStats {
            total_activations: 0,
            emergency_detections: 0,
            false_positive_rate: 0.0,
            avg_response_time_ms: 0,
            last_activation: None,
        }));

        Self {
            config,
            stats,
            model_path: model_path.to_string(),
        }
    }

    /// Initialize voice recognition
    pub async fn initialize(&mut self) -> AppResult<()> {
        // Initialize voice recognition system
        tracing::info!("Voice interface initialized with model: {}", self.model_path);
        Ok(())
    }

    /// Process audio input and return recognized text
    pub async fn process_audio(&mut self, audio_data: &[u8]) -> AppResult<String> {
        let start_time = std::time::Instant::now();
        
        // Apply noise filtering if enabled
        let processed_audio = if self.config.enable_noise_filtering {
            self.apply_noise_filtering(audio_data)?
        } else {
            audio_data.to_vec()
        };

        // Process with voice recognition
        let recognized_text = self.recognize_speech(&processed_audio)?;
        
        // Update statistics
        let response_time = start_time.elapsed().as_millis() as u64;
        self.update_stats(response_time).await;

        Ok(recognized_text)
    }

    /// Detect wake word in audio
    pub async fn detect_wake_word(&mut self, audio_data: &[u8]) -> AppResult<bool> {
        let recognized_text = self.process_audio(audio_data).await?;
        let detected = recognized_text
            .to_lowercase()
            .contains(&self.config.trigger.wake_word);
        
        if detected {
            tracing::info!("Wake word detected: {}", self.config.trigger.wake_word);
        }
        
        Ok(detected)
    }

    /// Detect emergency phrase in audio
    pub async fn detect_emergency_phrase(&mut self, audio_data: &[u8]) -> AppResult<Option<String>> {
        let recognized_text = self.process_audio(audio_data).await?;
        
        for phrase in &self.config.trigger.emergency_phrases {
            if recognized_text.to_lowercase().contains(phrase) {
                tracing::info!("Emergency phrase detected: {}", phrase);
                return Ok(Some(phrase.clone()));
            }
        }
        
        Ok(None)
    }

    /// Detect direct action phrase
    pub async fn detect_direct_action(&mut self, audio_data: &[u8]) -> AppResult<Option<String>> {
        let recognized_text = self.process_audio(audio_data).await?;
        
        for action in &self.config.trigger.direct_actions {
            if recognized_text.to_lowercase().contains(action) {
                tracing::info!("Direct action detected: {}", action);
                return Ok(Some(action.clone()));
            }
        }
        
        Ok(None)
    }

    /// Apply noise filtering to audio
    fn apply_noise_filtering(&self, audio_data: &[u8]) -> AppResult<Vec<u8>> {
        // Simplified noise filtering - in production would use RNNoise
        Ok(audio_data.to_vec())
    }

    /// Recognize speech from audio
    fn recognize_speech(&self, audio_data: &[u8]) -> AppResult<String> {
        // Simplified speech recognition - in production would use Vosk
        // For demo purposes, return a placeholder based on audio length
        if audio_data.len() > 1000 {
            Ok("hey sos drowning emergency".to_string())
        } else {
            Ok("hey sos".to_string())
        }
    }

    /// Update voice recognition statistics
    async fn update_stats(&self, response_time: u64) {
        let mut stats = self.stats.write().await;
        stats.total_activations += 1;
        stats.avg_response_time_ms = 
            (stats.avg_response_time_ms + response_time) / 2;
        stats.last_activation = Some(chrono::Utc::now());
    }

    /// Get voice recognition statistics
    pub async fn get_stats(&self) -> VoiceStats {
        self.stats.read().await.clone()
    }

    /// Adapt voice model with user data
    pub async fn adapt_model(&mut self, user_audio_data: &[u8]) -> AppResult<()> {
        if !self.config.enable_adaptive_training {
            return Ok(());
        }

        // Simplified model adaptation
        tracing::info!("Adapting voice model with user data ({} bytes)", user_audio_data.len());
        Ok(())
    }

    /// Get voice configuration
    pub fn get_config(&self) -> &VoiceConfig {
        &self.config
    }

    /// Update voice configuration
    pub fn update_config(&mut self, config: VoiceConfig) {
        self.config = config;
    }
} 