# Solana SOS Build Guide

Complete guide for building, testing, and running Solana SOS on your development machine and Android devices.

---

## Quick Start

For experienced developers who want to get started immediately:

```bash
# Clone repository
git clone https://github.com/paragoner1/crisis-companion.git
cd crisis-companion

# Install Rust targets
rustup target add aarch64-linux-android

# Build Rust library
cargo build --target aarch64-linux-android --release

# Open Android project in Android Studio
cd android-app
# Build -> Build APK

# Install on device
adb install app/build/outputs/apk/debug/app-debug.apk
```

---

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Environment Setup](#environment-setup)
3. [Building Rust Components](#building-rust-components)
4. [Building Android App](#building-android-app)
5. [Running Tests](#running-tests)
6. [Troubleshooting](#troubleshooting)
7. [Development Workflow](#development-workflow)

---

## System Requirements

### Development Machine

**Minimum:**
- CPU: Quad-core processor
- RAM: 8GB
- Storage: 10GB free space
- OS: macOS 10.15+, Ubuntu 20.04+, or Windows 10+ with WSL2

**Recommended:**
- CPU: 8+ core processor
- RAM: 16GB+
- Storage: 20GB+ SSD
- OS: macOS 13+, Ubuntu 22.04+

### Target Device

**Minimum:**
- Android 8.0 (API 26)
- RAM: 4GB
- Storage: 2GB free
- GPS and accelerometer sensors

**Recommended:**
- Seeker or Pixel 6+
- Android 13+ (API 33+)
- RAM: 8GB+
- Storage: 5GB+ free

---

## Environment Setup

### Step 1: Install Rust

#### macOS / Linux

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH (usually automatic)
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

#### Windows (WSL2)

```bash
# Inside WSL2 terminal
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart terminal or source
source $HOME/.cargo/env
```

### Step 2: Install Android NDK

#### Via Android Studio (Recommended)

1. Open Android Studio
2. Tools → SDK Manager
3. SDK Tools tab
4. Check "NDK (Side by side)"
5. Check "CMake"
6. Click "Apply" to install

#### Via Command Line

```bash
# macOS with Homebrew
brew install android-ndk

# Linux
wget https://dl.google.com/android/repository/android-ndk-r25c-linux.zip
unzip android-ndk-r25c-linux.zip -d ~/Android/
```

### Step 3: Set Environment Variables

Add to your shell profile (`.bashrc`, `.zshrc`, etc.):

```bash
# Android SDK
export ANDROID_HOME=$HOME/Library/Android/sdk  # macOS
# export ANDROID_HOME=$HOME/Android/Sdk  # Linux

# Android NDK
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653

# Add to PATH
export PATH=$PATH:$ANDROID_HOME/platform-tools
export PATH=$PATH:$ANDROID_HOME/tools
export PATH=$PATH:$ANDROID_NDK_HOME
```

Apply changes:
```bash
source ~/.zshrc  # or ~/.bashrc
```

Verify:
```bash
echo $ANDROID_HOME
echo $ANDROID_NDK_HOME
adb --version
```

### Step 4: Install Rust Android Targets

```bash
# Install Android targets
rustup target add aarch64-linux-android      # ARM64 (primary)
rustup target add armv7-linux-androideabi    # ARM32 (legacy)
rustup target add x86_64-linux-android       # Emulator (Intel)
rustup target add i686-linux-android         # Emulator (32-bit)

# Verify targets installed
rustup target list | grep android
```

### Step 5: Configure Cargo for Android

Create `~/.cargo/config.toml`:

```toml
[target.aarch64-linux-android]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android26-clang"

[target.armv7-linux-androideabi]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi26-clang"

[target.x86_64-linux-android]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android26-clang"
```

Note: Replace `darwin-x86_64` with `linux-x86_64` on Linux

### Step 6: Install Android Studio

1. Download from https://developer.android.com/studio
2. Install and complete setup wizard
3. Install recommended SDK components
4. Configure Java JDK (bundled with Android Studio)

Verify installation:
```bash
# Check Java
java -version

# Check Gradle (via wrapper)
cd android-app
./gradlew --version
```

---

## Building Rust Components

### Build for Android

```bash
# Navigate to project root
cd crisis-companion

# Build for ARM64 (primary target)
cargo build --target aarch64-linux-android --release

# Build for other targets (optional)
cargo build --target armv7-linux-androideabi --release
cargo build --target x86_64-linux-android --release
```

**Build Output:**
- ARM64: `target/aarch64-linux-android/release/libsolana_sos.so`
- ARM32: `target/armv7-linux-androideabi/release/libsolana_sos.so`
- x86_64: `target/x86_64-linux-android/release/libsolana_sos.so`

### Copy Libraries to Android Project

```bash
# Create JNI directories
mkdir -p android-app/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86_64}

# Copy libraries
cp target/aarch64-linux-android/release/libsolana_sos.so \
   android-app/app/src/main/jniLibs/arm64-v8a/

cp target/armv7-linux-androideabi/release/libsolana_sos.so \
   android-app/app/src/main/jniLibs/armeabi-v7a/

cp target/x86_64-linux-android/release/libsolana_sos.so \
   android-app/app/src/main/jniLibs/x86_64/
```

### Build for Local Testing (Development Machine)

```bash
# Build for your local machine
cargo build --release

# Run tests
cargo test

# Run examples
cargo run --example emergency_protocol_example
```

---

## Building Android App

### Option 1: Android Studio (GUI)

1. Open Android Studio
2. File → Open → Select `android-app` directory
3. Wait for Gradle sync to complete
4. Build → Make Project (⌘F9 / Ctrl+F9)
5. Build → Build Bundle(s) / APK(s) → Build APK(s)
6. Locate APK: `android-app/app/build/outputs/apk/debug/app-debug.apk`

### Option 2: Command Line (Gradle)

```bash
cd android-app

# Clean previous builds
./gradlew clean

# Build debug APK
./gradlew assembleDebug

# Build release APK (requires signing)
./gradlew assembleRelease

# Build app bundle (for Play Store)
./gradlew bundleRelease

# Run all checks
./gradlew check
```

### Build Outputs

```
android-app/app/build/outputs/
├── apk/
│   ├── debug/
│   │   └── app-debug.apk           # Debug build
│   └── release/
│       └── app-release.apk         # Release build (signed)
└── bundle/
    └── release/
        └── app-release.aab         # App bundle for stores
```

---

## Running Tests

### Rust Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_emergency_activation

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_tests

# Run examples as tests
cargo test --examples
```

### Android Tests

```bash
cd android-app

# Run unit tests
./gradlew test

# Run instrumented tests (requires device/emulator)
./gradlew connectedAndroidTest

# Run specific test
./gradlew test --tests "EmergencyServiceTest"
```

### Performance Benchmarks

```bash
# Run benchmarks (requires nightly Rust)
cargo +nightly bench

# Run specific benchmark
cargo +nightly bench voice_detection
```

---

## Installing on Device/Emulator

### Using ADB (Command Line)

```bash
# List connected devices
adb devices

# Install APK
adb install android-app/app/build/outputs/apk/debug/app-debug.apk

# Install and replace existing
adb install -r app-debug.apk

# Uninstall
adb uninstall com.solanasos.emergency

# Launch app
adb shell am start -n com.solanasos.emergency/.MainActivity

# View logs
adb logcat | grep "SolanaSOSEmergency"
```

### Using Android Studio

1. Connect device or start emulator
2. Select device from dropdown (top toolbar)
3. Click green "Run" button (▶️)
4. App installs and launches automatically

### Creating Emulator

```bash
# List available system images
sdkmanager --list | grep system-images

# Install system image
sdkmanager "system-images;android-33;google_apis;x86_64"

# Create AVD
avdmanager create avd -n Pixel_6_API_33 \
  -k "system-images;android-33;google_apis;x86_64" \
  -d "pixel_6"

# Start emulator
emulator -avd Pixel_6_API_33
```

---

## Troubleshooting

### Rust Build Issues

**Error: `ndk-sys only supports compiling for Android`**

```bash
# Ensure you're building with Android target
cargo build --target aarch64-linux-android --release
```

**Error: `linker not found`**

```bash
# Check NDK environment variable
echo $ANDROID_NDK_HOME

# Verify cargo config
cat ~/.cargo/config.toml

# Reinstall NDK via Android Studio
```

**Error: `can't find crate for 'std'`**

```bash
# Reinstall Rust target
rustup target remove aarch64-linux-android
rustup target add aarch64-linux-android
```

### Android Build Issues

**Error: `SDK location not found`**

Create `android-app/local.properties`:
```properties
sdk.dir=/Users/yourname/Library/Android/sdk
```

**Error: `Gradle sync failed`**

```bash
# Clear Gradle cache
rm -rf ~/.gradle/caches
cd android-app
./gradlew clean
```

**Error: `NDK not found`**

1. Open SDK Manager in Android Studio
2. Install NDK (Side by side)
3. Sync Gradle again

### ADB Issues

**Error: `device unauthorized`**

1. Revoke USB debugging authorization on device
2. Disconnect and reconnect USB
3. Accept authorization dialog on device

**Error: `adb: device offline`**

```bash
# Restart ADB server
adb kill-server
adb start-server
adb devices
```

### Runtime Issues

**App crashes on startup**

```bash
# Check logcat for errors
adb logcat | grep "AndroidRuntime"

# Verify native library loaded
adb logcat | grep "System.loadLibrary"
```

**Native library not found**

```bash
# Verify libraries in APK
unzip -l app-debug.apk | grep ".so"

# Should see:
# lib/arm64-v8a/libsolana_sos.so
```

---

## Development Workflow

### Typical Development Cycle

```bash
# 1. Make changes to Rust code
vim src/emergency_database.rs

# 2. Run local tests
cargo test

# 3. Build for Android
cargo build --target aarch64-linux-android --release

# 4. Copy to Android project
cp target/aarch64-linux-android/release/libsolana_sos.so \
   android-app/app/src/main/jniLibs/arm64-v8a/

# 5. Build Android app
cd android-app
./gradlew assembleDebug

# 6. Install and test
adb install -r app/build/outputs/apk/debug/app-debug.apk

# 7. Monitor logs
adb logcat | grep "SolanaSOSEmergency"
```

### Hot Reload (Android UI Changes)

Android Studio supports instant run for UI changes:
1. Make Kotlin/XML changes
2. Click "Apply Changes" (⌃⌘R / Ctrl+Shift+F10)
3. Changes apply without reinstall

### Performance Profiling

```bash
# Profile CPU usage
adb shell am profile start com.solanasos.emergency /data/local/tmp/trace.trace

# Stop profiling
adb shell am profile stop com.solanasos.emergency

# Pull trace
adb pull /data/local/tmp/trace.trace

# Analyze in Android Studio
# Profiler → Load from file
```

---

## CI/CD Integration

### GitHub Actions Example

Create `.github/workflows/build.yml`:

```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: cargo build --release

  android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: android-actions/setup-android@v2
      - run: cd android-app && ./gradlew build
```

---

## Build Optimization

### Faster Rust Builds

```bash
# Use mold linker (Linux)
cargo install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"

# Parallel compilation
export CARGO_BUILD_JOBS=8

# Incremental compilation
export CARGO_INCREMENTAL=1
```

### Faster Gradle Builds

Add to `gradle.properties`:
```properties
org.gradle.jvmargs=-Xmx4g -XX:MaxMetaspaceSize=512m
org.gradle.parallel=true
org.gradle.caching=true
android.enableJetifier=false
android.useAndroidX=true
```

---

## Release Builds

### 1. Update Version

`android-app/app/build.gradle`:
```gradle
defaultConfig {
    versionCode 2
    versionName "1.1.0"
}
```

### 2. Build Release

```bash
# Build Rust (release mode)
cargo build --target aarch64-linux-android --release

# Copy libraries
./scripts/copy-libs.sh

# Build Android (release)
cd android-app
./gradlew assembleRelease
```

### 3. Sign APK

```bash
jarsigner -verbose -sigalg SHA256withRSA -digestalg SHA-256 \
  -keystore release.keystore \
  app/build/outputs/apk/release/app-release-unsigned.apk \
  solana-sos

zipalign -v 4 \
  app-release-unsigned.apk \
  app-release.apk
```

### 4. Verify

```bash
# Verify signature
jarsigner -verify -verbose -certs app-release.apk

# Check size
ls -lh app-release.apk
```

---

## Additional Resources

- **API Documentation:** `/docs/API.md`
- **Architecture:** `/docs/ARCHITECTURE.md`
- **Deployment Guide:** `/docs/DEPLOYMENT.md`
- **Testing Strategy:** `/docs/TESTING_STRATEGY.md`
- **Performance:** `/docs/PERFORMANCE.md`

## Support

Need help?
- GitHub Issues: https://github.com/paragoner1/crisis-companion/issues
- X: @paragoner1

---

**Remember:** Building for life-critical emergency response software requires careful attention to detail. Test thoroughly before deployment.

