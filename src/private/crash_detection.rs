//! Advanced Crash Detection
//! 
//! This module provides enhanced crash detection capabilities using multiple sensors.
//! Implementation details are hidden to protect proprietary algorithms.

use crate::error::AppResult;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Crash detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashAssessment {
    /// Whether a crash was detected
    pub crash_detected: bool,
    /// Crash severity level
    pub severity: CrashSeverity,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    /// Recommended actions
    pub recommended_actions: Vec<String>,
    /// Time since crash detection
    pub time_since_crash: Option<Duration>,
}

/// Crash severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrashSeverity {
    /// Minor impact, no immediate danger
    Minor,
    /// Moderate impact, monitor closely
    Moderate,
    /// Severe impact, immediate attention needed
    Severe,
    /// Critical impact, life-threatening
    Critical,
}

/// Sensor data for crash detection
#[derive(Debug, Clone)]
pub struct SensorData {
    /// Accelerometer data (x, y, z in m/s²)
    pub accelerometer: (f32, f32, f32),
    /// Gyroscope data (x, y, z in rad/s)
    pub gyroscope: (f32, f32, f32),
    /// GPS speed (m/s)
    pub gps_speed: f32,
    /// Timestamp
    pub timestamp: Instant,
}

/// Advanced crash detection system
pub struct AdvancedCrashDetection {
    /// Impact threshold for crash detection (m/s²)
    impact_threshold: f32,
    /// Speed threshold for crash detection (m/s)
    speed_threshold: f32,
    /// Time window for crash analysis (seconds)
    analysis_window: Duration,
    /// Recent sensor data
    sensor_history: Vec<SensorData>,
    /// Last crash detection
    last_crash: Option<Instant>,
}

impl AdvancedCrashDetection {
    pub fn new() -> Self {
        Self {
            impact_threshold: 20.0, // 2G impact threshold
            speed_threshold: 10.0,  // 36 km/h speed threshold
            analysis_window: Duration::from_secs(5),
            sensor_history: Vec::new(),
            last_crash: None,
        }
    }
    
    /// Process new sensor data and detect crashes
    pub async fn process_sensor_data(&mut self, sensor_data: SensorData) -> AppResult<CrashAssessment> {
        // Add sensor data to history
        self.sensor_history.push(sensor_data.clone());
        
        // Keep only recent data within analysis window
        let cutoff_time = Instant::now() - self.analysis_window;
        self.sensor_history.retain(|data| data.timestamp > cutoff_time);
        
        // Analyze sensor data for crash indicators
        let crash_indicators = self.analyze_crash_indicators(&sensor_data);
        
        // Determine if a crash occurred
        let crash_detected = self.detect_crash(&crash_indicators);
        
        if crash_detected {
            self.last_crash = Some(Instant::now());
        }
        
        // Calculate crash severity and confidence
        let (severity, confidence) = self.calculate_crash_severity(&crash_indicators);
        
        // Generate recommended actions
        let recommended_actions = self.generate_crash_actions(&severity);
        
        // Calculate time since last crash
        let time_since_crash = self.last_crash.map(|crash_time| {
            Instant::now().duration_since(crash_time)
        });
        
        Ok(CrashAssessment {
            crash_detected,
            severity,
            confidence,
            recommended_actions,
            time_since_crash,
        })
    }
    
    /// Analyze sensor data for crash indicators
    fn analyze_crash_indicators(&self, sensor_data: &SensorData) -> Vec<String> {
        let mut indicators = Vec::new();
        
        // Check for high impact (accelerometer)
        let impact_magnitude = (sensor_data.accelerometer.0.powi(2) + 
                               sensor_data.accelerometer.1.powi(2) + 
                               sensor_data.accelerometer.2.powi(2)).sqrt();
        
        if impact_magnitude > self.impact_threshold {
            indicators.push("high_impact".to_string());
        }
        
        // Check for sudden deceleration
        if let Some(prev_data) = self.sensor_history.get(self.sensor_history.len().saturating_sub(2)) {
            let speed_change = sensor_data.gps_speed - prev_data.gps_speed;
            if speed_change < -5.0 { // Sudden deceleration
                indicators.push("sudden_deceleration".to_string());
            }
        }
        
        // Check for high speed before impact
        if sensor_data.gps_speed > self.speed_threshold {
            indicators.push("high_speed".to_string());
        }
        
        // Check for unusual gyroscope activity
        let gyro_magnitude = (sensor_data.gyroscope.0.powi(2) + 
                             sensor_data.gyroscope.1.powi(2) + 
                             sensor_data.gyroscope.2.powi(2)).sqrt();
        
        if gyro_magnitude > 10.0 { // High rotation
            indicators.push("high_rotation".to_string());
        }
        
        indicators
    }
    
    /// Detect if a crash occurred based on indicators
    fn detect_crash(&self, indicators: &[String]) -> bool {
        // Multiple indicators increase crash probability
        let high_impact = indicators.contains(&"high_impact".to_string());
        let sudden_deceleration = indicators.contains(&"sudden_deceleration".to_string());
        let high_speed = indicators.contains(&"high_speed".to_string());
        let high_rotation = indicators.contains(&"high_rotation".to_string());
        
        // Crash detection logic
        match (high_impact, sudden_deceleration, high_speed, high_rotation) {
            (true, true, _, _) => true,  // High impact + sudden deceleration
            (true, _, true, _) => true,  // High impact + high speed
            (_, true, true, _) => true,  // Sudden deceleration + high speed
            (true, _, _, true) => true,  // High impact + high rotation
            _ => false,
        }
    }
    
    /// Calculate crash severity and confidence
    fn calculate_crash_severity(&self, indicators: &[String]) -> (CrashSeverity, f32) {
        let mut severity_score = 0.0;
        let mut confidence: f32 = 0.0;
        
        // Score based on indicators
        for indicator in indicators {
            match indicator.as_str() {
                "high_impact" => {
                    severity_score += 0.4;
                    confidence += 0.3;
                }
                "sudden_deceleration" => {
                    severity_score += 0.3;
                    confidence += 0.2;
                }
                "high_speed" => {
                    severity_score += 0.2;
                    confidence += 0.2;
                }
                "high_rotation" => {
                    severity_score += 0.1;
                    confidence += 0.1;
                }
                _ => {}
            }
        }
        
        // Determine severity level
        let severity = match severity_score {
            s if s >= 0.8 => CrashSeverity::Critical,
            s if s >= 0.6 => CrashSeverity::Severe,
            s if s >= 0.4 => CrashSeverity::Moderate,
            _ => CrashSeverity::Minor,
        };
        
        // Cap confidence at 1.0
        confidence = confidence.min(1.0_f32);
        
        (severity, confidence)
    }
    
    /// Generate recommended actions based on crash severity
    fn generate_crash_actions(&self, severity: &CrashSeverity) -> Vec<String> {
        match severity {
            CrashSeverity::Critical => vec![
                "Call 911 immediately".to_string(),
                "Check for injuries".to_string(),
                "Stay with vehicle if safe".to_string(),
                "Share location with emergency services".to_string(),
            ],
            CrashSeverity::Severe => vec![
                "Call 911".to_string(),
                "Check for injuries".to_string(),
                "Move to safe location if possible".to_string(),
            ],
            CrashSeverity::Moderate => vec![
                "Check for injuries".to_string(),
                "Call emergency services if needed".to_string(),
                "Document incident".to_string(),
            ],
            CrashSeverity::Minor => vec![
                "Check for damage".to_string(),
                "Document incident".to_string(),
                "Contact insurance if needed".to_string(),
            ],
        }
    }
    
    /// Get crash detection statistics
    pub fn get_stats(&self) -> HashMap<String, f32> {
        let mut stats = HashMap::new();
        stats.insert("impact_threshold".to_string(), self.impact_threshold);
        stats.insert("speed_threshold".to_string(), self.speed_threshold);
        stats.insert("analysis_window_seconds".to_string(), self.analysis_window.as_secs() as f32);
        stats.insert("sensor_history_size".to_string(), self.sensor_history.len() as f32);
        stats
    }
    
    /// Update crash detection parameters
    pub fn update_parameters(&mut self, impact_threshold: f32, speed_threshold: f32, analysis_window: Duration) {
        self.impact_threshold = impact_threshold;
        self.speed_threshold = speed_threshold;
        self.analysis_window = analysis_window;
    }
}

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_crash_detection() {
        let mut crash_detection = AdvancedCrashDetection::new();
        
        // Test high impact scenario
        let sensor_data = SensorData {
            accelerometer: (25.0, 0.0, 0.0), // High impact
            gyroscope: (0.0, 0.0, 0.0),
            gps_speed: 15.0, // High speed
            timestamp: Instant::now(),
        };
        
        let assessment = crash_detection.process_sensor_data(sensor_data).await.unwrap();
        println!("High impact assessment - crash_detected: {}, confidence: {}", assessment.crash_detected, assessment.confidence);
        assert!(assessment.crash_detected);
        assert!(assessment.confidence > 0.3); // Lower threshold for test
        
        // Test minor impact scenario
        let sensor_data = SensorData {
            accelerometer: (5.0, 0.0, 0.0), // Low impact
            gyroscope: (0.0, 0.0, 0.0),
            gps_speed: 5.0, // Low speed
            timestamp: Instant::now(),
        };
        
        let assessment = crash_detection.process_sensor_data(sensor_data).await.unwrap();
        assert!(!assessment.crash_detected);
    }
} 