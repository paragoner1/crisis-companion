# 🚨 Solana SOS - Android App

## Overview

This Android app provides the mobile interface for Solana SOS, the voice-activated emergency response application. It integrates with the Rust backend via JNI and implements the Solana Mobile Stack for blockchain functionality.

## Features

### ✅ Implemented
- **Voice Recognition** - Android SpeechRecognizer for "Hey SOS" wake word
- **Emergency UI** - Touch-friendly emergency interface
- **Emergency Types** - Support for 12 emergency types
- **Permissions** - Audio, location, phone, SMS permissions
- **Solana Integration** - Mobile Wallet Adapter ready

### 🚧 In Progress
- **JNI Integration** - Connect to Rust backend
- **Emergency Service** - Background emergency handling
- **Location Services** - GPS integration
- **Solana Mobile Stack** - Full blockchain integration

## Architecture

```
Android App (Kotlin)
├── MainActivity.kt          # Main UI and voice recognition
├── EmergencyService.kt      # Background emergency handling
├── VoiceRecognitionService.kt # Voice processing
└── SolanaIntegration.kt    # Blockchain functionality

Rust Backend (JNI)
├── Voice recognition algorithms
├── Emergency response logic
├── Database operations
└── Blockchain transactions
```

## Setup

### Prerequisites
- Android Studio
- Android SDK (API 24+)
- Solana Mobile Stack SDK
- Rust toolchain (for JNI)

### Build Instructions

1. **Clone the repository**
   ```bash
   git clone https://github.com/paragoner1/crisis-companion.git
   cd crisis-companion/android-app
   ```

2. **Open in Android Studio**
   ```bash
   android-studio .
   ```

3. **Build the app**
   ```bash
   ./gradlew assembleDebug
   ```

4. **Install on device**
   ```bash
   adb install app/build/outputs/apk/debug/app-debug.apk
   ```

## Usage

### Voice Activation
1. Tap "Start Voice Recognition"
2. Say "Hey SOS" to activate
3. State the emergency type (drowning, heart attack, etc.)
4. Follow emergency instructions

### Manual Activation
1. Tap "Emergency Activation" button
2. Select emergency type from options
3. Follow provided instructions

### Solana Integration
1. Tap "Connect Solana Wallet"
2. Authorize with your Solana wallet
3. Earn SKR tokens for emergency preparedness

## Emergency Types Supported

- **Drowning** - Water rescue and post-extraction care
- **Heart Attack** - CPR and emergency response
- **Stroke** - FAST test and immediate care
- **Choking** - Heimlich maneuver and airway clearance
- **Bleeding** - Direct pressure and tourniquet application
- **Unconscious** - Assessment and basic life support
- **Seizure** - Safety measures and monitoring
- **Poisoning** - Poison control and emergency care
- **Severe Burns** - Cooling and emergency treatment
- **Diabetic Emergency** - Blood sugar management
- **Allergic Reaction** - EpiPen administration
- **Trauma** - Assessment and stabilization

## Permissions

The app requires the following permissions:
- **RECORD_AUDIO** - Voice recognition
- **ACCESS_FINE_LOCATION** - Emergency location sharing
- **CALL_PHONE** - Emergency 911 calls
- **SEND_SMS** - Emergency contact alerts
- **INTERNET** - Solana blockchain connectivity

## Development

### Adding New Emergency Types
1. Add emergency type to `MainActivity.kt` voice recognition
2. Add emergency instructions to `getEmergencyInstructions()`
3. Update UI layout if needed

### Integrating with Rust Backend
1. Create JNI bindings in Rust
2. Call Rust functions from Kotlin
3. Handle data conversion between languages

### Solana Mobile Stack Integration
1. Implement Mobile Wallet Adapter
2. Add SKR token reward system
3. Store emergency records on blockchain

## Testing

### Voice Recognition Testing
```bash
# Test wake word detection
adb shell input text "Hey SOS"

# Test emergency phrase recognition
adb shell input text "drowning help"
```

### Emergency Flow Testing
1. Activate emergency mode
2. Verify 911 call simulation
3. Test location sharing
4. Validate emergency instructions

## Deployment

### dApp Store Preparation
1. **Code signing** - Generate release APK
2. **Privacy policy** - Emergency app requirements
3. **Terms of service** - Legal compliance
4. **Emergency disclaimers** - Medical/legal protection

### Production Checklist
- [ ] Emergency protocols validated by medical professionals
- [ ] 911 integration tested with emergency services
- [ ] Privacy policy reviewed by legal counsel
- [ ] Solana Mobile Stack fully integrated
- [ ] Performance optimized for emergency scenarios
- [ ] Security audit completed

## Contributing

This Android app is part of the Solana SOS emergency response system. Contributions should focus on:

- **Emergency response accuracy** - Medical protocol validation
- **Voice recognition reliability** - Noise handling, accent support
- **Mobile UX optimization** - Emergency scenario usability
- **Solana integration** - Blockchain functionality
- **Accessibility** - Emergency access for all users

## License

Apache 2.0 License - See main repository for details.

---

**🚨 Emergency Disclaimer:** This app provides guidance for emergency situations but does not replace professional medical care. Always call 911 in life-threatening emergencies. 