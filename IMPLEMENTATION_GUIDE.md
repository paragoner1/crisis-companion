# 🛠️ Implementation Guide: Phase 1 & Phase 2

## 📱 **Phase 1: Android Native Implementation**

### **1.1 Replace Simulated Voice Recognition with Real Microphone**

**Current (Simulated):**
```rust
// In src/voice.rs - Current simulated implementation
async fn listening_loop(
    config: VoiceConfig,
    emergency_phrase_map: HashMap<String, EmergencyType>,
    trigger_sender: mpsc::Sender<VoiceTriggerResult>,
    is_listening: Arc<Mutex<bool>>,
) -> AppResult<()> {
    // Simulate audio processing
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Simulate raw audio data (in real app, this would come from microphone)
    let simulated_raw_audio = vec![0.1f32; 480]; // 480 samples for RNNoise
}
```

**Real Implementation:**
```rust
// New file: src/android/voice_recognition.rs
use android_audio::AudioRecord;
use android_audio::AudioManager;
use android_audio::AudioSource;

pub struct AndroidVoiceRecognition {
    audio_record: AudioRecord,
    sample_rate: u32,
    buffer_size: usize,
}

impl AndroidVoiceRecognition {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sample_rate = 16000; // Standard for voice recognition
        let buffer_size = 480; // RNNoise frame size
        
        let audio_record = AudioRecord::new(
            AudioSource::MIC,
            sample_rate,
            AudioFormat::PCM_16BIT,
            buffer_size * 2, // 16-bit = 2 bytes per sample
        )?;
        
        Ok(Self {
            audio_record,
            sample_rate,
            buffer_size,
        })
    }
    
    pub async fn start_listening(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.audio_record.start_recording()?;
        
        let mut buffer = vec![0i16; self.buffer_size];
        
        loop {
            // Read real audio from microphone
            let bytes_read = self.audio_record.read(&mut buffer)?;
            
            if bytes_read > 0 {
                // Convert to f32 for RNNoise processing
                let audio_samples: Vec<f32> = buffer.iter()
                    .map(|&sample| sample as f32 / 32768.0)
                    .collect();
                
                // Process with RNNoise
                let filtered_audio = self.noise_filter.process_audio(&audio_samples).await?;
                
                // Send to voice recognition
                self.process_voice_input(&filtered_audio).await?;
            }
        }
    }
}
```

### **1.2 Android Permissions and Background Service**

**AndroidManifest.xml:**
```xml
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="com.solanasos.crisiscompanion">
    
    <!-- Required permissions -->
    <uses-permission android:name="android.permission.RECORD_AUDIO" />
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
    <uses-permission android:name="android.permission.CALL_PHONE" />
    <uses-permission android:name="android.permission.FOREGROUND_SERVICE" />
    <uses-permission android:name="android.permission.WAKE_LOCK" />
    
    <application
        android:allowBackup="true"
        android:icon="@mipmap/ic_launcher"
        android:label="@string/app_name"
        android:theme="@style/AppTheme">
        
        <!-- Main activity -->
        <activity android:name=".MainActivity"
            android:exported="true">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
        
        <!-- Background service for voice recognition -->
        <service android:name=".VoiceRecognitionService"
            android:enabled="true"
            android:exported="false"
            android:foregroundServiceType="microphone" />
            
    </application>
</manifest>
```

**Background Service Implementation:**
```kotlin
// VoiceRecognitionService.kt
class VoiceRecognitionService : Service() {
    private lateinit var audioRecord: AudioRecord
    private var isListening = false
    
    override fun onCreate() {
        super.onCreate()
        initializeAudioRecording()
    }
    
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForeground(NOTIFICATION_ID, createNotification())
        startVoiceRecognition()
        return START_STICKY
    }
    
    private fun initializeAudioRecording() {
        val sampleRate = 16000
        val channelConfig = AudioFormat.CHANNEL_IN_MONO
        val audioFormat = AudioFormat.ENCODING_PCM_16BIT
        val bufferSize = AudioRecord.getMinBufferSize(sampleRate, channelConfig, audioFormat)
        
        audioRecord = AudioRecord(
            MediaRecorder.AudioSource.MIC,
            sampleRate,
            channelConfig,
            audioFormat,
            bufferSize
        )
    }
    
    private fun startVoiceRecognition() {
        isListening = true
        Thread {
            val buffer = ShortArray(480) // RNNoise frame size
            
            audioRecord.startRecording()
            
            while (isListening) {
                val bytesRead = audioRecord.read(buffer, 0, buffer.size)
                
                if (bytesRead > 0) {
                    // Send to Rust for processing
                    processAudioData(buffer)
                }
            }
        }.start()
    }
    
    private fun processAudioData(audioData: ShortArray) {
        // Call Rust function via JNI
        CrisisCompanionBridge.processAudioData(audioData)
    }
}
```

### **1.3 Emergency UI Implementation**

**Emergency Response Activity:**
```kotlin
// EmergencyResponseActivity.kt
class EmergencyResponseActivity : AppCompatActivity() {
    private lateinit var binding: ActivityEmergencyResponseBinding
    private lateinit var textToSpeech: TextToSpeech
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityEmergencyResponseBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        initializeEmergencyResponse()
    }
    
    private fun initializeEmergencyResponse() {
        // Set volume to maximum
        val audioManager = getSystemService(Context.AUDIO_SERVICE) as AudioManager
        audioManager.setStreamVolume(
            AudioManager.STREAM_MUSIC,
            audioManager.getStreamMaxVolume(AudioManager.STREAM_MUSIC),
            0
        )
        
        // Initialize text-to-speech
        textToSpeech = TextToSpeech(this) { status ->
            if (status == TextToSpeech.SUCCESS) {
                startEmergencyInstructions()
            }
        }
    }
    
    private fun startEmergencyInstructions() {
        val emergencyType = intent.getStringExtra("emergency_type")
        val instructions = getEmergencyInstructions(emergencyType)
        
        // Display instructions
        binding.instructionText.text = instructions
        
        // Speak instructions
        textToSpeech.speak(instructions, TextToSpeech.QUEUE_FLUSH, null, null)
        
        // Auto-dial 911
        dialEmergencyServices()
    }
}
```

---

## ⛓️ **Phase 2: Solana Mobile Stack Integration**

### **2.1 Solana Mobile Stack SDK Integration**

**Add to Cargo.toml:**
```toml
[dependencies]
solana-mobile-stack = "1.0"
solana-mobile-wallet-adapter = "1.0"
solana-sdk = "2.3"
```

**Solana Mobile Integration:**
```rust
// New file: src/solana_mobile/mod.rs
use solana_mobile_stack::MobileStack;
use solana_mobile_wallet_adapter::WalletAdapter;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;

pub struct SolanaMobileIntegration {
    mobile_stack: MobileStack,
    wallet_adapter: WalletAdapter,
    user_wallet: Option<Pubkey>,
}

impl SolanaMobileIntegration {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mobile_stack = MobileStack::new()?;
        let wallet_adapter = WalletAdapter::new()?;
        
        Ok(Self {
            mobile_stack,
            wallet_adapter,
            user_wallet: None,
        })
    }
    
    pub async fn connect_wallet(&mut self) -> Result<Pubkey, Box<dyn std::error::Error>> {
        // Connect to user's Solana wallet
        let wallet = self.wallet_adapter.connect().await?;
        self.user_wallet = Some(wallet.pubkey());
        
        info!("Connected to wallet: {}", wallet.pubkey());
        Ok(wallet.pubkey())
    }
    
    pub async fn store_emergency_data(
        &self,
        emergency_type: EmergencyType,
        audio_hash: String,
        timestamp: DateTime<Utc>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Create transaction to store emergency data on Solana
        let transaction = self.create_emergency_transaction(
            emergency_type,
            audio_hash,
            timestamp,
        )?;
        
        // Sign and send transaction
        let signature = self.wallet_adapter.sign_and_send_transaction(transaction).await?;
        
        info!("Emergency data stored on Solana: {}", signature);
        Ok(signature)
    }
    
    pub async fn process_bonk_rewards(
        &self,
        user_pubkey: Pubkey,
        reward_amount: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Process BONK token rewards for successful emergency response
        let transaction = self.create_bonk_reward_transaction(user_pubkey, reward_amount)?;
        let signature = self.wallet_adapter.sign_and_send_transaction(transaction).await?;
        
        info!("BONK rewards sent: {}", signature);
        Ok(signature)
    }
}
```

### **2.2 Secure Storage Implementation**

**Secure Storage for User Profiles:**
```rust
// New file: src/solana_mobile/secure_storage.rs
use solana_mobile_stack::SecureStorage;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SecureUserProfile {
    pub user_id: String,
    pub wallet_address: String,
    pub voice_model_data: Vec<u8>,
    pub emergency_phrases: Vec<String>,
    pub adaptation_score: f32,
}

pub struct SecureStorageManager {
    secure_storage: SecureStorage,
}

impl SecureStorageManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let secure_storage = SecureStorage::new()?;
        Ok(Self { secure_storage })
    }
    
    pub async fn store_user_profile(
        &self,
        profile: SecureUserProfile,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let encrypted_data = self.secure_storage.encrypt(&profile)?;
        self.secure_storage.store("user_profile", &encrypted_data).await?;
        Ok(())
    }
    
    pub async fn load_user_profile(
        &self,
        user_id: &str,
    ) -> Result<Option<SecureUserProfile>, Box<dyn std::error::Error>> {
        let encrypted_data = self.secure_storage.load("user_profile").await?;
        
        if let Some(data) = encrypted_data {
            let profile: SecureUserProfile = self.secure_storage.decrypt(&data)?;
            Ok(Some(profile))
        } else {
            Ok(None)
        }
    }
}
```

### **2.3 Device-Specific Optimizations**

**Seeker Hardware Integration:**
```rust
// New file: src/solana_mobile/device_coordination.rs
use solana_mobile_stack::DeviceInfo;
use solana_mobile_stack::HardwareFeatures;

pub struct SeekerDeviceOptimizer {
    device_info: DeviceInfo,
    hardware_features: HardwareFeatures,
}

impl SeekerDeviceOptimizer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let device_info = DeviceInfo::new()?;
        let hardware_features = HardwareFeatures::new()?;
        
        Ok(Self {
            device_info,
            hardware_features,
        })
    }
    
    pub fn optimize_for_seeker(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Optimize for Seeker's specific hardware
        if self.device_info.is_seeker_device() {
            // Use Seeker's optimized audio processing
            self.hardware_features.enable_optimized_audio()?;
            
            // Enable Seeker's secure enclave for sensitive data
            self.hardware_features.enable_secure_enclave()?;
            
            // Optimize battery usage for emergency scenarios
            self.hardware_features.set_emergency_power_mode()?;
        }
        
        Ok(())
    }
    
    pub fn get_device_capabilities(&self) -> DeviceCapabilities {
        DeviceCapabilities {
            has_secure_enclave: self.hardware_features.has_secure_enclave(),
            has_optimized_audio: self.hardware_features.has_optimized_audio(),
            battery_optimization_level: self.hardware_features.get_battery_optimization_level(),
            emergency_mode_support: self.hardware_features.supports_emergency_mode(),
        }
    }
}
```

### **2.4 Mobile Wallet Adapter Integration**

**Wallet Connection and Transaction Handling:**
```rust
// New file: src/solana_mobile/wallet_adapter.rs
use solana_mobile_wallet_adapter::{
    WalletAdapter, WalletConnection, TransactionRequest, SignTransactionRequest,
};

pub struct WalletIntegration {
    wallet_adapter: WalletAdapter,
    connection: Option<WalletConnection>,
}

impl WalletIntegration {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let wallet_adapter = WalletAdapter::new()?;
        Ok(Self {
            wallet_adapter,
            connection: None,
        })
    }
    
    pub async fn connect_wallet(&mut self) -> Result<WalletConnection, Box<dyn std::error::Error>> {
        let connection = self.wallet_adapter.connect().await?;
        self.connection = Some(connection.clone());
        
        info!("Connected to Solana wallet");
        Ok(connection)
    }
    
    pub async fn sign_emergency_transaction(
        &self,
        transaction: Transaction,
    ) -> Result<SignedTransaction, Box<dyn std::error::Error>> {
        let connection = self.connection.as_ref()
            .ok_or("Wallet not connected")?;
        
        let sign_request = SignTransactionRequest {
            transaction: transaction.serialize()?,
            accounts: vec![connection.pubkey()],
        };
        
        let signed_transaction = connection.sign_transaction(sign_request).await?;
        Ok(signed_transaction)
    }
    
    pub async fn send_emergency_reward(
        &self,
        recipient: Pubkey,
        amount: u64,
        token_mint: Pubkey,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let connection = self.connection.as_ref()
            .ok_or("Wallet not connected")?;
        
        // Create token transfer transaction
        let transaction = self.create_token_transfer_transaction(
            recipient,
            amount,
            token_mint,
        )?;
        
        // Sign and send
        let signed_tx = self.sign_emergency_transaction(transaction).await?;
        let signature = connection.send_transaction(signed_tx).await?;
        
        info!("Emergency reward sent: {}", signature);
        Ok(signature)
    }
}
```

---

## 🔧 **Implementation Steps**

### **Step 1: Set Up Android Development Environment**
```bash
# Install Android Studio
# Install Solana Mobile SDK
# Set up Rust Android toolchain
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android
```

### **Step 2: Create Android Project Structure**
```
android/
├── app/
│   ├── src/main/
│   │   ├── java/com/solanasos/crisiscompanion/
│   │   │   ├── MainActivity.kt
│   │   │   ├── VoiceRecognitionService.kt
│   │   │   └── EmergencyResponseActivity.kt
│   │   ├── res/
│   │   └── AndroidManifest.xml
│   └── build.gradle
└── build.gradle
```

### **Step 3: Implement JNI Bridge**
```rust
// src/android/jni_bridge.rs
use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use jni::sys::jint;

#[no_mangle]
pub extern "system" fn Java_com_solanasos_crisiscompanion_CrisisCompanionBridge_processAudioData(
    env: JNIEnv,
    _class: JClass,
    audio_data: JObject,
) -> jint {
    // Convert Java audio data to Rust
    let audio_array = env.get_int_array_elements(audio_data).unwrap();
    
    // Process with Rust voice recognition
    let result = process_audio_data(&audio_array);
    
    env.release_int_array_elements(audio_data, audio_array, 0).unwrap();
    
    result as jint
}
```

### **Step 4: Integrate Solana Mobile Stack**
```rust
// src/solana_mobile/integration.rs
pub async fn initialize_solana_mobile() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Solana Mobile Stack
    let mobile_stack = SolanaMobileIntegration::new()?;
    
    // Connect to wallet
    let wallet = mobile_stack.connect_wallet().await?;
    
    // Initialize secure storage
    let secure_storage = SecureStorageManager::new()?;
    
    // Optimize for Seeker device
    let device_optimizer = SeekerDeviceOptimizer::new()?;
    device_optimizer.optimize_for_seeker()?;
    
    info!("Solana Mobile integration initialized");
    Ok(())
}
```

---

## 🧪 **Testing Implementation**

### **Test Real Microphone Integration:**
```bash
# Test on Android device
cargo build --target aarch64-linux-android
adb install target/aarch64-linux-android/debug/crisis-companion.apk
adb logcat | grep CrisisCompanion
```

### **Test Solana Mobile Integration:**
```bash
# Test wallet connection
cargo run --bin solana_mobile_test

# Test emergency transaction
cargo run --bin emergency_transaction_test
```

---

**This implementation guide provides the concrete steps to transform your hackathon prototype into a production-ready Solana Mobile dApp with real Android integration and Solana Mobile Stack features.** 