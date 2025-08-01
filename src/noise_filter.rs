// Noise cancellation and filtering system

use tracing::info;
use nnnoiseless::DenoiseState;

#[derive(Debug)]
pub struct NoiseFilter {
    pub filter_type: NoiseFilterType,
    pub sensitivity: f32,
}

#[derive(Debug, Clone)]
pub enum NoiseFilterType {
    None, // No filtering (for testing)
    RNNoise, // Best for real-time, low latency
    SpectralGate, // Simple but effective
    DeepFilter, // AI-powered, higher accuracy
}

impl NoiseFilter {
    pub fn new(filter_type: NoiseFilterType, sensitivity: f32) -> Self {
        Self {
            filter_type,
            sensitivity,
        }
    }

    pub async fn process_audio(&self, raw_audio: &[f32]) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        match self.filter_type {
            NoiseFilterType::None => {
                info!("No noise filtering applied");
                Ok(raw_audio.to_vec())
            }
            NoiseFilterType::RNNoise => {
                info!("Processing audio with RNNoise filter");
                
                // Initialize RNNoise denoiser
                let mut denoiser = DenoiseState::new();
                
                // Process audio in frames (RNNoise expects 480-sample frames)
                let frame_size = 480;
                let mut filtered_audio = Vec::new();
                
                for chunk in raw_audio.chunks(frame_size) {
                    let mut frame = [0.0f32; 480];
                    for (i, &sample) in chunk.iter().enumerate() {
                        if i < 480 { frame[i] = sample; }
                    }
                    
                    let mut output_frame = [0.0f32; 480];
                    let _vad_probability = denoiser.process_frame(&mut output_frame, &frame);
                    filtered_audio.extend_from_slice(&output_frame);
                }
                
                Ok(filtered_audio)
            }
            NoiseFilterType::SpectralGate => {
                info!("Processing audio with Spectral Gate filter");
                // Simple spectral gate implementation
                let mut filtered_audio = Vec::new();
                let threshold = self.sensitivity * 0.1; // Adjustable threshold
                
                for &sample in raw_audio {
                    if sample.abs() > threshold {
                        filtered_audio.push(sample);
                    } else {
                        filtered_audio.push(0.0);
                    }
                }
                
                Ok(filtered_audio)
            }
            NoiseFilterType::DeepFilter => {
                info!("Processing audio with Deep Filter");
                // Placeholder for future AI-powered filtering
                Ok(raw_audio.to_vec())
            }
        }
    }
} 