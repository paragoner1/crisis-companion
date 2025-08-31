//! Location Tracking Module
//! 
//! Implements real-time GPS tracking, location services,
//! and location-based emergency response features.


use chrono::{DateTime, Utc};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct LocationTracking {
    current_location: Option<Location>,
    location_history: VecDeque<Location>,
    tracking_enabled: bool,
    update_interval_seconds: u32,
    accuracy_threshold: f64,
    max_history_size: usize,
    last_update: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
    pub altitude: Option<f64>,
    pub speed: Option<f64>,
    pub heading: Option<f64>,
    pub timestamp: DateTime<Utc>,
    pub source: LocationSource,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LocationSource {
    GPS,
    Network,
    Manual,
    Estimated,
}

#[derive(Debug, Clone)]
pub struct LocationUpdate {
    pub location: Location,
    pub is_significant: bool,
    pub distance_from_last: Option<f64>,
    pub time_since_last: Option<chrono::Duration>,
}

impl LocationTracking {
    pub fn new() -> Self {
        LocationTracking {
            current_location: None,
            location_history: VecDeque::new(),
            tracking_enabled: true,
            update_interval_seconds: 30,
            accuracy_threshold: 10.0, // 10 meters
            max_history_size: 100,
            last_update: None,
        }
    }
    
    /// Update current location
    pub fn update_location(&mut self, latitude: f64, longitude: f64, accuracy: f64) -> LocationUpdate {
        let new_location = Location {
            latitude,
            longitude,
            accuracy,
            altitude: None,
            speed: None,
            heading: None,
            timestamp: Utc::now(),
            source: LocationSource::GPS,
        };
        
        let update = self.process_location_update(new_location);
        
        if update.is_significant {
            self.current_location = Some(update.location.clone());
            self.location_history.push_back(update.location.clone());
            
            // Keep history size manageable
            if self.location_history.len() > self.max_history_size {
                self.location_history.pop_front();
            }
            
            self.last_update = Some(update.location.timestamp);
        }
        
        update
    }
    
    /// Update location with full data
    pub fn update_location_full(&mut self, location: Location) -> LocationUpdate {
        let update = self.process_location_update(location);
        
        if update.is_significant {
            self.current_location = Some(update.location.clone());
            self.location_history.push_back(update.location.clone());
            
            if self.location_history.len() > self.max_history_size {
                self.location_history.pop_front();
            }
            
            self.last_update = Some(update.location.timestamp);
        }
        
        update
    }
    
    /// Get current location
    pub fn get_current_location(&self) -> Option<Location> {
        self.current_location.clone()
    }
    
    /// Get location history
    pub fn get_location_history(&self) -> Vec<Location> {
        self.location_history.iter().cloned().collect()
    }
    
    /// Get recent locations (last N)
    pub fn get_recent_locations(&self, count: usize) -> Vec<Location> {
        self.location_history.iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }
    
    /// Calculate distance between two locations
    pub fn calculate_distance(&self, loc1: &Location, loc2: &Location) -> f64 {
        self.haversine_distance(loc1.latitude, loc1.longitude, loc2.latitude, loc2.longitude)
    }
    
    /// Check if location is within accuracy threshold
    pub fn is_location_accurate(&self, location: &Location) -> bool {
        location.accuracy <= self.accuracy_threshold
    }
    
    /// Get estimated speed based on recent locations
    pub fn get_estimated_speed(&self) -> Option<f64> {
        if self.location_history.len() < 2 {
            return None;
        }
        
        let recent_locations: Vec<_> = self.location_history.iter()
            .rev()
            .take(5)
            .collect();
        
        if recent_locations.len() < 2 {
            return None;
        }
        
        let mut total_distance = 0.0;
        let mut total_time = 0.0;
        
        for i in 0..recent_locations.len() - 1 {
            let loc1 = recent_locations[i];
            let loc2 = recent_locations[i + 1];
            
            let distance = self.calculate_distance(loc1, loc2);
            let time_diff = (loc2.timestamp - loc1.timestamp).num_seconds() as f64;
            
            if time_diff > 0.0 {
                total_distance += distance;
                total_time += time_diff;
            }
        }
        
        if total_time > 0.0 {
            Some(total_distance / total_time) // meters per second
        } else {
            None
        }
    }
    
    /// Get location trend (direction of movement)
    pub fn get_location_trend(&self) -> Option<LocationTrend> {
        if self.location_history.len() < 3 {
            return None;
        }
        
        let recent_locations: Vec<_> = self.location_history.iter()
            .rev()
            .take(5)
            .collect();
        
        if recent_locations.len() < 3 {
            return None;
        }
        
        let first = recent_locations.last().unwrap();
        let last = recent_locations.first().unwrap();
        
        let distance = self.calculate_distance(first, last);
        let time_diff = (last.timestamp - first.timestamp).num_seconds() as f64;
        
        if distance < 10.0 || time_diff < 60.0 {
            return Some(LocationTrend::Stationary);
        }
        
        let speed = distance / time_diff; // meters per second
        
        if speed < 1.0 {
            Some(LocationTrend::Walking)
        } else if speed < 5.0 {
            Some(LocationTrend::Running)
        } else if speed < 15.0 {
            Some(LocationTrend::Cycling)
        } else {
            Some(LocationTrend::Driving)
        }
    }
    
    /// Enable/disable location tracking
    pub fn set_tracking_enabled(&mut self, enabled: bool) {
        self.tracking_enabled = enabled;
    }
    
    /// Set update interval
    pub fn set_update_interval(&mut self, seconds: u32) {
        self.update_interval_seconds = seconds;
    }
    
    /// Set accuracy threshold
    pub fn set_accuracy_threshold(&mut self, meters: f64) {
        self.accuracy_threshold = meters;
    }
    
    /// Get tracking status
    pub fn get_tracking_status(&self) -> TrackingStatus {
        TrackingStatus {
            enabled: self.tracking_enabled,
            current_location: self.current_location.is_some(),
            location_count: self.location_history.len(),
            last_update: self.last_update,
            update_interval: self.update_interval_seconds,
            accuracy_threshold: self.accuracy_threshold,
        }
    }
    
    /// Clear location history
    pub fn clear_history(&mut self) {
        self.location_history.clear();
    }
    
    /// Process location update and determine if it's significant
    fn process_location_update(&self, new_location: Location) -> LocationUpdate {
        let mut is_significant = true;
        let mut distance_from_last = None;
        let mut time_since_last = None;
        
        if let Some(current) = &self.current_location {
            let distance = self.calculate_distance(current, &new_location);
            let time_diff = new_location.timestamp - current.timestamp;
            
            distance_from_last = Some(distance);
            time_since_last = Some(time_diff);
            
            // Consider update significant if:
            // 1. Distance is more than 10 meters, OR
            // 2. Time difference is more than 5 minutes, OR
            // 3. Accuracy improved significantly
            is_significant = distance > 10.0 || 
                           time_diff.num_minutes() > 5 ||
                           new_location.accuracy < current.accuracy * 0.5;
        }
        
        LocationUpdate {
            location: new_location,
            is_significant,
            distance_from_last,
            time_since_last,
        }
    }
    
    /// Calculate Haversine distance between two points
    fn haversine_distance(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        let r = 6371000.0; // Earth's radius in meters
        
        let lat1_rad = lat1.to_radians();
        let lat2_rad = lat2.to_radians();
        let delta_lat = (lat2 - lat1).to_radians();
        let delta_lon = (lon2 - lon1).to_radians();
        
        let a = (delta_lat / 2.0).sin() * (delta_lat / 2.0).sin() +
                lat1_rad.cos() * lat2_rad.cos() *
                (delta_lon / 2.0).sin() * (delta_lon / 2.0).sin();
        
        let c = 2.0 * a.sqrt().asin();
        
        r * c
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LocationTrend {
    Stationary,
    Walking,
    Running,
    Cycling,
    Driving,
}

#[derive(Debug, Clone)]
pub struct TrackingStatus {
    pub enabled: bool,
    pub current_location: bool,
    pub location_count: usize,
    pub last_update: Option<DateTime<Utc>>,
    pub update_interval: u32,
    pub accuracy_threshold: f64,
}
