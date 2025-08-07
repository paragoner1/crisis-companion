# 🚀 Production Setup Guide: Real Vosk & MWA Integration

## **📋 Complete Process for Getting Out of Demo Mode**

### **Step 1: Real Vosk Model Setup**

#### **1.1 Download Vosk Model (COMPLETED)**
```bash
# Already done - model downloaded to models/vosk-model-small-en-us-0.15/
cd models
curl -O https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.tar.gz
tar -xzf vosk-model-small-en-us-0.15.tar.gz
```

#### **1.2 Implement Real Vosk Recognition**
```rust
// In src/public/voice_interface.rs
// Replace the simulated recognition with real Vosk:

use vosk::{Model, Recognizer};

pub struct VoiceInterface {
    vosk_model: Option<Model>,
    vosk_recognizer: Option<Recognizer>,
    // ... other fields
}

impl VoiceInterface {
    pub async fn initialize(&mut self) -> AppResult<()> {
        // Load real Vosk model
        let model = Model::new(&self.model_path)
            .ok_or_else(|| AppError::Voice("Failed to load Vosk model".to_string()))?;
        
        // Create recognizer
        let recognizer = Recognizer::new(&model, self.config.sample_rate as f32)
            .ok_or_else(|| AppError::Voice("Failed to create recognizer".to_string()))?;
        
        self.vosk_model = Some(model);
        self.vosk_recognizer = Some(recognizer);
        
        Ok(())
    }
    
    fn recognize_speech(&self, audio_data: &[u8]) -> AppResult<String> {
        if let (Some(_model), Some(recognizer)) = (&self.vosk_model, &self.vosk_recognizer) {
            // Convert audio to PCM samples
            let samples = self.convert_to_pcm(audio_data)?;
            
            // Use real Vosk recognition
            recognizer.accept_waveform(&samples);
            let result = recognizer.result();
            
            // Parse JSON result
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&result.to_json()) {
                if let Some(text) = json["text"].as_str() {
                    return Ok(text.to_string());
                }
            }
        }
        
        // Fallback to pattern recognition
        self.fallback_recognition(audio_data)
    }
}
```

#### **1.3 Test Real Vosk Integration**
```bash
# Build and test
cargo build
cargo run --bin performance_test
```

### **Step 2: Real Solana Mobile SDK Setup**

#### **2.1 Add Solana Mobile Dependencies (COMPLETED)**
```gradle
// In android-app/app/build.gradle
dependencies {
    // Real Solana Mobile Stack
    implementation 'com.solana:mobile-wallet-adapter:1.0.0'
    implementation 'com.solana:solana-sdk:1.0.0'
    implementation 'com.solana:mobile-wallet-adapter-client:1.0.0'
    implementation 'com.solana:mobile-wallet-adapter-server:1.0.0'
}
```

#### **2.2 Implement Real MWA Connection**
```kotlin
// In MobileWalletAdapter.kt
import com.solana.mobilewalletadapter.MobileWalletAdapterClient
import com.solana.sdk.SolanaConnection

class MobileWalletAdapter(private val context: Context) {
    
    suspend fun connectWallet(): Boolean {
        return withContext(Dispatchers.IO) {
            try {
                // Real Mobile Wallet Adapter
                val walletAdapter = MobileWalletAdapterClient.Builder()
                    .setCluster(SOLANA_CLUSTER)
                    .build()
                
                val connection = walletAdapter.connect()
                val walletAddress = connection.getWalletAddress()
                
                connectedWallet = WalletInfo(
                    address = walletAddress,
                    publicKey = walletAddress,
                    cluster = SOLANA_CLUSTER,
                    connectedAt = System.currentTimeMillis()
                )
                
                isConnected = true
                Log.d(TAG, "Real Solana Mobile wallet connected: $walletAddress")
                true
            } catch (e: Exception) {
                Log.e(TAG, "Failed to connect to Solana Mobile wallet", e)
                false
            }
        }
    }
    
    suspend fun createTokenTransaction(tokenMint: String, amount: Int, recipient: String): String? {
        return withContext(Dispatchers.IO) {
            try {
                // Real Solana transaction
                val connection = SolanaConnection(SOLANA_RPC_URL)
                val transaction = Transaction()
                
                // Create token transfer instruction
                val transferInstruction = TokenTransferInstruction(
                    tokenMint = tokenMint,
                    amount = amount,
                    recipient = recipient
                )
                
                transaction.add(transferInstruction)
                
                // Sign and send transaction
                val signature = connection.sendTransaction(transaction)
                
                Log.d(TAG, "Real transaction sent: $signature")
                signature
            } catch (e: Exception) {
                Log.e(TAG, "Failed to create transaction", e)
                null
            }
        }
    }
}
```

### **Step 3: Production Testing**

#### **3.1 Test Real Voice Recognition**
```bash
# Test with real audio
cargo run --bin voice_test
```

#### **3.2 Test Real MWA Integration**
```bash
# Build and install Android app
cd android-app
./gradlew assembleDebug
adb install -r app/build/outputs/apk/debug/app-debug.apk
```

#### **3.3 Performance Validation**
```bash
# Run comprehensive tests
cargo run --bin performance_test
cargo run --bin edge_case_test
```

### **Step 4: Production Deployment**

#### **4.1 Environment Setup**
```bash
# Production environment variables
export VOSK_MODEL_PATH="models/vosk-model-small-en-us-0.15"
export SOLANA_RPC_URL="https://api.mainnet-beta.solana.com"
export SOLANA_CLUSTER="mainnet-beta"
```

#### **4.2 Build for Production**
```bash
# Rust backend
cargo build --release

# Android app
cd android-app
./gradlew assembleRelease
```

#### **4.3 Deploy to dApp Store**
```bash
# Prepare for Solana Mobile dApp Store
# 1. Test on Seeker device
# 2. Validate all emergency protocols
# 3. Verify voice recognition accuracy
# 4. Confirm MWA integration
# 5. Submit to dApp Store
```

## **🎯 Current Status:**

### **✅ Vosk Integration:**
- ✅ **Model Downloaded** - Real Vosk model available
- ✅ **Structure Ready** - Real Vosk integration framework
- ⚠️ **Implementation** - Need to complete real recognition
- ⚠️ **Testing** - Need to validate with real audio

### **✅ MWA Integration:**
- ✅ **Dependencies Added** - Solana Mobile SDK ready
- ✅ **Structure Ready** - Real MWA integration framework
- ⚠️ **Implementation** - Need to complete real connection
- ⚠️ **Testing** - Need to validate with real wallet

## **🚀 Next Steps:**

1. **Complete Real Vosk Recognition** - Implement actual speech recognition
2. **Complete Real MWA Connection** - Implement actual wallet connection
3. **Test with Real Audio** - Validate voice recognition accuracy
4. **Test with Real Wallet** - Validate MWA integration
5. **Performance Optimization** - Ensure sub-100ms response
6. **Production Deployment** - Deploy to dApp Store

**The framework is ready - you just need to complete the real implementations!** 🎯 