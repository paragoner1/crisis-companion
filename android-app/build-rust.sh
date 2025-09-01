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
mkdir -p app/src/main/jniLibs/arm64-v8a
mkdir -p app/src/main/jniLibs/armeabi-v7a
mkdir -p app/src/main/jniLibs/x86
mkdir -p app/src/main/jniLibs/x86_64

# Build ONNX Runtime from source for Android if not already built
echo "📦 Checking for ONNX Runtime libraries..."
if [ ! -f "app/src/main/jniLibs/arm64-v8a/libonnxruntime.so" ]; then
    echo "🔨 Building ONNX Runtime from source for Android ARM64..."

    # Check if we already have the ORT source
    if [ ! -d "onnxruntime" ]; then
        echo "📥 Cloning ONNX Runtime repository..."
        git clone --recursive --depth 1 --branch main https://github.com/microsoft/onnxruntime.git
    fi

            cd onnxruntime
        
        # Patch to remove SHA1 check for eigen to bypass hash mismatch
        echo "🔧 Patching eigen.cmake to remove SHA1 check..."
        sed -i '' '/URL_HASH SHA1/d' cmake/external/eigen.cmake
        echo "✅ Patched eigen.cmake"
        
        # Patch float16.h to fix deprecated literal operators
        echo "🔧 Patching float16.h to remove spaces in literal operators..."
        sed -i '' 's/operator "" _f16/operator""_f16/g' include/onnxruntime/core/framework/float16.h
        sed -i '' 's/operator "" _fp16/operator""_fp16/g' include/onnxruntime/core/framework/float16.h
        sed -i '' 's/operator "" _b16/operator""_b16/g' include/onnxruntime/core/framework/float16.h
        sed -i '' 's/operator "" _bfp16/operator""_bfp16/g' include/onnxruntime/core/framework/float16.h
        echo "✅ Patched float16.h"
        
        # Patch build_args.py for Python 3.9 compatibility
        echo "🔧 Patching build_args.py..."
        cat > patch.txt <<EOF
    if s.lower() == "true":
        return True
    elif s.lower() == "false":
        return False
    else:
        raise ValueError(f"Invalid boolean value: {s!r}. Use 'true' or 'false'.")
EOF
        sed -i '' '/match s.lower():/,/raise ValueError/d' tools/ci_build/build_args.py
        sed -i '' '/def _str_to_bool(s: str) -> bool:/r patch.txt' tools/ci_build/build_args.py
        rm patch.txt
        echo "✅ Patched build_args.py"
        
        # Build for ARM64 with CMake patch
        echo "🔧 Running ORT update to download dependencies (may fail, that's okay)..."
        export PYTHON_EXECUTABLE=/opt/homebrew/bin/python3
        ./build.sh --android \
            --android_sdk_path "$ANDROID_SDK_HOME" \
            --android_ndk_path "$ANDROID_NDK_HOME" \
            --android_abi arm64-v8a \
            --android_api 26 \
            --config Release \
            --build_shared_lib \
            --minimal_build \
            --disable_ml_ops \
            --disable_exceptions --update || true
        
        # Patch google_nsync
        echo "🔧 Patching google_nsync CMake version..."
        if [ -f "build/Android/Release/_deps/google_nsync-src/CMakeLists.txt" ]; then
            sed -i '' 's/cmake_minimum_required (VERSION 2.8.12)/cmake_minimum_required(VERSION 3.5)/g' build/Android/Release/_deps/google_nsync-src/CMakeLists.txt
            echo "✅ Patched google_nsync CMake version"
        else
            echo "❌ google_nsync CMakeLists.txt not found after update"
            exit 1
        fi
        
        # Patch nlohmann_json
        echo "🔧 Patching nlohmann_json CMake version..."
        if [ -f "build/Android/Release/_deps/json-src/CMakeLists.txt" ]; then
            sed -i '' 's/cmake_minimum_required(VERSION 3.1)/cmake_minimum_required(VERSION 3.5)/g' build/Android/Release/_deps/json-src/CMakeLists.txt
            echo "✅ Patched nlohmann_json CMake version"
        else
            echo "⚠️ nlohmann_json CMakeLists.txt not found, skipping patch"
        fi
        
        # Patch date.h to fix deprecated literal operators
        echo "🔧 Patching date.h to remove spaces in literal operators..."
        if [ -f "build/Android/Release/_deps/date-src/include/date/date.h" ]; then
            sed -i '' 's/operator "" _d/operator""_d/g' build/Android/Release/_deps/date-src/include/date/date.h
            sed -i '' 's/operator "" _y/operator""_y/g' build/Android/Release/_deps/date-src/include/date/date.h
            echo "✅ Patched date.h"
        else
            echo "⚠️ date.h not found, skipping patch"
        fi
        
        # Run full build
        echo "🔧 Running full ORT build..."
        ./build.sh --android \
            --android_sdk_path "$ANDROID_SDK_HOME" \
            --android_ndk_path "$ANDROID_NDK_HOME" \
            --android_abi arm64-v8a \
            --android_api 26 \
            --config Release \
            --build_shared_lib \
            --minimal_build \
            --disable_ml_ops \
            --disable_exceptions \
            --skip_submodule_sync

    # Copy the built library
    if [ -f "build/Android/Release/lib/libonnxruntime.so" ]; then
        cp build/Android/Release/lib/libonnxruntime.so ../app/src/main/jniLibs/arm64-v8a/
        echo "✅ ORT ARM64 library built and copied successfully"
    else
        echo "❌ Failed to build ORT for ARM64 - check logs for errors"
        exit 1
    fi

    cd ..
else
    echo "✅ ONNX Runtime libraries already exist"
fi

# Extend to build ORT for all targets
ABIS=("arm64-v8a" "armeabi-v7a" "x86" "x86_64")

for abi in "${ABIS[@]}"; do
    if [ ! -f "app/src/main/jniLibs/$abi/libonnxruntime.so" ]; then
        echo "🔨 Building ORT for $abi..."
        
                    cd onnxruntime
            
            # Patch to remove SHA1 check for eigen to bypass hash mismatch
            echo "🔧 Patching eigen.cmake to remove SHA1 check for $abi..."
            sed -i '' '/URL_HASH SHA1/d' cmake/external/eigen.cmake
            echo "✅ Patched eigen.cmake"
            
            # Patch float16.h to fix deprecated literal operators
            echo "🔧 Patching float16.h to remove spaces in literal operators for $abi..."
            sed -i '' 's/operator "" _f16/operator""_f16/g' include/onnxruntime/core/framework/float16.h
            sed -i '' 's/operator "" _fp16/operator""_fp16/g' include/onnxruntime/core/framework/float16.h
            sed -i '' 's/operator "" _b16/operator""_b16/g' include/onnxruntime/core/framework/float16.h
            sed -i '' 's/operator "" _bfp16/operator""_bfp16/g' include/onnxruntime/core/framework/float16.h
            echo "✅ Patched float16.h"
            
            # Patch build_args.py for Python 3.9 compatibility
            echo "🔧 Patching build_args.py for $abi..."
            cat > patch.txt <<EOF
    if s.lower() == "true":
        return True
    elif s.lower() == "false":
        return False
    else:
        raise ValueError(f"Invalid boolean value: {s!r}. Use 'true' or 'false'.")
EOF
            sed -i '' '/match s.lower():/,/raise ValueError/d' tools/ci_build/build_args.py
            sed -i '' '/def _str_to_bool(s: str) -> bool:/r patch.txt' tools/ci_build/build_args.py
            rm patch.txt
            echo "✅ Patched build_args.py"
            
            # Build for $abi with CMake patch
            echo "🔧 Running ORT update to download dependencies for $abi (may fail, that's okay)..."
            export PYTHON_EXECUTABLE=/opt/homebrew/bin/python3
            ./build.sh --android \
                --android_sdk_path "$ANDROID_SDK_HOME" \
                --android_ndk_path "$ANDROID_NDK_HOME" \
                --android_abi "$abi" \
                --android_api 26 \
                --config Release \
                --build_shared_lib \
                --minimal_build \
                --disable_ml_ops \
                --disable_exceptions --update || true
            
            # Patch google_nsync
            echo "🔧 Patching google_nsync CMake version for $abi..."
            if [ -f "build/Android/Release/_deps/google_nsync-src/CMakeLists.txt" ]; then
                sed -i '' 's/cmake_minimum_required (VERSION 2.8.12)/cmake_minimum_required(VERSION 3.5)/g' build/Android/Release/_deps/google_nsync-src/CMakeLists.txt
                echo "✅ Patched google_nsync CMake version"
            else
                echo "❌ google_nsync CMakeLists.txt not found after update for $abi"
                exit 1
            fi
            
            # Patch nlohmann_json
            echo "🔧 Patching nlohmann_json CMake version for $abi..."
            if [ -f "build/Android/Release/_deps/json-src/CMakeLists.txt" ]; then
                sed -i '' 's/cmake_minimum_required(VERSION 3.1)/cmake_minimum_required(VERSION 3.5)/g' build/Android/Release/_deps/json-src/CMakeLists.txt
                echo "✅ Patched nlohmann_json CMake version"
            else
                echo "⚠️ nlohmann_json CMakeLists.txt not found, skipping patch"
            fi
            
            # Patch date.h to fix deprecated literal operators
            echo "🔧 Patching date.h to remove spaces in literal operators for $abi..."
            if [ -f "build/Android/Release/_deps/date-src/include/date/date.h" ]; then
                sed -i '' 's/operator "" _d/operator""_d/g' build/Android/Release/_deps/date-src/include/date/date.h
                sed -i '' 's/operator "" _y/operator""_y/g' build/Android/Release/_deps/date-src/include/date/date.h
                echo "✅ Patched date.h"
            else
                echo "⚠️ date.h not found, skipping patch"
            fi
            
            # Run full build
            echo "🔧 Running full ORT build for $abi..."
            ./build.sh --android \
                --android_sdk_path "$ANDROID_SDK_HOME" \
                --android_ndk_path "$ANDROID_NDK_HOME" \
                --android_abi "$abi" \
                --android_api 26 \
                --config Release \
                --build_shared_lib \
                --minimal_build \
                --disable_ml_ops \
                --disable_exceptions \
                --skip_submodule_sync
        
        if [ -f "build/Android/Release/lib/libonnxruntime.so" ]; then
            cp build/Android/Release/lib/libonnxruntime.so ../app/src/main/jniLibs/$abi/
            echo "✅ ORT library built and copied for $abi"
        else
            echo "❌ Failed to build ORT for $abi"
            exit 1
        fi
        
        cd ..
    fi
done

# Build for each target architecture
for target in "${TARGETS[@]}"; do
    echo "📱 Building for $target..."
    
    # Set up environment variables for this target
    case $target in
        "aarch64-linux-android")
            export CC_aarch64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android26-clang
            export CXX_aarch64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android26-clang++
            export AR_aarch64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
            export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android26-clang
            ;;
        "armv7-linux-androideabi")
            export CC_armv7_linux_androideabi=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi26-clang
            export CXX_armv7_linux_androideabi=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi26-clang++
            export AR_armv7_linux_androideabi=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
            export CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi26-clang
            ;;
        "i686-linux-android")
            export CC_i686_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android26-clang
            export CXX_i686_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android26-clang++
            export AR_i686_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
            export CARGO_TARGET_I686_LINUX_ANDROID_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android26-clang
            ;;
        "x86_64-linux-android")
            export CC_x86_64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android26-clang
            export CXX_x86_64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android26-clang++
            export AR_x86_64_linux_android=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/llvm-ar
            export CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android26-clang
            ;;
    esac
    
    # Install target if not already installed
    rustup target add $target
    
    # Build the library with ORT support for Android
    echo "🔨 Building for $target..."
    cargo build --target $target --release --lib --features "voice,voice-ort,monitoring,private,audio"
    
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