#!/bin/bash

# Build script for Solana SOS Rust library for Android
# This script compiles the Rust backend for Android JNI integration

set -e

echo "🔧 Building Solana SOS Rust library for Android..."

# Set up Android NDK paths (adjust these for your system)
export ANDROID_NDK_HOME=${ANDROID_NDK_HOME:-$HOME/Library/Android/sdk/ndk/29.0.13846066}
export ANDROID_SDK_HOME=${ANDROID_SDK_HOME:-$HOME/Library/Android/sdk}

echo "📱 NDK Home: $ANDROID_NDK_HOME"
echo "📱 SDK Home: $ANDROID_SDK_HOME"

# Set up NDK toolchain paths
export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin:$PATH

# Target architectures for Android
TARGETS=("aarch64-linux-android" "armv7-linux-androideabi" "i686-linux-android" "x86_64-linux-android")

# Create output directory
mkdir -p android-app/app/src/main/jniLibs

# Build for each target architecture
for target in "${TARGETS[@]}"; do
    echo "📱 Building for $target..."
    
    # Set up environment variables for this target
    case $target in
        "aarch64-linux-android")
            export CC_aarch64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang
            export CXX_aarch64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang++
            export AR_aarch64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
            export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android21-clang
            ;;
        "armv7-linux-androideabi")
            export CC_armv7_linux_androideabi=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang
            export CXX_armv7_linux_androideabi=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang++
            export AR_armv7_linux_androideabi=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
            export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi21-clang
            ;;
        "i686-linux-android")
            export CC_i686_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android21-clang
            export CXX_i686_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android21-clang++
            export AR_i686_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
            export CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android21-clang
            ;;
        "x86_64-linux-android")
            export CC_x86_64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android21-clang
            export CXX_x86_64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android21-clang++
            export AR_x86_64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
            export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android21-clang
            ;;
    esac
    
    # Install target if not already installed
    rustup target add $target
    
    # Build the library
    echo "🔨 Building for $target..."
    cargo build --target $target --release --lib
    
    # Determine the correct library name and path
    case $target in
        "aarch64-linux-android")
            lib_name="libsolana_sos.so"
            lib_path="../target/aarch64-linux-android/release/$lib_name"
            dest_dir="android-app/app/src/main/jniLibs/arm64-v8a"
            ;;
        "armv7-linux-androideabi")
            lib_name="libsolana_sos.so"
            lib_path="../target/armv7-linux-androideabi/release/$lib_name"
            dest_dir="android-app/app/src/main/jniLibs/armeabi-v7a"
            ;;
        "i686-linux-android")
            lib_name="libsolana_sos.so"
            lib_path="../target/i686-linux-android/release/$lib_name"
            dest_dir="android-app/app/src/main/jniLibs/x86"
            ;;
        "x86_64-linux-android")
            lib_name="libsolana_sos.so"
            lib_path="../target/x86_64-linux-android/release/$lib_name"
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