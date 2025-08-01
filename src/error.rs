use thiserror::Error;

/// Main error type for the Crisis Companion application
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Emergency error: {0}")]
    Emergency(String),
    
    #[error("Voice recognition error: {0}")]
    Voice(String),
    
    #[error("Audio error: {0}")]
    Audio(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Gamification error: {0}")]
    Gamification(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Blockchain error: {0}")]
    Blockchain(String),
    
    #[error("UI error: {0}")]
    UI(String),
    
    #[error("Bluetooth error: {0}")]
    Bluetooth(String),
    
    #[error("JNI error: {0}")]
    JNI(String),
    
    #[error("Confirmation timeout")]
    ConfirmationTimeout,
    
    #[error("Invalid voice response")]
    InvalidVoiceResponse,
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::Unknown(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<jni::errors::Error> for AppError {
    fn from(err: jni::errors::Error) -> Self {
        AppError::JNI(err.to_string())
    }
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::Config(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

/// Result type for the Crisis Companion application
pub type AppResult<T> = Result<T, AppError>; 