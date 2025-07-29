# Crisis Companion

A voice-activated emergency response app for Solana Mobile Seeker phone (Android-based), with potential Android expansion.

## 🚨 Project Overview

Crisis Companion is a life-saving emergency response application that detects emergency voice triggers (e.g., "Drowning help!") and provides immediate, coordinated emergency assistance. The app is designed for the Solana Mobile Seeker platform with future expansion to Android devices.

### Key Features

- **Voice Trigger Detection**: Offline speech recognition using Vosk to detect emergency phrases with <1% false positives
- **Auto Volume Control**: Sets phone volume to 100% during emergencies via Android AudioManager
- **Multi-Device Coordination**: Uses Bluetooth Low Energy to coordinate multiple nearby devices
- **Offline Emergency Instructions**: SQLite database with step-by-step emergency procedures
- **Solana Blockchain Integration**: Stores audio hashes and emergency data on-chain
- **Emergency Response Features**: Auto-dial 911, location sharing, audio recording

### Revenue Goals

- **2026**: $150,000–$300,000/yr (2,500 users at $5–$10/mo)
- **2027**: $1.26M/yr (15,000 users) + $750,000/yr (15 enterprise clients)
- **2030**: $10M–$50M Solana acquisition

## 🏗️ Architecture

```
Crisis Companion/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # Library exports
│   ├── app.rs            # Main application coordinator
│   ├── voice.rs          # Voice trigger detection (Vosk)
│   ├── audio.rs          # Audio management (TTS, volume control)
│   ├── database.rs       # SQLite emergency instructions
│   ├── coordination.rs   # BLE device coordination
│   ├── emergency.rs      # Emergency response handling
│   ├── blockchain.rs     # Solana integration
│   ├── ui.rs             # Cross-platform UI (egui/Android)
│   ├── config.rs         # Configuration management
│   └── error.rs          # Error handling
├── assets/
│   └── database.sql      # SQLite schema
├── Cargo.toml            # Dependencies and configuration
└── README.md
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Android SDK (for Android development)
- Solana CLI (for blockchain features)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/crisis-companion/crisis-companion.git
   cd crisis-companion
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Download Vosk model** (required for voice recognition)
   ```bash
   mkdir -p models
   # Download vosk-model-small-en-us-0.15 from https://alphacephei.com/vosk/models
   # Extract to models/vosk-model-small-en-us-0.15/
   ```

4. **Initialize database**
   ```bash
   mkdir -p data
   sqlite3 data/emergencies.db < assets/database.sql
   ```

### Running the Application

#### Desktop Mode (for testing)
```bash
cargo run -- --desktop
```

#### Mobile Mode
```bash
cargo run
```

#### With custom configuration
```bash
cargo run -- --config custom_config.toml --debug
```

## 🧪 Testing

### Run all tests
```bash
cargo test
```

### Run specific module tests
```bash
cargo test voice
cargo test audio
cargo test database
cargo test coordination
cargo test emergency
cargo test ui
cargo test blockchain
```

### Test voice trigger detection
```bash
cargo test test_emergency_phrase_detection
```

### Test emergency response flow
```bash
cargo test test_emergency_response_lifecycle
```

### Test database functionality
```bash
cargo test test_database_creation
```

## 📱 Android Development

### Setup Android Environment

1. **Install Android SDK**
   ```bash
   # Install Android Studio or command line tools
   # Set ANDROID_HOME environment variable
   export ANDROID_HOME=/path/to/android/sdk
   ```

2. **Install NDK**
   ```bash
   # Install Android NDK via Android Studio or command line
   # Set ANDROID_NDK_HOME environment variable
   export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/version
   ```

3. **Build for Android**
   ```bash
   cargo build --target aarch64-linux-android --release
   cargo build --target armv7-linux-androideabi --release
   cargo build --target i686-linux-android --release
   cargo build --target x86_64-linux-android --release
   ```

### Android Permissions

Add these permissions to `AndroidManifest.xml`:

```xml
<uses-permission android:name="android.permission.RECORD_AUDIO" />
<uses-permission android:name="android.permission.MODIFY_AUDIO_SETTINGS" />
<uses-permission android:name="android.permission.BLUETOOTH" />
<uses-permission android:name="android.permission.BLUETOOTH_ADMIN" />
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
<uses-permission android:name="android.permission.CALL_PHONE" />
<uses-permission android:name="android.permission.SEND_SMS" />
<uses-permission android:name="android.permission.INTERNET" />
```

## 🔧 Configuration

### Default Configuration

The app uses default configuration values. Create a `config.toml` file to customize:

```toml
[voice]
model_path = "models/vosk-model-small-en-us-0.15"
confidence_threshold = 0.8
emergency_phrases = ["drowning help", "fire help", "heart attack help"]

[audio]
default_volume = 0.5
emergency_volume = 1.0
enable_recording = true

[database]
database_path = "data/emergencies.db"
enable_encryption = false

[coordination]
service_uuid = "12345678-1234-1234-1234-123456789abc"
max_nearby_devices = 5

[emergency]
auto_dial_911 = true
enable_location_sharing = true
enable_audio_recording = true

[blockchain]
enable_blockchain = false
rpc_endpoint = "https://api.mainnet-beta.solana.com"
```

## 📊 Database Schema

The SQLite database includes:

- **emergency_types**: Types of emergencies (drowning, fire, etc.)
- **emergency_instructions**: Step-by-step instructions for each emergency
- **emergency_responses**: Records of emergency responses
- **audio_recordings**: Encrypted audio recordings
- **device_coordination**: BLE coordination data
- **blockchain_transactions**: Solana transaction records
- **user_settings**: User preferences
- **emergency_contacts**: Emergency contact information

## 🔐 Security & Privacy

### Data Protection

- Audio recordings are encrypted using AES-GCM
- Location data is only shared during emergencies
- Blockchain storage uses zero-knowledge proofs where possible
- HIPAA/GDPR compliance for medical data

### Emergency Protocols

- Voice triggers require high confidence (>80%)
- Multiple device coordination prevents false positives
- Emergency data is stored locally and on blockchain
- Audio recordings are automatically deleted after 30 days

## 🚀 Development Roadmap

### Phase 1 (August 2025) - Prototype
- [x] Voice trigger detection for "Drowning help"
- [x] Auto volume control
- [x] SQLite database with 3 emergency types
- [x] Basic Android integration

### Phase 2 (January 2026) - MVP
- [ ] BLE device coordination
- [ ] 10-emergency database expansion
- [ ] TTS/MP3 audio playback
- [ ] Solana blockchain integration
- [ ] Android app store preparation

### Phase 3 (April 2026) - Beta Launch
- [ ] Solana Breakout hackathon submission
- [ ] 2,000 beta users
- [ ] Enterprise client pilots
- [ ] Performance optimization

### Phase 4 (July 2026) - Public Launch
- [ ] Solana dApp store launch
- [ ] Freemium model ($5–$10/mo)
- [ ] 2,500 paying users target
- [ ] Enterprise sales team

## 🤝 Contributing

### Development Setup

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/voice-improvements
   ```
3. **Make your changes**
4. **Add tests**
   ```bash
   cargo test
   ```
5. **Submit a pull request**

### Code Style

- Follow Rust conventions
- Use meaningful variable names
- Add comprehensive tests
- Document public APIs
- Handle errors gracefully

### Testing Guidelines

- Unit tests for all modules
- Integration tests for emergency flows
- Performance tests for voice recognition
- Security tests for data protection

## 📈 Performance Targets

### Voice Recognition
- <100ms trigger detection
- <1% false positive rate
- Works in noisy environments (beach, crowd)

### Audio Processing
- <50ms volume adjustment
- High-quality TTS synthesis
- Efficient audio compression

### Database Queries
- <10ms instruction retrieval
- Optimized indexes for performance
- Efficient emergency response tracking

### Battery Optimization
- Minimal BLE power usage
- Efficient voice processing
- Smart wake-up patterns

## 🐛 Troubleshooting

### Common Issues

**Voice recognition not working**
```bash
# Check Vosk model installation
ls models/vosk-model-small-en-us-0.15/
# Verify audio permissions on Android
```

**Database connection errors**
```bash
# Check database file permissions
ls -la data/emergencies.db
# Reinitialize database
sqlite3 data/emergencies.db < assets/database.sql
```

**Android build failures**
```bash
# Check NDK installation
echo $ANDROID_NDK_HOME
# Verify target installation
rustup target list --installed
```

### Debug Mode

Run with debug logging:
```bash
cargo run -- --debug
```

### Performance Profiling

```bash
# Profile voice recognition
cargo test test_voice_performance -- --nocapture

# Profile database queries
cargo test test_database_performance -- --nocapture
```

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Vosk for offline speech recognition
- Solana Foundation for blockchain infrastructure
- Rust community for excellent tooling
- Emergency response professionals for guidance

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/crisis-companion/crisis-companion/issues)
- **Discussions**: [GitHub Discussions](https://github.com/crisis-companion/crisis-companion/discussions)
- **Email**: support@crisiscompanion.com

---

**⚠️ Emergency Notice**: This software is designed for emergency response. Always call 911 for life-threatening emergencies. This app is a supplement to, not a replacement for, professional emergency services. 