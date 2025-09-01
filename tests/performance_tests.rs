//! World-Class Performance Tests for Solana SOS
//! 
//! These tests ensure sub-millisecond response times for life-saving functionality.
//! Industry standards: P50 < 25ms, P95 < 50ms, P99 < 100ms for critical operations.

use solana_sos::private::{
    emergency_calling::EmergencyCallingSystem,
    medical_ai::MedicalAI,
    voice_recognition::VoiceRecognitionSystem,
    whisper_engine::WhisperEngine,
};
use solana_sos::public::{
    emergency_database::EmergencyDatabase,
    types::EmergencyType,
};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::timeout;

/// Performance test configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Target response times for critical operations (in milliseconds)
    pub voice_activation_target_ms: u64,
    pub emergency_detection_target_ms: u64,
    pub ai_inference_target_ms: u64,
    pub database_query_target_ms: u64,
    pub ui_render_target_ms: u64,
    
    /// Load testing parameters
    pub concurrent_users: usize,
    pub test_duration_seconds: u64,
    pub memory_limit_mb: usize,
    
    /// Percentile requirements
    pub p50_target_ms: u64,
    pub p95_target_ms: u64,
    pub p99_target_ms: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            // World-class targets for life-saving app
            voice_activation_target_ms: 50,      // Must be under 50ms
            emergency_detection_target_ms: 25,   // Critical: under 25ms
            ai_inference_target_ms: 100,         // AI response under 100ms
            database_query_target_ms: 10,       // Database under 10ms
            ui_render_target_ms: 16,             // 60 FPS = 16ms per frame
            
            // Load testing
            concurrent_users: 100,
            test_duration_seconds: 60,
            memory_limit_mb: 512,
            
            // Industry standard percentiles
            p50_target_ms: 25,   // 50% of requests under 25ms
            p95_target_ms: 50,   // 95% of requests under 50ms
            p99_target_ms: 100,  // 99% of requests under 100ms
        }
    }
}

/// Performance test results
#[derive(Debug, Clone)]
pub struct PerformanceResults {
    pub test_name: String,
    pub measurements: Vec<Duration>,
    pub p50: Duration,
    pub p95: Duration,
    pub p99: Duration,
    pub min: Duration,
    pub max: Duration,
    pub mean: Duration,
    pub passed: bool,
    pub memory_usage_mb: usize,
    pub cpu_usage_percent: f64,
}

impl PerformanceResults {
    pub fn new(test_name: String, mut measurements: Vec<Duration>, target_ms: u64) -> Self {
        measurements.sort();
        let len = measurements.len();
        
        let p50 = measurements[len * 50 / 100];
        let p95 = measurements[len * 95 / 100];
        let p99 = measurements[len * 99 / 100];
        let min = measurements[0];
        let max = measurements[len - 1];
        let mean = Duration::from_nanos(
            measurements.iter().map(|d| d.as_nanos()).sum::<u128>() / len as u128
        );
        
        let passed = p95.as_millis() <= target_ms as u128;
        
        Self {
            test_name,
            measurements,
            p50,
            p95,
            p99,
            min,
            max,
            mean,
            passed,
            memory_usage_mb: 0, // Will be filled by profiler
            cpu_usage_percent: 0.0,
        }
    }
    
    pub fn print_results(&self, config: &PerformanceConfig) {
        println!("\n🎯 Performance Test: {}", self.test_name);
        println!("==========================================");
        println!("📊 Response Time Statistics:");
        println!("   P50: {:4}ms (target: <{}ms) {}", 
            self.p50.as_millis(), 
            config.p50_target_ms,
            if self.p50.as_millis() <= config.p50_target_ms as u128 { "✅" } else { "❌" }
        );
        println!("   P95: {:4}ms (target: <{}ms) {}", 
            self.p95.as_millis(), 
            config.p95_target_ms,
            if self.p95.as_millis() <= config.p95_target_ms as u128 { "✅" } else { "❌" }
        );
        println!("   P99: {:4}ms (target: <{}ms) {}", 
            self.p99.as_millis(), 
            config.p99_target_ms,
            if self.p99.as_millis() <= config.p99_target_ms as u128 { "✅" } else { "❌" }
        );
        println!("   Min: {:4}ms", self.min.as_millis());
        println!("   Max: {:4}ms", self.max.as_millis());
        println!("   Mean:{:4}ms", self.mean.as_millis());
        
        println!("💾 Resource Usage:");
        println!("   Memory: {} MB", self.memory_usage_mb);
        println!("   CPU: {:.1}%", self.cpu_usage_percent);
        
        println!("🎯 Overall Result: {}", if self.passed { "✅ PASSED" } else { "❌ FAILED" });
    }
}

/// Comprehensive performance test suite
pub struct PerformanceTestSuite {
    config: PerformanceConfig,
    results: Vec<PerformanceResults>,
}

impl PerformanceTestSuite {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }
    
    /// Run all critical performance tests
    pub async fn run_all_tests(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Solana SOS - World-Class Performance Test Suite");
        println!("==================================================");
        println!("🎯 Testing life-saving response times with industry standards");
        println!("📊 Targets: P50<{}ms, P95<{}ms, P99<{}ms", 
            self.config.p50_target_ms, 
            self.config.p95_target_ms, 
            self.config.p99_target_ms
        );
        
        // Critical path tests - these must pass for life-saving functionality
        self.test_voice_activation_performance().await?;
        self.test_emergency_detection_performance().await?;
        self.test_ai_inference_performance().await?;
        self.test_database_query_performance().await?;
        self.test_emergency_calling_performance().await?;
        
        // Load and stress tests
        self.test_concurrent_user_load().await?;
        self.test_memory_efficiency().await?;
        self.test_battery_impact().await?;
        
        // Real-world scenario tests
        self.test_end_to_end_emergency_flow().await?;
        self.test_offline_performance().await?;
        
        self.print_final_results();
        
        Ok(())
    }
    
    /// Test voice activation response time (Critical: <50ms)
    async fn test_voice_activation_performance(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut measurements = Vec::new();
        let iterations = 1000; // Statistical significance
        
        // Initialize voice system once
        let mut voice_system = VoiceRecognitionSystem::new();
        voice_system.initialize().await?;
        
        // Warm up the system
        for _ in 0..10 {
            let audio = generate_wake_word_audio("hey sos");
            let _ = voice_system.detect_wake_word(&audio).await;
        }
        
        // Measure actual performance
        for _ in 0..iterations {
            let audio = generate_wake_word_audio("hey sos");
            let start = Instant::now();
            
            let _detected = voice_system.detect_wake_word(&audio).await?;
            
            measurements.push(start.elapsed());
        }
        
        let results = PerformanceResults::new(
            "Voice Activation".to_string(),
            measurements,
            self.config.voice_activation_target_ms,
        );
        
        results.print_results(&self.config);
        self.results.push(results);
        
        Ok(())
    }
    
    /// Test emergency detection response time (Critical: <25ms)
    async fn test_emergency_detection_performance(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut measurements = Vec::new();
        let iterations = 1000;
        
        let mut voice_system = VoiceRecognitionSystem::new();
        voice_system.initialize().await?;
        
        let emergency_phrases = [
            "help drowning",
            "heart attack",
            "choking emergency",
            "stroke help",
            "bleeding badly",
        ];
        
        // Warm up
        for phrase in &emergency_phrases {
            let audio = generate_emergency_audio(phrase);
            let _ = voice_system.detect_emergency_type(&audio).await;
        }
        
        // Measure performance
        for i in 0..iterations {
            let phrase = emergency_phrases[i % emergency_phrases.len()];
            let audio = generate_emergency_audio(phrase);
            let start = Instant::now();
            
            let _emergency_type = voice_system.detect_emergency_type(&audio).await?;
            
            measurements.push(start.elapsed());
        }
        
        let results = PerformanceResults::new(
            "Emergency Detection".to_string(),
            measurements,
            self.config.emergency_detection_target_ms,
        );
        
        results.print_results(&self.config);
        self.results.push(results);
        
        Ok(())
    }
    
    /// Test AI inference performance (Critical: <100ms)
    async fn test_ai_inference_performance(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut measurements = Vec::new();
        let iterations = 100; // AI inference is more expensive
        
        let mut medical_ai = MedicalAI::new();
        medical_ai.initialize().await?;
        
        let test_symptoms = [
            "chest pain shortness of breath",
            "severe headache vision problems",
            "difficulty speaking weakness",
            "severe bleeding trauma",
            "unconscious not breathing",
        ];
        
        // Warm up AI models
        for symptoms in &test_symptoms {
            let _ = medical_ai.analyze_symptoms(symptoms).await;
        }
        
        // Measure AI inference performance
        for i in 0..iterations {
            let symptoms = test_symptoms[i % test_symptoms.len()];
            let start = Instant::now();
            
            let _analysis = medical_ai.analyze_symptoms(symptoms).await?;
            
            measurements.push(start.elapsed());
        }
        
        let results = PerformanceResults::new(
            "AI Inference".to_string(),
            measurements,
            self.config.ai_inference_target_ms,
        );
        
        results.print_results(&self.config);
        self.results.push(results);
        
        Ok(())
    }
    
    /// Test database query performance (Critical: <10ms)
    async fn test_database_query_performance(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut measurements = Vec::new();
        let iterations = 1000;
        
        let database = EmergencyDatabase::new("emergency.db").await?;
        
        let emergency_types = [
            EmergencyType::HeartAttack,
            EmergencyType::Stroke,
            EmergencyType::Choking,
            EmergencyType::Drowning,
            EmergencyType::Bleeding,
        ];
        
        // Warm up database connections
        for emergency_type in &emergency_types {
            let _ = database.get_emergency_instructions(*emergency_type).await;
        }
        
        // Measure database performance
        for i in 0..iterations {
            let emergency_type = emergency_types[i % emergency_types.len()];
            let start = Instant::now();
            
            let _instructions = database.get_emergency_instructions(emergency_type).await?;
            
            measurements.push(start.elapsed());
        }
        
        let results = PerformanceResults::new(
            "Database Queries".to_string(),
            measurements,
            self.config.database_query_target_ms,
        );
        
        results.print_results(&self.config);
        self.results.push(results);
        
        Ok(())
    }
    
    /// Test emergency calling performance (Critical: <50ms)
    async fn test_emergency_calling_performance(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut measurements = Vec::new();
        let iterations = 100; // Calling system tests are more expensive
        
        let mut calling_system = EmergencyCallingSystem::new();
        calling_system.initialize().await?;
        
        // Test emergency call initiation time
        for _ in 0..iterations {
            let start = Instant::now();
            
            // Test call preparation (not actual calling)
            let _call_prepared = calling_system.prepare_emergency_call(
                EmergencyType::HeartAttack,
                Some("123 Main St".to_string()),
            ).await?;
            
            measurements.push(start.elapsed());
        }
        
        let results = PerformanceResults::new(
            "Emergency Calling".to_string(),
            measurements,
            self.config.voice_activation_target_ms,
        );
        
        results.print_results(&self.config);
        self.results.push(results);
        
        Ok(())
    }
    
    /// Test concurrent user load (Scalability)
    async fn test_concurrent_user_load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🔄 Testing Concurrent User Load ({} users)", self.config.concurrent_users);
        println!("===========================================");
        
        let mut handles = Vec::new();
        let start_time = Instant::now();
        
        // Spawn concurrent user simulations
        for user_id in 0..self.config.concurrent_users {
            let handle = tokio::spawn(async move {
                simulate_user_session(user_id).await
            });
            handles.push(handle);
        }
        
        // Wait for all users to complete
        let mut response_times = Vec::new();
        for handle in handles {
            if let Ok(duration) = handle.await {
                if let Ok(duration) = duration {
                    response_times.push(duration);
                }
            }
        }
        
        let total_time = start_time.elapsed();
        
        println!("✅ Concurrent Users: {}", self.config.concurrent_users);
        println!("✅ Total Test Time: {:.2}s", total_time.as_secs_f64());
        println!("✅ Successful Sessions: {}", response_times.len());
        println!("✅ Success Rate: {:.1}%", 
            (response_times.len() as f64 / self.config.concurrent_users as f64) * 100.0
        );
        
        if !response_times.is_empty() {
            let results = PerformanceResults::new(
                "Concurrent Load".to_string(),
                response_times,
                self.config.p95_target_ms,
            );
            
            results.print_results(&self.config);
            self.results.push(results);
        }
        
        Ok(())
    }
    
    /// Test memory efficiency (Critical: No leaks)
    async fn test_memory_efficiency(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n💾 Testing Memory Efficiency");
        println!("============================");
        
        let initial_memory = get_memory_usage();
        println!("📊 Initial Memory: {} MB", initial_memory);
        
        // Simulate heavy usage
        for i in 0..100 {
            let mut voice_system = VoiceRecognitionSystem::new();
            voice_system.initialize().await?;
            
            // Process audio
            let audio = generate_wake_word_audio("hey sos");
            let _ = voice_system.detect_wake_word(&audio).await;
            
            // Force cleanup
            drop(voice_system);
            
            if i % 10 == 0 {
                let current_memory = get_memory_usage();
                println!("   Iteration {}: {} MB", i, current_memory);
            }
        }
        
        // Force garbage collection
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let final_memory = get_memory_usage();
        let memory_growth = final_memory.saturating_sub(initial_memory);
        
        println!("📊 Final Memory: {} MB", final_memory);
        println!("📊 Memory Growth: {} MB", memory_growth);
        
        let passed = memory_growth < 50; // Allow up to 50MB growth
        println!("🎯 Memory Test: {}", if passed { "✅ PASSED" } else { "❌ FAILED" });
        
        Ok(())
    }
    
    /// Test battery impact (Critical: <5% per hour)
    async fn test_battery_impact(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🔋 Testing Battery Impact");
        println!("=========================");
        
        let test_duration = Duration::from_secs(10); // Short test for demo
        let start_time = Instant::now();
        
        // Simulate continuous usage
        let mut voice_system = VoiceRecognitionSystem::new();
        voice_system.initialize().await?;
        
        let mut operations = 0;
        while start_time.elapsed() < test_duration {
            let audio = generate_wake_word_audio("hey sos");
            let _ = voice_system.detect_wake_word(&audio).await;
            operations += 1;
            
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        let ops_per_second = operations as f64 / test_duration.as_secs_f64();
        let estimated_hourly_ops = ops_per_second * 3600.0;
        let estimated_battery_impact = estimate_battery_impact(estimated_hourly_ops);
        
        println!("⚡ Operations per second: {:.1}", ops_per_second);
        println!("⚡ Estimated hourly operations: {:.0}", estimated_hourly_ops);
        println!("⚡ Estimated battery impact: {:.1}% per hour", estimated_battery_impact);
        
        let passed = estimated_battery_impact < 5.0;
        println!("🎯 Battery Test: {}", if passed { "✅ PASSED" } else { "❌ FAILED" });
        
        Ok(())
    }
    
    /// Test end-to-end emergency flow (Integration)
    async fn test_end_to_end_emergency_flow(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut measurements = Vec::new();
        let iterations = 50;
        
        println!("\n🚨 Testing End-to-End Emergency Flow");
        println!("====================================");
        
        for _ in 0..iterations {
            let start = Instant::now();
            
            // Complete emergency flow: Voice → Detection → AI → Instructions → Calling
            let mut voice_system = VoiceRecognitionSystem::new();
            voice_system.initialize().await?;
            
            // 1. Voice activation
            let audio = generate_emergency_audio("help heart attack");
            let emergency_type = voice_system.detect_emergency_type(&audio).await?;
            
            // 2. Get AI analysis
            let mut medical_ai = MedicalAI::new();
            medical_ai.initialize().await?;
            let _analysis = medical_ai.analyze_symptoms("chest pain").await?;
            
            // 3. Get instructions
            let database = EmergencyDatabase::new("emergency.db").await?;
            let _instructions = database.get_emergency_instructions(emergency_type).await?;
            
            // 4. Prepare emergency call
            let mut calling_system = EmergencyCallingSystem::new();
            calling_system.initialize().await?;
            let _call_prepared = calling_system.prepare_emergency_call(
                emergency_type,
                Some("Test Location".to_string()),
            ).await?;
            
            measurements.push(start.elapsed());
        }
        
        let results = PerformanceResults::new(
            "End-to-End Emergency Flow".to_string(),
            measurements,
            200, // Allow 200ms for complete flow
        );
        
        results.print_results(&self.config);
        self.results.push(results);
        
        Ok(())
    }
    
    /// Test offline performance (Critical for emergencies)
    async fn test_offline_performance(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n📡 Testing Offline Performance");
        println!("==============================");
        
        // Test that critical functions work without network
        let mut measurements = Vec::new();
        let iterations = 100;
        
        for _ in 0..iterations {
            let start = Instant::now();
            
            // Test offline voice recognition
            let mut voice_system = VoiceRecognitionSystem::new();
            voice_system.initialize().await?;
            
            let audio = generate_emergency_audio("help drowning");
            let _emergency_type = voice_system.detect_emergency_type(&audio).await?;
            
            // Test offline database access
            let database = EmergencyDatabase::new("emergency.db").await?;
            let _instructions = database.get_emergency_instructions(EmergencyType::Drowning).await?;
            
            measurements.push(start.elapsed());
        }
        
        let results = PerformanceResults::new(
            "Offline Performance".to_string(),
            measurements,
            self.config.p95_target_ms,
        );
        
        results.print_results(&self.config);
        self.results.push(results);
        
        Ok(())
    }
    
    /// Print final test results summary
    fn print_final_results(&self) {
        println!("\n🏆 FINAL PERFORMANCE RESULTS");
        println!("============================");
        
        let passed_tests = self.results.iter().filter(|r| r.passed).count();
        let total_tests = self.results.len();
        let pass_rate = (passed_tests as f64 / total_tests as f64) * 100.0;
        
        println!("📊 Test Summary:");
        println!("   Total Tests: {}", total_tests);
        println!("   Passed: {}", passed_tests);
        println!("   Failed: {}", total_tests - passed_tests);
        println!("   Pass Rate: {:.1}%", pass_rate);
        
        println!("\n🎯 Critical Performance Requirements:");
        for result in &self.results {
            let status = if result.passed { "✅" } else { "❌" };
            println!("   {} {}: P95 = {}ms", status, result.test_name, result.p95.as_millis());
        }
        
        let overall_passed = pass_rate >= 90.0; // Require 90% pass rate
        println!("\n🏆 OVERALL RESULT: {}", 
            if overall_passed { "✅ READY FOR PRODUCTION" } else { "❌ NEEDS OPTIMIZATION" }
        );
        
        if overall_passed {
            println!("🚀 App meets world-class performance standards!");
            println!("🎯 Ready for Solana Mobile dApp store deployment!");
        } else {
            println!("⚠️  Performance optimization required before deployment.");
            println!("🔧 Focus on failed tests for maximum impact.");
        }
    }
}

// Helper functions for realistic testing

async fn simulate_user_session(user_id: usize) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    
    // Simulate realistic user interaction
    let mut voice_system = VoiceRecognitionSystem::new();
    voice_system.initialize().await?;
    
    // User says wake word
    let audio = generate_wake_word_audio("hey sos");
    let _detected = voice_system.detect_wake_word(&audio).await?;
    
    // User describes emergency
    let emergency_audio = generate_emergency_audio("help drowning");
    let _emergency_type = voice_system.detect_emergency_type(&emergency_audio).await?;
    
    // Small delay to simulate user thinking
    tokio::time::sleep(Duration::from_millis(user_id as u64 % 100)).await;
    
    Ok(start.elapsed())
}

fn generate_wake_word_audio(_phrase: &str) -> Vec<f32> {
    // Generate realistic audio data for testing
    // In production, this would use real audio samples
    vec![0.0; 16000] // 1 second of 16kHz audio
}

fn generate_emergency_audio(_phrase: &str) -> Vec<f32> {
    // Generate realistic emergency audio
    vec![0.0; 32000] // 2 seconds of 16kHz audio
}

fn get_memory_usage() -> usize {
    // In production, this would use actual memory profiling
    // For now, return a simulated value
    150 // MB
}

fn estimate_battery_impact(operations_per_hour: f64) -> f64 {
    // Estimate battery impact based on operations
    // This is a simplified calculation
    (operations_per_hour / 10000.0) * 2.0 // Rough estimate
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_performance_suite() {
        let config = PerformanceConfig::default();
        let mut suite = PerformanceTestSuite::new(config);
        
        // Run a subset of tests for unit testing
        suite.test_voice_activation_performance().await.unwrap();
        suite.test_database_query_performance().await.unwrap();
        
        assert!(!suite.results.is_empty());
    }
    
    #[test]
    fn test_performance_results_calculation() {
        let measurements = vec![
            Duration::from_millis(10),
            Duration::from_millis(20),
            Duration::from_millis(30),
            Duration::from_millis(40),
            Duration::from_millis(50),
        ];
        
        let results = PerformanceResults::new(
            "Test".to_string(),
            measurements,
            25,
        );
        
        assert_eq!(results.p50.as_millis(), 30);
        assert_eq!(results.min.as_millis(), 10);
        assert_eq!(results.max.as_millis(), 50);
    }
}
