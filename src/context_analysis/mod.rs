//! Context-Aware Emergency Guidance for Solana SOS
//! 
//! This module analyzes the current stage of an emergency and provides
//! context-appropriate guidance, skipping irrelevant steps.
//! 
//! # Features
//! - Real-time emergency stage detection
//! - Context-aware guidance skipping
//! - Situation-specific instructions
//! - Dynamic guidance adaptation

pub mod stage_detection;
pub mod guidance_generation;
pub mod context_analysis;

pub use stage_detection::*;
pub use guidance_generation::*;
pub use context_analysis::*; 