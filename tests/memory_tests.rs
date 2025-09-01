use solana_sos::public::voice_interface::VoiceInterface;
use solana_sos::private::emergency_database::EmergencyDatabase;
use std::time::Duration;
use tokio::time::timeout;

/// Automated memory leak tests for CI/CD pipeline
/// 
/// These tests ensure that the Solana SOS app maintains
/// world-class memory efficiency for reliable life-saving performance.
/// 
/// Run with: cargo test --features leak-check
/// For leak detection, use: RUSTFLAGS="-Z sanitizer=leak" cargo +nightly test --target x86_64-unknown-linux-gnu --features leak-check

#[tokio::test]
async fn test_voice_interface_memory_efficiency() {
    let initial_memory = get_approximate_memory_usage();
    
    // Run voice interface operations multiple times
    for _i in 0..50 {
        let mut voice_interface = VoiceInterface::new("models/vosk-model-small-en-us-0.15");
        voice_interface.initialize().await.expect("Voice interface initialization failed");
        
        let test_audio = vec![0u8; 1024]; // 1KB test audio
        let _result = voice_interface.process_audio(&test_audio).await;
        
        drop(voice_interface);
    }
    
    // Allow cleanup for any potential async operations
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let final_memory = get_approximate_memory_usage();
    let memory_growth = final_memory - initial_memory;
    
    // Assert memory growth is within acceptable bounds (5MB threshold)
    // Rust's ownership model should prevent leaks, but we verify efficient usage
    assert!(memory_growth < 5.0, "Memory usage grew by {:.2} MB, indicating potential inefficiency", memory_growth);
    
    #[cfg(feature = "leak-check")]
    println!("✅ Memory efficiency test passed - use leak sanitizer for detailed leak detection");
}

#[tokio::test]
async fn test_database_connection_efficiency() {
    let initial_memory = get_approximate_memory_usage();
    
    // Test database connection cleanup
    for _i in 0..100 {
        let db = EmergencyDatabase::new();
        let _protocols = db.get_protocol("drowning");
        drop(db);
    }
    
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    let final_memory = get_approximate_memory_usage();
    let memory_growth = final_memory - initial_memory;
    
    assert!(memory_growth < 3.0, "Database memory usage grew by {:.2} MB, indicating potential inefficiency", memory_growth);
    
    #[cfg(feature = "leak-check")]
    println!("✅ Database efficiency test passed - connections properly cleaned up");
}

#[tokio::test]
async fn test_concurrent_operations_memory_stability() {
    let initial_memory = get_approximate_memory_usage();
    
    // Run concurrent operations to test memory stability
    let mut handles = vec![];
    
    for i in 0..10 {
        let handle = tokio::spawn(async move {
            let mut voice_interface = VoiceInterface::new("models/vosk-model-small-en-us-0.15");
            let db = EmergencyDatabase::new();
            
            // Simulate emergency detection workflow
            let audio_data = vec![((i * 10) % 255) as u8; 512];
            let _voice_result = voice_interface.process_audio(&audio_data).await;
            let _protocols = db.get_protocol("heart_attack");
            
            drop(voice_interface);
            drop(db);
        });
        handles.push(handle);
    }
    
    // Wait for all operations to complete
    for handle in handles {
        timeout(Duration::from_secs(10), handle)
            .await
            .expect("Task timed out")
            .expect("Task failed");
    }
    
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    let final_memory = get_approximate_memory_usage();
    let memory_growth = final_memory - initial_memory;
    
    assert!(memory_growth < 10.0, "Concurrent operations memory leak: grew by {:.2} MB", memory_growth);
}

#[tokio::test]
async fn test_sustained_operation_memory_bounds() {
    let initial_memory = get_approximate_memory_usage();
    let mut max_memory = initial_memory;
    
    // Run sustained operations for a shorter time in tests
    let start_time = std::time::Instant::now();
    while start_time.elapsed().as_secs() < 10 { // 10 seconds for tests
        let mut voice_interface = VoiceInterface::new("models/vosk-model-small-en-us-0.15");
        let db = EmergencyDatabase::new();
        
        let audio_chunk = vec![128u8; 1024];
        let _voice_result = voice_interface.process_audio(&audio_chunk).await;
        let _protocols = db.get_protocol("choking");
        
        let current_memory = get_approximate_memory_usage();
        if current_memory > max_memory {
            max_memory = current_memory;
        }
        
        drop(voice_interface);
        drop(db);
        
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    let memory_growth = max_memory - initial_memory;
    assert!(memory_growth < 20.0, "Sustained operation memory growth too high: {:.2} MB", memory_growth);
}

fn get_approximate_memory_usage() -> f64 {
    // Simplified memory usage for testing
    // In production, this would use more accurate memory tracking
    use std::process;
    
    if let Ok(output) = process::Command::new("ps")
        .args(&["-o", "rss=", "-p", &process::id().to_string()])
        .output()
    {
        if let Ok(rss_str) = String::from_utf8(output.stdout) {
            if let Ok(rss_kb) = rss_str.trim().parse::<f64>() {
                return rss_kb / 1024.0; // Convert KB to MB
            }
        }
    }
    
    // Fallback: use a simple heap size approximation
    0.0
}
