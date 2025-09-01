//! Private Implementation Modules
//! 
//! This module contains the proprietary implementation details for Solana SOS.
//! These modules are protected by .gitignore and contain sensitive algorithms.

#[cfg(feature = "voice")]
pub mod voice_recognition;
#[cfg(feature = "voice")]
pub mod whisper_engine;
pub mod audio_engine;
pub mod emergency_logic;
pub mod gamification_engine;
pub mod safety_engine;
pub mod blockchain_engine;
pub mod database_engine;
pub mod ui_engine;
pub mod context_analysis; pub mod noise_filter;
pub mod emergency_database;
pub mod emergency_calling;
pub mod first_responder_network;
pub mod solana_blockchain;
pub mod location_tracking;
pub mod token_system;
pub mod gamification;
pub mod safety_features;
pub mod crash_detection;
pub mod coordination;
#[cfg(feature = "voice")]
pub mod medical_ai;
pub mod role_detection;
pub mod adaptive_training;
pub mod training_ai_interface;
pub mod training_interface;
pub mod blockchain_interface;
pub mod database_interface;
pub mod gamification_interface;
pub mod safety_interface;
pub mod ui_interface;
pub mod viral_sharing;