use thiserror::Error;

/// Main error type for the Crisis Companion application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("Voice recognition error: {0}")]
    Voice(String),
    
    #[error("Audio processing error: {0}")]
    Audio(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Bluetooth error: {0}")]
    Bluetooth(String),
    
    #[error("Solana blockchain error: {0}")]
    Blockchain(String),
    
    #[error("Emergency handling error: {0}")]
    Emergency(String),
    
    #[error("UI error: {0}")]
    UI(String),
    
    #[error("JNI error: {0}")]
    JNI(#[from] jni::errors::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
    
    #[error("Role detection confirmation timeout")]
    ConfirmationTimeout,
    
    #[error("Invalid voice response for role detection")]
    InvalidVoiceResponse,
    
    #[error("Role detection failed: {0}")]
    RoleDetection(String),
    
    #[error("AI inference failed: {0}")]
    AIInference(String),
    
    #[error("Sensor fusion failed: {0}")]
    SensorFusion(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::Unknown(err.to_string())
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

/// Result type for the Crisis Companion application
pub type AppResult<T> = Result<T, AppError>; 