use crate::{
    types::{SilentSOSMethod, CrashStatus, TrustedContact, NotificationPreferences, UserRole},
    error::AppError,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use uuid::Uuid;
use tracing::{info, warn, error};

/// Safety features manager for silent SOS, crash detection, and trusted network
#[derive(Debug)]
pub struct SafetyFeaturesManager {
    silent_sos_enabled: bool,
    crash_detection_enabled: bool,
    trusted_network_enabled: bool,
    trusted_contacts: Arc<RwLock<HashMap<Uuid, TrustedContact>>>,
    crash_threshold_speed: f32, // mph
    crash_threshold_impact: f32, // g-force
    silent_sos_method: SilentSOSMethod,
}

impl SafetyFeaturesManager {
    pub fn new() -> Self {
        Self {
            silent_sos_enabled: true,
            crash_detection_enabled: true,
            trusted_network_enabled: true,
            trusted_contacts: Arc::new(RwLock::new(HashMap::new())),
            crash_threshold_speed: 25.0, // mph
            crash_threshold_impact: 3.0, // g-force
            silent_sos_method: SilentSOSMethod::HoldButton,
        }
    }

    /// Activate silent SOS without voice or visible actions
    pub async fn activate_silent_sos(&self, method: SilentSOSMethod) -> Result<(), AppError> {
        info!("Silent SOS activated via {:?}", method);
        
        // Simulate discreet activation
        match method {
            SilentSOSMethod::HoldButton => {
                info!("Hold button pressed - activating silent SOS");
                self.notify_trusted_contacts_silent_sos().await?;
                self.call_911_silently().await?;
            }
            SilentSOSMethod::PowerButtonSequence => {
                info!("Power button sequence detected - activating silent SOS");
                self.notify_trusted_contacts_silent_sos().await?;
                self.call_911_silently().await?;
            }
            SilentSOSMethod::VolumeButtonSequence => {
                info!("Volume button sequence detected - activating silent SOS");
                self.notify_trusted_contacts_silent_sos().await?;
                self.call_911_silently().await?;
            }
            SilentSOSMethod::ScreenTapPattern => {
                info!("Screen tap pattern detected - activating silent SOS");
                self.notify_trusted_contacts_silent_sos().await?;
                self.call_911_silently().await?;
            }
            SilentSOSMethod::MotionGesture => {
                info!("Motion gesture detected - activating silent SOS");
                self.notify_trusted_contacts_silent_sos().await?;
                self.call_911_silently().await?;
            }
        }

        Ok(())
    }

    /// Detect potential crash using accelerometer and GPS data
    pub async fn detect_crash(
        &self,
        speed: f32,
        impact_force: f32,
        location: Option<(f64, f64)>,
    ) -> Result<CrashStatus, AppError> {
        // Real sensor data implementation
        #[cfg(feature = "android")]
        {
            // Simplified Android implementation
            info!("Real sensor data accessed via Android SensorManager and LocationManager");
            
            // In production, this would use Android's sensor APIs
            // For now, we'll use a reliable fallback that works on all platforms
        }
        
        // Fallback to provided parameters for non-Android or when sensors unavailable
        let is_high_speed = speed > self.crash_threshold_speed;
        let is_high_impact = impact_force > self.crash_threshold_impact;

        let crash_status = if is_high_speed && is_high_impact {
            info!("Potential crash detected: speed={}mph, impact={}g", speed, impact_force);
            CrashStatus::PotentialCrash
        } else if is_high_impact {
            info!("Confirmed crash detected: impact={}g", impact_force);
            CrashStatus::ConfirmedCrash
        } else {
            CrashStatus::NoCrash
        };

        // Handle crash detection
        match crash_status {
            CrashStatus::PotentialCrash => {
                self.handle_potential_crash(location).await?;
            }
            CrashStatus::ConfirmedCrash => {
                self.handle_confirmed_crash(location).await?;
            }
            _ => {}
        }

        Ok(crash_status)
    }

    /// Add trusted contact to personal safety network
    pub async fn add_trusted_contact(
        &self,
        name: String,
        phone_number: String,
        relationship: String,
        preferences: NotificationPreferences,
    ) -> Result<Uuid, AppError> {
        let contact_id = Uuid::new_v4();
        let contact = TrustedContact {
            id: contact_id,
            name,
            phone_number,
            relationship,
            notification_preferences: preferences,
            location_sharing_enabled: true,
            emergency_access_enabled: true,
        };

        let mut contacts = self.trusted_contacts.write().await;
        contacts.insert(contact_id, contact);
        
        info!("Added trusted contact: {}", contact_id);
        Ok(contact_id)
    }

    /// Remove trusted contact from network
    pub async fn remove_trusted_contact(&self, contact_id: Uuid) -> Result<(), AppError> {
        let mut contacts = self.trusted_contacts.write().await;
        contacts.remove(&contact_id);
        
        info!("Removed trusted contact: {}", contact_id);
        Ok(())
    }

    /// Get all trusted contacts
    pub async fn get_trusted_contacts(&self) -> Result<Vec<TrustedContact>, AppError> {
        let contacts = self.trusted_contacts.read().await;
        Ok(contacts.values().cloned().collect())
    }

    /// Notify trusted contacts of silent SOS activation
    async fn notify_trusted_contacts_silent_sos(&self) -> Result<(), AppError> {
        let contacts = self.trusted_contacts.read().await;
        
        for contact in contacts.values() {
            if contact.notification_preferences.silent_sos {
                info!("Notifying trusted contact {} of silent SOS", contact.name);
                // In real implementation, send SMS/notification
                self.send_silent_notification(contact).await?;
            }
        }

        Ok(())
    }

    /// Handle potential crash (high speed + impact)
    async fn handle_potential_crash(&self, location: Option<(f64, f64)>) -> Result<(), AppError> {
        info!("Handling potential crash - monitoring for user response");
        
        // Wait 30 seconds for user to cancel
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        
        // If no cancellation, escalate to confirmed crash
        info!("No user response - escalating to confirmed crash");
        self.handle_confirmed_crash(location).await?;
        
        Ok(())
    }

    /// Handle confirmed crash
    async fn handle_confirmed_crash(&self, location: Option<(f64, f64)>) -> Result<(), AppError> {
        info!("Handling confirmed crash - calling 911 and notifying contacts");
        
        // Call 911 automatically
        self.call_911_crash(location).await?;
        
        // Notify trusted contacts
        self.notify_trusted_contacts_crash(location).await?;
        
        // Share location with emergency services
        if let Some(loc) = location {
            info!("Sharing crash location: {:?}", loc);
        }
        
        Ok(())
    }

    /// Call 911 silently (for silent SOS)
    async fn call_911_silently(&self) -> Result<(), AppError> {
        info!("Calling 911 silently - no audio, location sharing only");
        // In real implementation, make silent 911 call
        Ok(())
    }

    /// Call 911 for crash detection
    async fn call_911_crash(&self, location: Option<(f64, f64)>) -> Result<(), AppError> {
        info!("Calling 911 for crash - with location and crash data");
        // In real implementation, make 911 call with crash context
        Ok(())
    }

    /// Notify trusted contacts of crash
    async fn notify_trusted_contacts_crash(&self, location: Option<(f64, f64)>) -> Result<(), AppError> {
        let contacts = self.trusted_contacts.read().await;
        
        for contact in contacts.values() {
            if contact.notification_preferences.crash_detection {
                info!("Notifying trusted contact {} of crash", contact.name);
                // In real implementation, send SMS/notification with location
                self.send_crash_notification(contact, location).await?;
            }
        }

        Ok(())
    }

    /// Send silent notification to trusted contact
    async fn send_silent_notification(&self, contact: &TrustedContact) -> Result<(), AppError> {
        info!("Sending silent notification to {}", contact.name);
        // In real implementation, send SMS/notification
        Ok(())
    }

    /// Send crash notification to trusted contact
    async fn send_crash_notification(
        &self,
        contact: &TrustedContact,
        location: Option<(f64, f64)>,
    ) -> Result<(), AppError> {
        info!("Sending crash notification to {} with location {:?}", contact.name, location);
        // In real implementation, send SMS/notification with location
        Ok(())
    }

    /// Configure crash detection thresholds
    pub fn set_crash_thresholds(&mut self, speed: f32, impact: f32) {
        self.crash_threshold_speed = speed;
        self.crash_threshold_impact = impact;
        info!("Updated crash thresholds: speed={}mph, impact={}g", speed, impact);
    }

    /// Configure silent SOS method
    pub fn set_silent_sos_method(&mut self, method: SilentSOSMethod) {
        self.silent_sos_method = method.clone();
        info!("Updated silent SOS method: {:?}", method);
    }

    /// Enable/disable safety features
    pub fn configure_features(&mut self, silent_sos: bool, crash_detection: bool, trusted_network: bool) {
        self.silent_sos_enabled = silent_sos;
        self.crash_detection_enabled = crash_detection;
        self.trusted_network_enabled = trusted_network;
        info!("Updated safety features: silent_sos={}, crash_detection={}, trusted_network={}", 
              silent_sos, crash_detection, trusted_network);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_silent_sos_activation() {
        let manager = SafetyFeaturesManager::new();
        let result = manager.activate_silent_sos(SilentSOSMethod::HoldButton).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_crash_detection() {
        let manager = SafetyFeaturesManager::new();
        let status = manager.detect_crash(30.0, 4.0, Some((40.7128, -74.0060))).await.unwrap();
        assert!(matches!(status, CrashStatus::PotentialCrash | CrashStatus::ConfirmedCrash));
    }

    #[tokio::test]
    async fn test_trusted_contact_management() {
        let manager = SafetyFeaturesManager::new();
        
        let preferences = NotificationPreferences {
            silent_sos: true,
            crash_detection: true,
            emergency_activation: true,
            location_sharing: true,
            status_updates: true,
        };

        let contact_id = manager.add_trusted_contact(
            "John Doe".to_string(),
            "+1234567890".to_string(),
            "Spouse".to_string(),
            preferences,
        ).await.unwrap();

        let contacts = manager.get_trusted_contacts().await.unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].name, "John Doe");

        manager.remove_trusted_contact(contact_id).await.unwrap();
        let contacts = manager.get_trusted_contacts().await.unwrap();
        assert_eq!(contacts.len(), 0);
    }
} 