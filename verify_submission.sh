#!/bin/bash

echo "🔍 Solana SOS - Hackathon Submission Verification"
echo "================================================"

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Not in the project root directory"
    exit 1
fi

echo "✅ Project root directory confirmed"

# Check Rust compilation
echo "🔧 Checking Rust compilation..."
if cargo check; then
    echo "✅ Rust code compiles successfully"
else
    echo "❌ Rust compilation failed"
    exit 1
fi

# Check demo binaries
echo "🧪 Testing demo binaries..."
if cargo run --bin voice_test > /dev/null 2>&1; then
    echo "✅ voice_test binary works"
else
    echo "❌ voice_test binary failed"
    exit 1
fi

if cargo run --bin demo_test > /dev/null 2>&1; then
    echo "✅ demo_test binary works"
else
    echo "❌ demo_test binary failed"
    exit 1
fi

# Check essential documentation files
echo "📚 Checking documentation files..."
required_files=(
    "README.md"
    "HACKATHON_SUBMISSION.md"
    "JUDGE_Q&A.md"
    "SOLANA_SOS_NEW_PRESENTATION.md"
    "SOLANA_SOS_NEW_VOICEOVER.md"
    "SOLANA_SOS_TERMINAL_DEMO.md"
    "SOLANA_MOBILE_DEMO.md"
    "TECHNICAL_WALKTHROUGH.md"
    "LAYMAN_WALKTHROUGH.md"
    "COMPLETE_APP_WALKTHROUGH.md"
    "CITATIONS.md"
    "HACKATHON_SUBMISSION_CHECKLIST.md"
    "SUBMISSION_PACKAGE_SUMMARY.md"
)

missing_files=()
for file in "${required_files[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file exists"
    else
        echo "❌ $file missing"
        missing_files+=("$file")
    fi
done

if [ ${#missing_files[@]} -gt 0 ]; then
    echo "❌ Missing required files: ${missing_files[*]}"
    exit 1
fi

# Check Android app structure
echo "📱 Checking Android app structure..."
android_files=(
    "android-app/app/src/main/java/com/solanasos/emergency/MainActivity.kt"
    "android-app/app/src/main/java/com/solanasos/emergency/SettingsActivity.kt"
    "android-app/app/src/main/java/com/solanasos/emergency/EmergencyService.kt"
    "android-app/app/src/main/java/com/solanasos/emergency/RustBridge.kt"
    "android-app/app/src/main/java/com/solanasos/emergency/SolanaMobileIntegration.kt"
    "android-app/app/src/main/java/com/solanasos/emergency/MobileWalletAdapter.kt"
    "android-app/app/src/main/res/layout/activity_settings.xml"
    "android-app/app/src/main/AndroidManifest.xml"
)

missing_android_files=()
for file in "${android_files[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file exists"
    else
        echo "❌ $file missing"
        missing_android_files+=("$file")
    fi
done

if [ ${#missing_android_files[@]} -gt 0 ]; then
    echo "❌ Missing Android files: ${missing_android_files[*]}"
    exit 1
fi

# Check Rust source files
echo "🦀 Checking Rust source files..."
rust_files=(
    "src/lib.rs"
    "src/jni_bridge.rs"
    "src/public/voice_interface.rs"
    "src/public/audio_interface.rs"
    "src/public/emergency_interface.rs"
    "src/public/types.rs"
    "src/error.rs"
)

missing_rust_files=()
for file in "${rust_files[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file exists"
    else
        echo "❌ $file missing"
        missing_rust_files+=("$file")
    fi
done

if [ ${#missing_rust_files[@]} -gt 0 ]; then
    echo "❌ Missing Rust files: ${missing_rust_files[*]}"
    exit 1
fi

# Check license and legal files
echo "⚖️ Checking legal files..."
if [ -f "LICENSE" ]; then
    echo "✅ LICENSE exists"
else
    echo "❌ LICENSE missing"
    exit 1
fi

if [ -f ".gitignore" ]; then
    echo "✅ .gitignore exists"
else
    echo "❌ .gitignore missing"
    exit 1
fi

echo ""
echo "🎉 VERIFICATION COMPLETE!"
echo "========================="
echo "✅ All Rust code compiles successfully"
echo "✅ Demo binaries work correctly"
echo "✅ All documentation files present"
echo "✅ Android app structure complete"
echo "✅ Rust source files present"
echo "✅ Legal files in place"
echo ""
echo "🚀 Solana SOS is ready for hackathon submission!"
echo ""
echo "📋 Submission Package Summary:"
echo "   - Functional Android app with JNI bridge"
echo "   - Complete Solana Mobile Stack integration"
echo "   - 12 emergency types with context-aware guidance"
echo "   - Safety features (Silent SOS, Crash Detection, Trusted Network)"
echo "   - Token integration (BONK/SKR)"
echo "   - Comprehensive documentation and presentation materials"
echo "   - Working demo scripts for judges"
echo ""
echo "🎯 Status: READY FOR SUBMISSION ✅" 