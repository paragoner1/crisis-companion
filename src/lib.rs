//! Solana SOS - Voice-Activated Emergency Response System
//! 
//! This library provides the core functionality for Solana SOS, a voice-activated
//! emergency response app that works offline and online. The system combines
//! context-aware guidance with SOS Hero gamification to transform ordinary people
//! into life-saving heroes.
//! 
//! ## Key Features
//! 
//! - **Voice-Activated**: Responds to emergency phrases in under 100ms
//! - **Offline-First**: Works in storms, remote areas, or power failures
//! - **Context-Aware**: Understands emergency stages and provides appropriate guidance
//! - **SOS Hero Gamification**: 10 hero levels with XP, achievements, and token rewards
//! - **Safety Features**: Silent SOS, crash detection, and trusted network
//! - **Hybrid Architecture**: Seamless offline/online operation
//! 
//! ## Architecture
//! 
//! The system is built with a hybrid offline/online architecture:
//! 
//! - **Offline Mode**: Voice recognition, context analysis, and emergency guidance
//! - **Online Mode**: AI enhancement, real-time updates, and cloud synchronization
//! - **Hybrid Mode**: Seamless handoff between modes with context preservation
//! 
//! ## Emergency Types Supported
//! 
//! Solana SOS handles 12 critical life-threatening emergencies:
//! 
//! - Drowning, Heart Attack, Stroke
//! - Choking, Bleeding, Unconscious
//! - Seizure, Poisoning, Severe Burns
//! - Diabetic Emergency, Allergic Reaction, Trauma
//! 
//! ## Direct Actions
//! 
//! For trained responders, the system recognizes 11 direct action phrases:
//! 
//! - CPR, Heimlich, AED, Tourniquet, EpiPen
//! - Rescue Breathing, First Aid, FAST Test
//! - Poison Control, Cool Burn, Medical Alert
//! 
//! ## SOS Hero Gamification
//! 
//! The SOS Hero system features:
//! 
//! - 10 hero levels from Novice to Legend
//! - XP rewards for learning and interventions
//! - BONK and SKR token rewards
//! - 20+ achievements to unlock
//! - Viral growth through trusted networks
//! 
//! ## Safety Features
//! 
//! Advanced safety features include:
//! 
//! - **Silent SOS**: Discreet activation for dangerous situations
//! - **Crash Detection**: Automatic 911 calling based on sensor data
//! - **Trusted Network**: Personal network of emergency contacts
//! 
//! ## Technology Stack
//! 
//! - **Language**: Rust for reliability and performance
//! - **Voice Recognition**: Vosk with RNNoise noise filtering
//! - **Database**: SQLite for local storage
//! - **Blockchain**: Solana for tamper-proof records
//! - **Platform**: Android JNI for native integration
//! 
//! ## Getting Started
//! 
//! ```rust
//! use solana_sos::app::SolanaSOSApp;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut app = SolanaSOSApp::new().await?;
//!     app.initialize().await?;
//!     app.run().await?;
//!     Ok(())
//! }
//! ```
//! 
//! ## Demo Commands
//! 
//! ```bash
//! # Complete app walkthrough
//! cargo run --bin complete_walkthrough
//! 
//! # Gamification demo
//! cargo run --bin gamification_demo
//! 
//! # Basic demo
//! cargo run --bin demo_test
//! ```

// Public modules (visible to users)
pub mod public {
    pub mod voice_interface;
    pub mod audio_interface;
    pub mod emergency_interface;
    pub mod types;
}

// Private modules (implementation details - hidden by .gitignore)
#[cfg(feature = "private")]
pub mod private;

// JNI Bridge for Android integration
pub mod jni_bridge;

// Core modules (always available)
pub mod app;
pub mod config;
pub mod error;

// Re-export main types for easy access
pub use app::SolanaSOSApp;
pub use error::AppResult;
pub use public::types::{EmergencyType, EmergencyStage, DirectAction, ConnectivityMode, GuidanceMode};

// Re-export interface types
pub use public::voice_interface::{VoiceTrigger, VoiceConfig, VoiceStats};
pub use public::audio_interface::{AudioProcessor, AudioConfig, AudioStats};
pub use public::emergency_interface::{EmergencySystem, EmergencyConfig, EmergencyStats};
