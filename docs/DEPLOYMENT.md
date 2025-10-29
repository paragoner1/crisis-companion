# Deployment Guide for Solana SOS

## Overview

This guide covers building, testing, and deploying Solana SOS for Android devices. The application targets the Solana Mobile ecosystem with specific optimizations for Seeker while maintaining compatibility with standard Android devices.

---

## Prerequisites

### Development Environment

**Required Software:**
- Android Studio Arctic Fox (2020.3.1) or later
- Android SDK API Level 26+ (Android 8.0+)
- Android NDK r25c or later
- Rust toolchain (stable channel)
- Gradle 7.0+

**Rust Targets:**
```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android
```

**Android NDK Setup:**
```bash
export ANDROID_NDK_HOME=/path/to/ndk
export ANDROID_HOME=/path/to/sdk
```

### Hardware Requirements

**Development Machine:**
- CPU: Multi-core processor (4+ cores recommended)
- RAM: 16GB minimum, 32GB recommended
- Storage: 20GB free space for SDK, NDK, and build artifacts
- OS: macOS, Linux, or Windows with WSL2

**Target Devices:**
- Android 8.0 (API 26) or higher
- 4GB RAM minimum
- ARM64 architecture (primary target)
- GPS and accelerometer sensors

---

## Project Structure

```
solana-sos/
├── android-app/              # Android application
│   ├── app/
│   │   ├── src/main/
│   │   │   ├── java/        # Kotlin source code
│   │   │   ├── res/         # Android resources
│   │   │   └── cpp/         # Native C++ bridge
│   │   └── build.gradle     # App build configuration
│   ├── build.gradle         # Project build configuration
│   └── gradle.properties    # Gradle settings
├── src/                     # Rust source code
│   ├── lib.rs              # Library entry point
│   ├── jni_bridge.rs       # JNI bindings
│   ├── emergency_database.rs
│   └── onchain/            # Blockchain integration
├── models/                  # Voice recognition models
├── Cargo.toml              # Rust dependencies
└── build.rs                # Rust build script
```

---

## Building the Application

### Step 1: Prepare Rust Library

Build the Rust core library for Android targets:

```bash
# Navigate to project root
cd /path/to/solana-sos

# Build for ARM64 (primary target)
cargo build --target aarch64-linux-android --release

# Build for ARMv7 (compatibility)
cargo build --target armv7-linux-androideabi --release

# Build for x86_64 (emulator support)
cargo build --target x86_64-linux-android --release
```

**Build Output:**
- Libraries generated in `target/{architecture}/release/`
- Files: `libsolana_sos.so`
- Copy to `android-app/app/src/main/jniLibs/{architecture}/`

### Step 2: Build Android Application

#### Using Android Studio (Recommended)

1. Open Android Studio
2. File → Open → Select `android-app` directory
3. Wait for Gradle sync to complete
4. Build → Make Project (Ctrl+F9 / Cmd+F9)
5. Build → Build Bundle(s) / APK(s) → Build APK(s)

#### Using Command Line

```bash
cd android-app

# Clean previous builds
./gradlew clean

# Build debug APK
./gradlew assembleDebug

# Build release APK
./gradlew assembleRelease

# Build app bundle (for Play Store)
./gradlew bundleRelease
```

**Build Outputs:**
- Debug APK: `android-app/app/build/outputs/apk/debug/app-debug.apk`
- Release APK: `android-app/app/build/outputs/apk/release/app-release.apk`
- App Bundle: `android-app/app/build/outputs/bundle/release/app-release.aab`

### Step 3: Sign Release Build

For production deployment, sign the APK/AAB:

```bash
# Generate keystore (first time only)
keytool -genkey -v -keystore solana-sos-release.keystore \
  -alias solana-sos -keyalg RSA -keysize 2048 -validity 10000

# Sign APK
jarsigner -verbose -sigalg SHA256withRSA -digestalg SHA-256 \
  -keystore solana-sos-release.keystore \
  app/build/outputs/apk/release/app-release-unsigned.apk solana-sos

# Align APK
zipalign -v 4 app-release-unsigned.apk app-release.apk
```

---

## Testing Before Deployment

### Local Testing on Emulator

1. **Create AVD:**
   - Tools → Device Manager → Create Device
   - Select: Pixel 5 or Pixel 6
   - System Image: API 33 (Android 13) with Google APIs
   - Graphics: Hardware - GLES 2.0

2. **Install and Test:**
```bash
# Start emulator
emulator -avd Pixel_5_API_33

# Install APK
adb install app/build/outputs/apk/debug/app-debug.apk

# Launch app
adb shell am start -n com.solanasos.emergency/.MainActivity

# View logs
adb logcat | grep "SolanaSOSEmergency"
```

### Physical Device Testing

1. **Enable Developer Mode:**
   - Settings → About Phone → Tap "Build Number" 7 times

2. **Enable USB Debugging:**
   - Settings → Developer Options → USB Debugging

3. **Install:**
```bash
# Connect device via USB
adb devices

# Install APK
adb install app-debug.apk

# Monitor device logs
adb logcat -s SolanaSOSEmergency:V
```

### Seeker Specific Testing

For Seeker devices, verify:
- Mobile Wallet Adapter integration
- dApp Store compatibility
- Solana-specific sensors and features

```bash
# Check for Seeker-specific features
adb shell pm list features | grep solana

# Test wallet adapter
adb logcat | grep "MobileWalletAdapter"
```

---

## Deployment Channels

### Option 1: Solana dApp Store (Recommended)

**Benefits:**
- Native Solana Mobile ecosystem
- Built-in wallet integration
- Crypto-native user base
- Lower fees than traditional stores

**Submission Process:**
1. Create developer account on Solana dApp Store
2. Prepare store listing:
   - App name: "Solana SOS"
   - Category: Health & Safety
   - Screenshots (4-8 required)
   - Description (see marketing guide)
3. Upload signed AAB file
4. Submit for review
5. Monitor review status

**Requirements:**
- Signed app bundle (.aab)
- Privacy policy URL
- Content rating questionnaire
- Feature graphic (1024x500)

### Option 2: Google Play Store

**Benefits:**
- Largest user base
- Established distribution
- Automatic updates

**Submission Process:**
1. Create Google Play Console account ($25 one-time fee)
2. Create new application
3. Complete store listing:
   - Title, description, screenshots
   - Content rating (ESRB: Everyone)
   - Privacy policy
4. Upload signed app bundle
5. Submit for review (typically 2-7 days)

**Play Store Listing Requirements:**
- High-res icon: 512x512
- Feature graphic: 1024x500
- Screenshots: Minimum 2, recommended 8
- Privacy policy URL (required)
- Content rating certificate

### Option 3: Direct APK Distribution

For beta testing or direct distribution:

**Hosting Options:**
- GitHub Releases
- Self-hosted download page
- Firebase App Distribution

**Distribution:**
```bash
# Create release on GitHub
gh release create v1.0.0 app-release.apk \
  --title "Solana SOS v1.0.0" \
  --notes "Initial release"
```

**User Installation:**
1. Download APK from trusted source
2. Enable "Install from Unknown Sources"
3. Open APK file to install
4. Grant required permissions

---

## Configuration for Production

### Release Build Configuration

Update `android-app/app/build.gradle`:

```gradle
android {
    defaultConfig {
        applicationId "com.solanasos.emergency"
        minSdkVersion 26
        targetSdkVersion 34
        versionCode 1
        versionName "1.0.0"
    }

    buildTypes {
        release {
            minifyEnabled true
            shrinkResources true
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'),
                         'proguard-rules.pro'
            signingConfig signingConfigs.release
        }
    }
}
```

### ProGuard Configuration

For code obfuscation and optimization, ensure `proguard-rules.pro` includes:

```proguard
# Keep Rust JNI interfaces
-keepclasseswithmembernames class * {
    native <methods>;
}

# Keep Solana SDK classes
-keep class com.solana.** { *; }

# Keep emergency service classes
-keep class com.solanasos.emergency.** { *; }
```

### Environment Variables

Set production configuration in `gradle.properties`:

```properties
# Production Solana RPC endpoint
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com

# Release signing configuration
RELEASE_STORE_FILE=/path/to/keystore
RELEASE_STORE_PASSWORD=***
RELEASE_KEY_ALIAS=solana-sos
RELEASE_KEY_PASSWORD=***
```

---

## Performance Optimization

### APK Size Optimization

Reduce APK size for faster downloads:

```gradle
android {
    // Enable APK splits
    splits {
        abi {
            enable true
            reset()
            include 'arm64-v8a', 'armeabi-v7a', 'x86_64'
            universalApk false
        }
    }
}
```

**Results:**
- Universal APK: ~45MB
- ARM64 split: ~18MB
- ARMv7 split: ~16MB

### Runtime Performance

Optimize for emergency response speed:

```kotlin
// Enable hardware acceleration
<application android:hardwareAccelerated="true">

// Use aggressive app standby exemptions for emergency services
<uses-permission android:name="android.permission.REQUEST_IGNORE_BATTERY_OPTIMIZATIONS"/>
```

---

## Post-Deployment

### Monitoring

**Crash Reporting:**
- Integrate Firebase Crashlytics (optional)
- Monitor Android Vitals in Play Console
- Review user feedback regularly

**Analytics:**
- Track app launches
- Monitor emergency activation rates
- Measure training completion rates
- Review blockchain transaction success rates

### Updates

**Version Management:**
```gradle
// Increment for each release
versionCode 2          // Integer, increments by 1
versionName "1.1.0"    // Semantic versioning
```

**Release Process:**
1. Update version numbers
2. Update CHANGELOG.md
3. Build signed release
4. Test on multiple devices
5. Upload to distribution channel
6. Monitor initial rollout
7. Respond to user feedback

### Rollback Plan

If critical issues are discovered:

1. **Immediate Response:**
   - Halt staged rollout in Play Console
   - Publish hotfix version if possible
   - Revert to previous version if necessary

2. **Communication:**
   - Update app description with known issues
   - Post on social media (X: @paragoner1)
   - Respond to user reviews

---

## Security Considerations

### Pre-Deployment Checklist

- [ ] All API keys secured (not in source code)
- [ ] ProGuard/R8 obfuscation enabled
- [ ] Network security config in place
- [ ] Certificate pinning implemented
- [ ] Sensitive data encrypted
- [ ] Debug logging disabled
- [ ] Test accounts removed
- [ ] Security audit completed

### Permission Review

Ensure only necessary permissions requested:

```xml
<!-- Critical permissions -->
<uses-permission android:name="android.permission.INTERNET" />
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
<uses-permission android:name="android.permission.RECORD_AUDIO" />
<uses-permission android:name="android.permission.CALL_PHONE" />
<uses-permission android:name="android.permission.SEND_SMS" />
<uses-permission android:name="android.permission.READ_CONTACTS" />
```

Each permission must be justified in privacy policy.

---

## Troubleshooting

### Common Build Issues

**Issue: NDK not found**
```bash
# Solution: Set NDK path
export ANDROID_NDK_HOME=/path/to/ndk
```

**Issue: Rust library not linked**
```bash
# Solution: Verify JNI libs directory
ls android-app/app/src/main/jniLibs/arm64-v8a/
# Should contain libsolana_sos.so
```

**Issue: Gradle sync failed**
```bash
# Solution: Clear Gradle cache
./gradlew clean
rm -rf ~/.gradle/caches
```

### Runtime Issues

**Issue: App crashes on startup**
- Check logcat for native library errors
- Verify all JNI methods are properly bound
- Test on multiple device architectures

**Issue: Voice recognition not working**
- Verify microphone permissions granted
- Check voice model files are included
- Test audio input path

---

## Continuous Integration

### GitHub Actions Workflow

Create `.github/workflows/build.yml`:

```yaml
name: Build Android APK

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: aarch64-linux-android
      
      - name: Build Rust library
        run: cargo build --target aarch64-linux-android --release
      
      - name: Set up JDK
        uses: actions/setup-java@v3
        with:
          java-version: '17'
          distribution: 'temurin'
      
      - name: Build Android APK
        run: |
          cd android-app
          ./gradlew assembleDebug
      
      - name: Upload APK
        uses: actions/upload-artifact@v3
        with:
          name: app-debug
          path: android-app/app/build/outputs/apk/debug/app-debug.apk
```

---

## Support and Resources

**Documentation:**
- Architecture: `/docs/ARCHITECTURE.md`
- Security: `/docs/SECURITY.md`
- Contributing: `/docs/CONTRIBUTING.md`

**Contact:**
- X: @paragoner1
- Repository: https://github.com/paragoner1/crisis-companion

**External Resources:**
- Android Developer Guide: https://developer.android.com
- Solana Mobile Docs: https://docs.solanamobile.com
- Rust Android Guide: https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html

---

## Conclusion

Deploying Solana SOS requires careful attention to build configuration, testing, and security. This life-saving application must maintain the highest standards of reliability and performance. Follow this guide to ensure successful deployment across all target platforms.

