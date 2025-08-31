# Development Guide

## Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Android SDK** - For mobile deployment
- **Solana CLI** - For blockchain integration

## Quick Start

```bash
# Clone the repository
git clone https://github.com/paragoner1/crisis-companion.git
cd crisis-companion

# Build the project
cargo build

# Run tests
cargo test

# Start the application
cargo run
```

## Demo Commands

```bash
# Voice recognition demo
cargo run --bin voice_test

# Complete walkthrough demo
cargo run --bin complete_walkthrough

# Gamification demo
cargo run --bin gamification_demo

# Safety features demo
cargo run --bin safety_features_test

# Context analysis demo
cargo run --bin context_analysis_test
```

## Technology Stack

- **Rust** - Reliability and performance for critical emergency systems
- **Vosk** - Offline speech recognition for voice activation
- **RNNoise** - Enterprise-grade noise filtering for voice clarity
- **SQLite** - Local data storage with encrypted database
- **Solana** - Blockchain integration for verification and token rewards
- **Android JNI** - Native platform integration for mobile deployment

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

## Testing Without Physical Device
- Use Android Emulator.
- Cloud services: BrowserStack for real hardware tests.

## Performance Testing

```bash
# Voice recognition performance
cargo run --bin voice_test -- --benchmark

# Emergency response timing
cargo run --bin demo_test -- --timing
```

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

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

## API Documentation

See [docs/API.md](docs/API.md) for complete developer API reference.
