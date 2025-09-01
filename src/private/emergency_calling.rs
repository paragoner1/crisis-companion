
use serde::{Deserialize, Serialize};
use crate::private::first_responder_network::FirstResponderNetwork;
use crate::public::types::{EmergencyType, Location};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyContact {
    pub name: String,
    pub phone_number: String,
    pub relationship: String,
    pub notification_enabled: bool,
    pub last_notified: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    pub address: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyCall {
    pub emergency_type: String,
    pub location: LocationData,
    pub caller_info: CallerInfo,
    pub emergency_contacts: Vec<EmergencyContact>,
    pub context_flags: Vec<String>,
    pub timestamp: u64,
    pub call_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallerInfo {
    pub name: String,
    pub phone_number: String,
    pub emergency_contacts: Vec<EmergencyContact>,
    pub medical_info: Option<MedicalInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalInfo {
    pub allergies: Vec<String>,
    pub medications: Vec<String>,
    pub conditions: Vec<String>,
    pub blood_type: Option<String>,
}

pub struct EmergencyCaller {
    contacts: Vec<EmergencyContact>,
    location_service: LocationService,
    call_history: Vec<EmergencyCall>,
    first_responder_network: Arc<FirstResponderNetwork>,
}

impl EmergencyCaller {
    pub fn new() -> Self {
        EmergencyCaller {
            contacts: Vec::new(),
            location_service: LocationService::new(),
            call_history: Vec::new(),
            first_responder_network: Arc::new(FirstResponderNetwork::new()),
        }
    }
    
    pub fn new_with_network(network: Arc<FirstResponderNetwork>) -> Self {
        EmergencyCaller {
            contacts: Vec::new(),
            location_service: LocationService::new(),
            call_history: Vec::new(),
            first_responder_network: network,
        }
    }
    
    pub fn add_emergency_contact(&mut self, contact: EmergencyContact) {
        self.contacts.push(contact);
    }
    
    pub fn remove_emergency_contact(&mut self, phone_number: &str) {
        self.contacts.retain(|contact| contact.phone_number != phone_number);
    }
    
    pub fn get_emergency_contacts(&self) -> &Vec<EmergencyContact> {
        &self.contacts
    }
    
    pub async fn call_911(&mut self, emergency_type: &str, context_flags: &[String]) -> Result<String, EmergencyCallError> {
        // Get current location
        let location = self.location_service.get_current_location().await?;
        
        // Create emergency call record
        let call = EmergencyCall {
            emergency_type: emergency_type.to_string(),
            location: location.clone(),
            caller_info: self.get_caller_info(),
            emergency_contacts: self.contacts.clone(),
            context_flags: context_flags.to_vec(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            call_id: self.generate_call_id(),
        };
        
        // Store call record
        self.call_history.push(call.clone());
        
        // Make the actual 911 call
        self.make_emergency_call(&call).await?;
        
        Ok(call.call_id)
    }
    
    async fn make_emergency_call(&self, call: &EmergencyCall) -> Result<(), EmergencyCallError> {
        // This would integrate with Android's emergency calling system
        // For now, we'll simulate the call process
        
        // 1. Check if device has emergency calling capability
        if !self.has_emergency_calling_capability() {
            return Err(EmergencyCallError::NoEmergencyCallingCapability);
        }
        
        // 2. Prepare emergency call data
        let call_data = self.prepare_emergency_call_data(call);
        
        // 3. Make the call with location data
        self.dial_911_with_location(&call_data).await?;
        
        // 4. Notify emergency contacts if appropriate
        if self.should_notify_contacts(call) {
            self.notify_emergency_contacts(call).await?;
        }
        
        // 5. Broadcast to first responder network
        self.broadcast_to_first_responders(call).await?;
        
        Ok(())
    }
    
    fn has_emergency_calling_capability(&self) -> bool {
        // Check if device supports emergency calling
        // This would check Android's emergency calling permissions
        true // For demo purposes
    }
    
    fn prepare_emergency_call_data(&self, call: &EmergencyCall) -> EmergencyCallData {
        EmergencyCallData {
            emergency_type: call.emergency_type.clone(),
            location: call.location.clone(),
            caller_name: call.caller_info.name.clone(),
            caller_phone: call.caller_info.phone_number.clone(),
            medical_info: call.caller_info.medical_info.clone(),
            context_flags: call.context_flags.clone(),
        }
    }
    
    async fn dial_911_with_location(&self, call_data: &EmergencyCallData) -> Result<(), EmergencyCallError> {
        // This would use Android's emergency calling API
        // For now, we'll simulate the process
        
        // 1. Format location data for emergency services
        let location_string = self.format_location_for_emergency(&call_data.location);
        
        // 2. Prepare emergency message
        let emergency_message = self.format_emergency_message(call_data);
        
        // 3. Initiate emergency call
        // In real implementation, this would use Android's emergency calling system
        println!("🚨 EMERGENCY CALL INITIATED");
        println!("Emergency Type: {}", call_data.emergency_type);
        println!("Location: {}", location_string);
        println!("Message: {}", emergency_message);
        
        Ok(())
    }
    
    fn format_location_for_emergency(&self, location: &LocationData) -> String {
        if let Some(address) = &location.address {
            format!("{} (GPS: {:.6}, {:.6})", address, location.latitude, location.longitude)
        } else {
            format!("GPS Coordinates: {:.6}, {:.6}", location.latitude, location.longitude)
        }
    }
    
    fn format_emergency_message(&self, call_data: &EmergencyCallData) -> String {
        let mut message = format!("Emergency: {} emergency", call_data.emergency_type);
        
        if !call_data.context_flags.is_empty() {
            message.push_str(&format!(" - Context: {}", call_data.context_flags.join(", ")));
        }
        
        if let Some(medical_info) = &call_data.medical_info {
            if !medical_info.allergies.is_empty() {
                message.push_str(&format!(" - Allergies: {}", medical_info.allergies.join(", ")));
            }
        }
        
        message
    }
    
    async fn notify_emergency_contacts(&self, call: &EmergencyCall) -> Result<(), EmergencyCallError> {
        for contact in &call.emergency_contacts {
            if contact.notification_enabled {
                self.send_emergency_notification(contact, call).await?;
            }
        }
        Ok(())
    }
    
    async fn send_emergency_notification(&self, contact: &EmergencyContact, call: &EmergencyCall) -> Result<(), EmergencyCallError> {
        // This would send SMS/notification to emergency contact
        let message = self.format_contact_notification(call);
        
        println!("📱 Sending emergency notification to {}: {}", contact.name, message);
        
        // In real implementation, this would use Android's SMS API
        Ok(())
    }
    
    fn format_contact_notification(&self, call: &EmergencyCall) -> String {
        let location = self.format_location_for_emergency(&call.location);
        format!(
            "EMERGENCY: {} emergency at {}. Call 911 immediately if needed.",
            call.emergency_type.replace("_", " "),
            location
        )
    }
    
    fn should_notify_contacts(&self, call: &EmergencyCall) -> bool {
        // Only notify contacts for certain emergency types or conditions
        matches!(call.emergency_type.as_str(), "silent_sos" | "crash_detection")
    }
    
    fn get_caller_info(&self) -> CallerInfo {
        // This would get caller info from app settings
        CallerInfo {
            name: "Emergency User".to_string(),
            phone_number: "911".to_string(), // This would be the actual phone number
            emergency_contacts: self.contacts.clone(),
            medical_info: None, // This would be populated from user settings
        }
    }
    
    fn generate_call_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("EMG_{}", timestamp)
    }
    
    pub fn get_call_history(&self) -> &Vec<EmergencyCall> {
        &self.call_history
    }
    
    async fn broadcast_to_first_responders(&self, call: &EmergencyCall) -> Result<(), EmergencyCallError> {
        // Convert emergency call to emergency type and location for first responder network
        let emergency_type = self.parse_emergency_type(&call.emergency_type);
        let location = Location {
            latitude: call.location.latitude,
            longitude: call.location.longitude,
            altitude: None,
            accuracy: Some(call.location.accuracy),
            timestamp: call.location.timestamp,
        };
        
        // Determine severity based on emergency type and context flags
        let severity = self.calculate_emergency_severity(&call.emergency_type, &call.context_flags);
        
        // Create description from context flags
        let description = if call.context_flags.is_empty() {
            format!("{} emergency requiring immediate assistance", call.emergency_type.replace("_", " "))
        } else {
            format!("{} emergency - Context: {}", 
                   call.emergency_type.replace("_", " "), 
                   call.context_flags.join(", "))
        };
        
        // Broadcast to first responder network
        match self.first_responder_network.broadcast_emergency(
            emergency_type,
            location,
            severity,
            description,
            call.call_id.clone(),
        ).await {
            Ok(broadcast_id) => {
                println!("📡 First responder broadcast sent: {}", broadcast_id);
                Ok(())
            }
            Err(e) => {
                eprintln!("⚠️ Failed to broadcast to first responders: {}", e);
                // Don't fail the entire emergency call if first responder broadcast fails
                Ok(())
            }
        }
    }
    
    fn parse_emergency_type(&self, emergency_str: &str) -> EmergencyType {
        match emergency_str.to_lowercase().as_str() {
            "heart_attack" | "cardiac_arrest" => EmergencyType::HeartAttack,
            "stroke" | "brain_attack" => EmergencyType::Stroke,
            "choking" | "airway_obstruction" => EmergencyType::Choking,
            "drowning" | "water_emergency" => EmergencyType::Drowning,
            "seizure" | "convulsions" => EmergencyType::Seizure,
            "allergic_reaction" | "anaphylaxis" => EmergencyType::AllergicReaction,
            "burns" | "fire_injury" => EmergencyType::SevereBurns,
            "bleeding" | "severe_bleeding" => EmergencyType::Bleeding,
            "fracture" | "broken_bone" => EmergencyType::Trauma,
            "poisoning" | "overdose" => EmergencyType::Poisoning,
            _ => EmergencyType::HeartAttack, // Default fallback
        }
    }
    
    fn calculate_emergency_severity(&self, emergency_type: &str, context_flags: &[String]) -> u8 {
        let mut severity = match emergency_type.to_lowercase().as_str() {
            "heart_attack" | "cardiac_arrest" | "stroke" => 9, // Critical
            "choking" | "drowning" => 8, // High
            "severe_bleeding" | "anaphylaxis" => 7, // High
            "seizure" | "burns" => 6, // Medium-High
            "fracture" | "poisoning" => 5, // Medium
            _ => 5, // Default medium
        };
        
        // Adjust based on context flags
        for flag in context_flags {
            match flag.to_lowercase().as_str() {
                "unconscious" | "not_breathing" | "no_pulse" => severity = 10,
                "severe" | "critical" | "life_threatening" => severity = severity.max(8),
                "moderate" | "serious" => severity = severity.max(6),
                "minor" | "stable" => severity = severity.min(4),
                _ => {}
            }
        }
        
        severity.min(10).max(1)
    }
    
    /// Get access to the first responder network
    pub fn get_first_responder_network(&self) -> Arc<FirstResponderNetwork> {
        self.first_responder_network.clone()
    }
}

#[derive(Debug, Clone)]
pub struct EmergencyCallData {
    pub emergency_type: String,
    pub location: LocationData,
    pub caller_name: String,
    pub caller_phone: String,
    pub medical_info: Option<MedicalInfo>,
    pub context_flags: Vec<String>,
}

pub struct LocationService {
    // This would integrate with Android's location services
}

impl LocationService {
    pub fn new() -> Self {
        LocationService {}
    }
    
    pub async fn get_current_location(&self) -> Result<LocationData, EmergencyCallError> {
        // This would use Android's location services
        // For demo purposes, return a mock location
        Ok(LocationData {
            latitude: 37.7749,
            longitude: -122.4194,
            accuracy: 10.0,
            address: Some("123 Main St, San Francisco, CA".to_string()),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EmergencyCallError {
    #[error("No emergency calling capability")]
    NoEmergencyCallingCapability,
    #[error("Location service unavailable")]
    LocationServiceUnavailable,
    #[error("Emergency call failed")]
    EmergencyCallFailed,
    #[error("Contact notification failed")]
    ContactNotificationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_emergency_caller_creation() {
        let caller = EmergencyCaller::new();
        assert_eq!(caller.get_emergency_contacts().len(), 0);
    }
    
    #[test]
    fn test_add_emergency_contact() {
        let mut caller = EmergencyCaller::new();
        let contact = EmergencyContact {
            name: "Mom".to_string(),
            phone_number: "555-1234".to_string(),
            relationship: "Mother".to_string(),
            notification_enabled: true,
            last_notified: None,
        };
        
        caller.add_emergency_contact(contact);
        assert_eq!(caller.get_emergency_contacts().len(), 1);
    }
    
    #[tokio::test]
    async fn test_location_service() {
        let location_service = LocationService::new();
        let location = location_service.get_current_location().await.unwrap();
        
        assert_eq!(location.latitude, 37.7749);
        assert_eq!(location.longitude, -122.4194);
        assert!(location.address.is_some());
    }
} 