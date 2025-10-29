# Solana SOS - Development Guide

**World-Class Development Environment for Life-Saving AI Technology**

> **Quick Start:** For complete step-by-step build instructions, see [BUILD_GUIDE.md](/BUILD_GUIDE.md)
> 
> **Production Deployment:** For deployment procedures and distribution, see [Deployment Guide](/docs/DEPLOYMENT.md)

---

## Prerequisites

### **Core Development Tools**
- **Rust 1.75+** - [Install Rust](https://rustup.rs/) with latest stable toolchain
- **Android Studio & SDK** - For mobile deployment and testing
- **Android NDK 26+** - Required for AAudio API and native library compilation
- **Solana CLI** - For blockchain integration and token rewards

### **AI Development Dependencies**
- **ONNX Runtime 1.16+** - For on-device AI inference
- **Python 3.9+** - For model conversion and optimization (optional)
- **Model Assets** - Pre-trained Whisper, MobileBERT, and T5 models

### **Platform-Specific Requirements**
- **macOS**: Xcode Command Line Tools for iOS development
- **Linux**: Build essentials and cross-compilation tools
- **Windows**: Visual Studio Build Tools and WSL2 (recommended)

---

## Quick Start

### **1. Environment Setup**
```bash
# Clone the repository
git clone https://github.com/paragoner1/crisis-companion-enhanced.git
cd crisis-companion-enhanced

# Install Rust targets for cross-compilation
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Verify installation
cargo --version
rustc --version
```

### **2. Development Build**
```bash
# Build with all features enabled
cargo build --features "voice,monitoring,audio,android"

# Run comprehensive tests
cargo test --all-features

# Build for Android (requires NDK setup)
./android-app/build-rust.sh

# Performance testing
cargo run --bin simple_perf_test
```

### **3. AI Model Setup**
```bash
# Download and verify AI models (automated)
cargo run --bin model_setup

# Verify model integrity
cargo run --bin verify_models

# Test AI inference pipeline
cargo run --bin ai_test_suite
```

---

## AI-Enhanced Technology Stack

### **Core AI Infrastructure**
- **ONNX Runtime (ORT) 2.0.0-rc.10** - Production-ready on-device inference
- **Whisper-base** - 74M parameter multilingual speech recognition
- **RNNoise** - Professional-grade noise reduction for >95% accuracy

### **Mobile & Platform Integration**
- **Rust 1.75+** - Memory-safe, high-performance core
- **Android NDK 26+** - Native Android integration with AAudio
- **Solana SDK 2.0** - Blockchain integration for secure token rewards
- **JNI Bridge** - Seamless Rust-Java interoperability
- **SQLite** - Encrypted local database for emergency protocols

### **Security & Privacy**
- **AES-256 Encryption** - Military-grade data protection
- **SHA256 Verification** - Model integrity assurance
- **On-Device Processing** - Zero data transmission privacy model
- **HIPAA/GDPR Compliance** - Healthcare-grade privacy standards

---

## Development Commands

### **Core Development**
```bash
# Development build with debug info
cargo build --features "voice,monitoring,audio"

# Release build optimized for mobile
cargo build --release --features "voice,audio,android"

# Run with debug logging
RUST_LOG=debug cargo run --features "voice,monitoring"

# Memory profiling
cargo run --bin memory_profiler --features "voice,monitoring"
```

### **AI Model Testing**
```bash
# Test Whisper speech recognition
cargo run --bin whisper_test --features "voice"

# Test T5 guidance generation
cargo run --bin guidance_test --features "voice"

# Complete AI pipeline test
cargo run --bin ai_integration_test --features "voice,monitoring"
```

### **Mobile Development**
```bash
# Build Android native library
./android-app/build-rust.sh

# Install on connected device
adb install android-app/app/build/outputs/apk/debug/app-debug.apk

# Monitor Android logs
adb logcat -s CrisisCompanion:D

# Test on Android emulator
./scripts/test_android_emulator.sh
```

### **Performance & Benchmarking**
```bash
# Voice-to-action latency test
cargo run --bin latency_benchmark --features "voice"

# AI inference performance
cargo run --bin ai_performance_test --features "voice"

# Memory usage analysis
cargo run --bin memory_analysis --features "voice,monitoring"

# Battery impact measurement
cargo run --bin battery_test --features "voice,audio"
```

## Key Components

- **Voice Interface** - `src/public/voice_interface.rs`
- **Emergency System** - `src/public/emergency_interface.rs`
- **Gamification** - `src/public/gamification_interface.rs`
- **Safety Features** - `src/public/safety_interface.rs`

## Building for Android

```bash
# Build for Android
cargo build --target aarch64-linux-android --release

# Install on device
adb install target/aarch64-linux-android/release/solana-sos.apk
```

## Database Setup

```bash
# Initialize database
sqlite3 crisis_companion.db < assets/database.sql

# Verify setup
sqlite3 crisis_companion.db "SELECT COUNT(*) FROM emergency_types;"
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test --bin voice_test
cargo test --bin safety_features_test
cargo test --bin gamification_demo
```

### Comprehensive Testing Strategy

For detailed testing approaches for life-critical software, including protocol validation, performance benchmarks, integration tests, and security audits, see our [Testing Strategy](/docs/TESTING_STRATEGY.md).

Review working test implementations in [Integration Test Examples](/tests/integration_test_examples.rs).

### Testing Without Physical Device
- Use Android Emulator (see [Build Guide](/BUILD_GUIDE.md) for setup)
- Cloud services: BrowserStack for real hardware tests
- Seeker device testing: Enhanced performance on Solana Mobile hardware

## Performance Testing

```bash
# Voice recognition performance
cargo run --bin voice_test -- --benchmark

# Emergency response timing
cargo run --bin demo_test -- --timing
```

For detailed performance targets, benchmarking methodology, and optimization strategies, see [Performance Documentation](/docs/PERFORMANCE.md).

## Troubleshooting

### Common Issues

1. **Voice Recognition Not Working**
   - Ensure microphone permissions are granted
   - Check Vosk model installation
   - Verify audio device configuration

2. **Database Connection Errors**
   - Verify SQLite installation
   - Check database file permissions
   - Ensure database schema is initialized

3. **Android Build Issues**
   - Verify Android NDK installation
   - Check target architecture support
   - Ensure all dependencies are available

### Debug Mode

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Verbose output
cargo run -- --verbose
```

## Additional Resources

### Developer Documentation
- **[Build Guide](/BUILD_GUIDE.md)** - Complete step-by-step build instructions for all platforms
- **[API Documentation](/docs/API.md)** - Complete developer API reference with JNI bridge details
- **[Architecture Overview](/ARCHITECTURE.md)** - System design and component interactions
- **[Contributing Guidelines](/CONTRIBUTING.md)** - Contribution process and standards

### Implementation Examples
- **[Emergency Protocol Example](/examples/emergency_protocol_example.rs)** - Database-driven guidance system
- **[Voice Activation Example](/examples/voice_activation_example.rs)** - Speech recognition implementation
- **[Blockchain Rewards Example](/examples/blockchain_rewards_example.rs)** - Solana token integration
- **[Examples README](/examples/README.md)** - Guide to all code examples

### Quality Assurance
- **[Testing Strategy](/docs/TESTING_STRATEGY.md)** - Comprehensive testing approach for life-critical systems
- **[Performance Benchmarks](/docs/PERFORMANCE.md)** - Latency targets and optimization strategies
- **[Security Model](/docs/SECURITY.md)** - Threat model and security architecture
- **[Deployment Guide](/docs/DEPLOYMENT.md)** - Production build and distribution procedures
