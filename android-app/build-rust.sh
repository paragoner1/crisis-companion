#!/bin/bash

# Build script for Solana SOS Rust library for Android
# This script compiles the Rust backend for Android JNI integration

set -e

echo "🔧 Building Solana SOS Rust library for Android..."

# Set up Android NDK paths (adjust these for your system)
export ANDROID_NDK_HOME=${ANDROID_NDK_HOME:-$HOME/Library/Android/sdk/ndk}
export ANDROID_SDK_HOME=${ANDROID_SDK_HOME:-$HOME/Library/Android/sdk}

# Target architectures for Android
TARGETS=("aarch64-linux-android" "armv7-linux-androideabi" "i686-linux-android" "x86_64-linux-android")

# Create output directory
mkdir -p android-app/app/src/main/jniLibs

# Build for each target architecture
for target in "${TARGETS[@]}"; do
    echo "📱 Building for $target..."
    
    # Install target if not already installed
    rustup target add $target
    
    # Build the library
    cargo build --target $target --release --lib
    
    # Determine the correct library name and path
    case $target in
        "aarch64-linux-android")
            lib_name="libsolana_sos.so"
            lib_path="target/aarch64-linux-android/release/$lib_name"
            dest_dir="android-app/app/src/main/jniLibs/arm64-v8a"
            ;;
        "armv7-linux-androideabi")
            lib_name="libsolana_sos.so"
            lib_path="target/armv7-linux-androideabi/release/$lib_name"
            dest_dir="android-app/app/src/main/jniLibs/armeabi-v7a"
            ;;
        "i686-linux-android")
            lib_name="libsolana_sos.so"
            lib_path="target/i686-linux-android/release/$lib_name"
            dest_dir="android-app/app/src/main/jniLibs/x86"
            ;;
        "x86_64-linux-android")
            lib_name="libsolana_sos.so"
            lib_path="target/x86_64-linux-android/release/$lib_name"
            dest_dir="android-app/app/src/main/jniLibs/x86_64"
            ;;
    esac
    
    # Create destination directory
    mkdir -p "$dest_dir"
    
    # Copy the library
    if [ -f "$lib_path" ]; then
        cp "$lib_path" "$dest_dir/"
        echo "✅ Copied $lib_name to $dest_dir"
    else
        echo "❌ Library not found at $lib_path"
        exit 1
    fi
done

echo "🎉 Rust library build complete!"
echo "📱 Libraries copied to android-app/app/src/main/jniLibs/"

# Show the built libraries
echo "📋 Built libraries:"
find android-app/app/src/main/jniLibs -name "*.so" -exec ls -la {} \;

echo ""
echo "🚀 Next steps:"
echo "1. Open android-app/ in Android Studio"
echo "2. Build and run the Android app"
echo "3. Test voice recognition and emergency features" 