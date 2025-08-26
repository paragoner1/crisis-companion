//! Safety Features Module
//! 
//! Implements advanced safety features including silent SOS activation,
//! crash detection, trusted network alerts, and location tracking.

use crate::error::AppResult;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SafetyFeatures {
    silent_sos_enabled: bool,
    crash_detection_enabled: bool,
    trusted_network_enabled: bool,
    location_tracking_enabled: bool,
    crash_threshold: CrashThreshold,
    silent_sos_activation: Option<SilentSOSActivation>,
    crash_detection: Option<CrashDetection>,
    trusted_network: TrustedNetwork,
    location_tracker: LocationTracker,
}

#[derive(Debug, Clone)]
pub struct CrashThreshold {
    pub speed_threshold_mph: f64, // 25 mph
    pub acceleration_threshold_g: f64, // 3g force
    pub time_window_ms: u32, // 100ms window
}

#[derive(Debug, Clone)]
pub struct SilentSOSActivation {
    pub activated_at: DateTime<Utc>,
    pub activation_method: SilentSOSMethod,
    pub location: Option<Location>,
    pub contacts_notified: Vec<String>,
    pub status: SilentSOSStatus,
}

#[derive(Debug, Clone)]
pub struct CrashDetection {
    pub detected_at: DateTime<Utc>,
    pub crash_severity: CrashSeverity,
    pub location: Option<Location>,
    pub sensor_data: SensorData,
    pub auto_911_called: bool,
    pub cancellation_window_expires: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TrustedNetwork {
    pub contacts: Vec<TrustedContact>,
    pub network_id: String,
    pub last_alert: Option<DateTime<Utc>>,
    pub alert_cooldown_minutes: u32,
}

#[derive(Debug, Clone)]
pub struct LocationTracker {
    pub current_location: Option<Location>,
    pub location_history: Vec<Location>,
    pub tracking_enabled: bool,
    pub update_interval_seconds: u32,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    pub timestamp: DateTime<Utc>,
    pub altitude: Option<f64>,
    pub speed: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct TrustedContact {
    pub name: String,
    pub phone_number: String,
    pub relationship: String,
    pub notification_enabled: bool,
    pub last_notified: Option<DateTime<Utc>>,
    pub response_time_seconds: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct SensorData {
    pub acceleration_x: f64,
    pub acceleration_y: f64,
    pub acceleration_z: f64,
    pub speed: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SilentSOSMethod {
    ButtonHold,
    PowerButtonSequence,
    VoiceCommand,
    Gesture,
    AppTrigger,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SilentSOSStatus {
    Activated,
    ContactsNotified,
    EmergencyServicesCalled,
    Cancelled,
    Resolved,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CrashSeverity {
    Minor,
    Moderate,
    Severe,
    Critical,
}

impl SafetyFeatures {
    pub fn new() -> Self {
        SafetyFeatures {
            silent_sos_enabled: true,
            crash_detection_enabled: true,
            trusted_network_enabled: true,
            location_tracking_enabled: true,
            crash_threshold: CrashThreshold {
                speed_threshold_mph: 25.0,
                acceleration_threshold_g: 3.0,
                time_window_ms: 100,
            },
            silent_sos_activation: None,
            crash_detection: None,
            trusted_network: TrustedNetwork::new(),
            location_tracker: LocationTracker::new(),
        }
    }
    
    /// Activate silent SOS
    pub fn activate_silent_sos(&mut self, method: SilentSOSMethod) -> Result<(), String> {
        if !self.silent_sos_enabled {
            return Err("Silent SOS is disabled".to_string());
        }
        
        let location = self.location_tracker.get_current_location();
        
        let activation = SilentSOSActivation {
            activated_at: Utc::now(),
            activation_method: method,
            location,
            contacts_notified: Vec::new(),
            status: SilentSOSStatus::Activated,
        };
        
        self.silent_sos_activation = Some(activation);
        
        // Notify trusted network
        self.notify_trusted_network("Silent SOS activated", &location)
            .map_err(|e| format!("Failed to notify trusted network: {}", e))?;
        
        Ok(())
    }
    
    /// Detect crash based on sensor data
    pub fn detect_crash(&mut self, sensor_data: SensorData) -> Result<bool, String> {
        if !self.crash_detection_enabled {
            return Ok(false);
        }
        
        let speed_mph = sensor_data.speed * 2.237; // Convert m/s to mph
        let acceleration_magnitude = (sensor_data.acceleration_x.powi(2) + 
                                    sensor_data.acceleration_y.powi(2) + 
                                    sensor_data.acceleration_z.powi(2)).sqrt();
        let acceleration_g = acceleration_magnitude / 9.81; // Convert to g-force
        
        let is_crash = speed_mph >= self.crash_threshold.speed_threshold_mph && 
                      acceleration_g >= self.crash_threshold.acceleration_threshold_g;
        
        if is_crash {
            let location = self.location_tracker.get_current_location();
            let severity = self.determine_crash_severity(&sensor_data);
            
            let crash_detection = CrashDetection {
                detected_at: Utc::now(),
                crash_severity: severity,
                location,
                sensor_data,
                auto_911_called: false,
                cancellation_window_expires: Utc::now() + chrono::Duration::seconds(30),
            };
            
            self.crash_detection = Some(crash_detection);
            
            // Auto-call 911 for severe crashes
            if severity == CrashSeverity::Severe || severity == CrashSeverity::Critical {
                self.auto_call_911_crash().map_err(|e| format!("Failed to auto-call 911: {}", e))?;
            }
            
            // Notify trusted network
            self.notify_trusted_network("Crash detected", &location)
                .map_err(|e| format!("Failed to notify trusted network: {}", e))?;
        }
        
        Ok(is_crash)
    }
    
    /// Add trusted contact
    pub fn add_trusted_contact(&mut self, contact: TrustedContact) {
        self.trusted_network.add_contact(contact);
    }
    
    /// Get trusted contacts
    pub fn get_trusted_contacts(&self) -> &Vec<TrustedContact> {
        self.trusted_network.get_contacts()
    }
    
    /// Update location tracking
    pub fn update_location(&mut self, latitude: f64, longitude: f64, accuracy: f64) {
        self.location_tracker.update_location(latitude, longitude, accuracy);
    }
    
    /// Get current location
    pub fn get_current_location(&self) -> Option<Location> {
        self.location_tracker.get_current_location()
    }
    
    /// Cancel silent SOS
    pub fn cancel_silent_sos(&mut self) -> Result<(), String> {
        if let Some(activation) = &mut self.silent_sos_activation {
            activation.status = SilentSOSStatus::Cancelled;
            Ok(())
        } else {
            Err("No silent SOS activation to cancel".to_string())
        }
    }
    
    /// Cancel crash detection auto-911
    pub fn cancel_crash_auto_911(&mut self) -> Result<(), String> {
        if let Some(crash) = &mut self.crash_detection {
            if Utc::now() <= crash.cancellation_window_expires {
                crash.auto_911_called = false;
                Ok(())
            } else {
                Err("Cancellation window has expired".to_string())
            }
        } else {
            Err("No crash detection to cancel".to_string())
        }
    }
    
    /// Enable/disable safety features
    pub fn set_silent_sos_enabled(&mut self, enabled: bool) {
        self.silent_sos_enabled = enabled;
    }
    
    pub fn set_crash_detection_enabled(&mut self, enabled: bool) {
        self.crash_detection_enabled = enabled;
    }
    
    pub fn set_trusted_network_enabled(&mut self, enabled: bool) {
        self.trusted_network_enabled = enabled;
    }
    
    pub fn set_location_tracking_enabled(&mut self, enabled: bool) {
        self.location_tracking_enabled = enabled;
    }
    
    /// Get safety feature status
    pub fn get_safety_status(&self) -> SafetyStatus {
        SafetyStatus {
            silent_sos_enabled: self.silent_sos_enabled,
            crash_detection_enabled: self.crash_detection_enabled,
            trusted_network_enabled: self.trusted_network_enabled,
            location_tracking_enabled: self.location_tracking_enabled,
            silent_sos_active: self.silent_sos_activation.is_some(),
            crash_detected: self.crash_detection.is_some(),
            trusted_contacts_count: self.trusted_network.contacts.len(),
        }
    }
    
    fn determine_crash_severity(&self, sensor_data: &SensorData) -> CrashSeverity {
        let acceleration_magnitude = (sensor_data.acceleration_x.powi(2) + 
                                    sensor_data.acceleration_y.powi(2) + 
                                    sensor_data.acceleration_z.powi(2)).sqrt();
        let acceleration_g = acceleration_magnitude / 9.81;
        
        match acceleration_g {
            g if g >= 10.0 => CrashSeverity::Critical,
            g if g >= 7.0 => CrashSeverity::Severe,
            g if g >= 5.0 => CrashSeverity::Moderate,
            _ => CrashSeverity::Minor,
        }
    }
    
    fn auto_call_911_crash(&mut self) -> Result<(), String> {
        if let Some(crash) = &mut self.crash_detection {
            crash.auto_911_called = true;
            // In real implementation, this would make the actual 911 call
            Ok(())
        } else {
            Err("No crash detection to call 911 for".to_string())
        }
    }
    
    fn notify_trusted_network(&self, message: &str, location: &Option<Location>) -> Result<(), String> {
        if !self.trusted_network_enabled {
            return Ok(());
        }
        
        // Check cooldown
        if let Some(last_alert) = self.trusted_network.last_alert {
            let cooldown_duration = chrono::Duration::minutes(self.trusted_network.alert_cooldown_minutes as i64);
            if Utc::now() - last_alert < cooldown_duration {
                return Ok(());
            }
        }
        
        // In real implementation, this would send notifications to all trusted contacts
        // For now, we'll simulate the notification
        
        Ok(())
    }
}

impl TrustedNetwork {
    fn new() -> Self {
        TrustedNetwork {
            contacts: Vec::new(),
            network_id: uuid::Uuid::new_v4().to_string(),
            last_alert: None,
            alert_cooldown_minutes: 5,
        }
    }
    
    fn add_contact(&mut self, contact: TrustedContact) {
        self.contacts.push(contact);
    }
    
    fn get_contacts(&self) -> &Vec<TrustedContact> {
        &self.contacts
    }
}

impl LocationTracker {
    fn new() -> Self {
        LocationTracker {
            current_location: None,
            location_history: Vec::new(),
            tracking_enabled: true,
            update_interval_seconds: 30,
        }
    }
    
    fn update_location(&mut self, latitude: f64, longitude: f64, accuracy: f64) {
        let location = Location {
            latitude,
            longitude,
            accuracy,
            timestamp: Utc::now(),
            altitude: None,
            speed: None,
        };
        
        self.current_location = Some(location.clone());
        self.location_history.push(location);
        
        // Keep only last 100 locations
        if self.location_history.len() > 100 {
            self.location_history.remove(0);
        }
    }
    
    fn get_current_location(&self) -> Option<Location> {
        self.current_location.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SafetyStatus {
    pub silent_sos_enabled: bool,
    pub crash_detection_enabled: bool,
    pub trusted_network_enabled: bool,
    pub location_tracking_enabled: bool,
    pub silent_sos_active: bool,
    pub crash_detected: bool,
    pub trusted_contacts_count: usize,
}
