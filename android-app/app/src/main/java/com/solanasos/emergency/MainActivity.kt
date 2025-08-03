package com.solanasos.emergency

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Bundle
import android.speech.RecognitionListener
import android.speech.RecognizerIntent
import android.speech.SpeechRecognizer
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.lifecycle.lifecycleScope
import com.solanasos.emergency.databinding.ActivityMainBinding
import kotlinx.coroutines.launch
import android.util.Log
import java.util.*

class MainActivity : AppCompatActivity() {
    
    private lateinit var binding: ActivityMainBinding
    private lateinit var speechRecognizer: SpeechRecognizer
    private lateinit var solanaMobile: SolanaMobileIntegration
    private lateinit var rustBridge: RustBridge
    private var isListening = false
    private var isEmergencyMode = false
    
    companion object {
        private const val TAG = "MainActivity"
        private const val PERMISSION_REQUEST_CODE = 123
        private const val WAKE_WORD = "hey sos"
        private const val EMERGENCY_PHRASES = listOf(
            "drowning", "heart attack", "choking", "bleeding", "unconscious",
            "stroke", "seizure", "poisoning", "burn", "diabetic", "allergic", "trauma"
        )
    }
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        // Initialize Solana integration
        solanaMobile = SolanaMobileIntegration(this)
        
        // Initialize Rust bridge
        rustBridge = RustBridge(this)
        val backendInitialized = rustBridge.initializeBackend()
        if (!backendInitialized) {
            Log.e(TAG, "Failed to initialize Rust backend")
            Toast.makeText(this, "Backend initialization failed", Toast.LENGTH_LONG).show()
        }
        
        setupUI()
        setupVoiceRecognition()
        requestPermissions()
        
        Log.d(TAG, "Solana SOS Android app initialized")
    }
    
    private fun setupUI() {
        // Emergency activation button
        binding.btnEmergency.setOnClickListener {
            activateEmergencyMode()
        }
        
        // Voice recognition toggle
        binding.btnVoiceToggle.setOnClickListener {
            toggleVoiceRecognition()
        }
        
        // Solana wallet connection
        binding.btnConnectWallet.setOnClickListener {
            connectSolanaWallet()
        }
        
        // Settings
        binding.btnSettings.setOnClickListener {
            openSettings()
        }
        
        // Update initial status
        updateStatus("Ready for emergency activation")
    }
    
    private fun setupVoiceRecognition() {
        speechRecognizer = SpeechRecognizer.createSpeechRecognizer(this)
        speechRecognizer.setRecognitionListener(object : RecognitionListener {
            override fun onReadyForSpeech(params: Bundle?) {
                updateStatus("Listening for 'Hey SOS'...")
                isListening = true
                Log.d(TAG, "Voice recognition started")
            }
            
            override fun onResults(results: Bundle?) {
                val matches = results?.getStringArrayList(SpeechRecognizer.RESULTS_RECOGNITION)
                matches?.let { handleVoiceInput(it) }
                isListening = false
                Log.d(TAG, "Voice recognition results: $matches")
            }
            
            override fun onError(error: Int) {
                updateStatus("Voice recognition error")
                isListening = false
                Log.e(TAG, "Voice recognition error: $error")
            }
            
            // Other required methods
            override fun onBeginningOfSpeech() {}
            override fun onRmsChanged(rmsdB: Float) {}
            override fun onBufferReceived(buffer: ByteArray?) {}
            override fun onEndOfSpeech() {}
            override fun onPartialResults(partialResults: Bundle?) {}
            override fun onEvent(eventType: Int, params: Bundle?) {}
        })
    }
    
    private fun handleVoiceInput(matches: ArrayList<String>) {
        val input = matches[0].lowercase()
        Log.d(TAG, "Processing voice input: $input")
        
        if (input.contains(WAKE_WORD)) {
            // Wake word detected - start listening for emergency
            updateStatus("Wake word detected! Say emergency type...")
            startEmergencyListening()
        } else if (isListening && isEmergencyMode) {
            // Check for emergency phrases
            val detectedEmergency = detectEmergencyType(input)
            if (detectedEmergency != null) {
                activateEmergency(detectedEmergency)
            } else {
                updateStatus("Emergency not recognized. Try again.")
                startVoiceRecognition()
            }
        }
    }
    
    private fun detectEmergencyType(input: String): String? {
        return EMERGENCY_PHRASES.find { phrase ->
            input.contains(phrase)
        }
    }
    
    private fun activateEmergency(emergencyType: String) {
        updateStatus("EMERGENCY: $emergencyType - Calling 911...")
        Log.d(TAG, "Emergency activated: $emergencyType")
        
        // Start emergency service
        val intent = Intent(this, EmergencyService::class.java).apply {
            putExtra("emergency_type", emergencyType)
        }
        startService(intent)
        
        // Show emergency UI with context-aware guidance
        showEmergencyUI(emergencyType)
        
        // Award Solana rewards for emergency activation using Rust backend
        lifecycleScope.launch {
            val rewarded = rustBridge.awardEmergencyTokens(emergencyType, "emergency_activation")
            if (rewarded) {
                val totalRewards = rustBridge.getTotalRewards()
                Toast.makeText(this@MainActivity, "Awarded SKR/BONK tokens! Total: $totalRewards", Toast.LENGTH_LONG).show()
            }
        }
        
        // Record emergency on blockchain using Rust backend
        lifecycleScope.launch {
            val recordId = rustBridge.recordEmergencyData(
                emergencyType = emergencyType,
                location = "Current location", // Would get from GPS
                actions = listOf("emergency_activation", "voice_recognition"),
                outcome = "In Progress"
            )
            if (recordId != null) {
                Log.d(TAG, "Emergency recorded on blockchain: $recordId")
            }
        }
    }
    
    private fun showEmergencyUI(emergencyType: String) {
        binding.emergencyLayout.visibility = android.view.View.VISIBLE
        binding.tvEmergencyType.text = emergencyType
        
        // Get emergency instructions from Rust library with context awareness
        val instructions = getEmergencyInstructionsWithContext(emergencyType)
        binding.tvInstructions.text = instructions
        
        Log.d(TAG, "Emergency UI shown for: $emergencyType")
    }
    
    private fun getEmergencyInstructionsWithContext(emergencyType: String): String {
        // Use Rust backend for context-aware guidance
        return try {
            val userPhrase = "emergency activation" // Would be the actual user phrase
            val location = "current location" // Would be actual GPS location
            rustBridge.getEmergencyInstructionsWithContext(emergencyType, userPhrase, location)
        } catch (e: Exception) {
            Log.e(TAG, "Error getting context-aware instructions, using fallback", e)
            getEmergencyInstructions(emergencyType) // Fallback to basic instructions
        }
    }
    
    private fun getEmergencyInstructions(emergencyType: String): String {
        // This would call your Rust library via JNI
        return when (emergencyType) {
            "drowning" -> "1. Remove victim from water\n2. Check breathing\n3. Begin CPR if needed\n4. Call 911"
            "heart attack" -> "1. Call 911 immediately\n2. Have victim sit down\n3. Loosen tight clothing\n4. Monitor breathing"
            "choking" -> "1. Perform Heimlich maneuver\n2. 5 back blows, 5 abdominal thrusts\n3. Call 911 if not resolved"
            "bleeding" -> "1. Apply direct pressure\n2. Elevate if possible\n3. Use tourniquet if severe\n4. Call 911"
            "unconscious" -> "1. Check breathing\n2. Begin CPR if needed\n3. Call 911 immediately\n4. Monitor for changes"
            "stroke" -> "1. Remember FAST\n2. Face, Arm, Speech, Time\n3. Call 911 immediately\n4. Note time of onset"
            "seizure" -> "1. Clear area of objects\n2. Don't restrain\n3. Time the seizure\n4. Call 911 if >5 minutes"
            "poisoning" -> "1. Call Poison Control\n2. Don't induce vomiting\n3. Save container\n4. Call 911 if severe"
            "burn" -> "1. Cool with water\n2. Don't use ice\n3. Cover with clean cloth\n4. Call 911 if severe"
            "diabetic" -> "1. Check blood sugar\n2. Give sugar if low\n3. Call 911 if unconscious\n4. Monitor breathing"
            "allergic" -> "1. Use EpiPen if available\n2. Call 911 immediately\n3. Monitor breathing\n4. Lie flat if dizzy"
            "trauma" -> "1. Stop bleeding\n2. Immobilize injuries\n3. Call 911\n4. Monitor consciousness"
            else -> "Call 911 immediately and follow emergency dispatcher instructions"
        }
    }
    
    private fun toggleVoiceRecognition() {
        if (isListening) {
            stopVoiceRecognition()
        } else {
            startVoiceRecognition()
        }
    }
    
    private fun startVoiceRecognition() {
        val intent = Intent(RecognizerIntent.ACTION_RECOGNIZE_SPEECH).apply {
            putExtra(RecognizerIntent.EXTRA_LANGUAGE_MODEL, RecognizerIntent.LANGUAGE_MODEL_FREE_FORM)
            putExtra(RecognizerIntent.EXTRA_LANGUAGE, Locale.getDefault())
            putExtra(RecognizerIntent.EXTRA_PROMPT, "Say 'Hey SOS' to activate emergency mode")
        }
        
        try {
            speechRecognizer.startListening(intent)
            binding.btnVoiceToggle.text = "Stop Listening"
            Log.d(TAG, "Voice recognition started")
        } catch (e: Exception) {
            Toast.makeText(this, "Voice recognition not available", Toast.LENGTH_SHORT).show()
            Log.e(TAG, "Voice recognition error", e)
        }
    }
    
    private fun stopVoiceRecognition() {
        speechRecognizer.stopListening()
        binding.btnVoiceToggle.text = "Start Voice Recognition"
        updateStatus("Voice recognition stopped")
        isListening = false
        Log.d(TAG, "Voice recognition stopped")
    }
    
    private fun startEmergencyListening() {
        val intent = Intent(RecognizerIntent.ACTION_RECOGNIZE_SPEECH).apply {
            putExtra(RecognizerIntent.EXTRA_LANGUAGE_MODEL, RecognizerIntent.LANGUAGE_MODEL_FREE_FORM)
            putExtra(RecognizerIntent.EXTRA_LANGUAGE, Locale.getDefault())
            putExtra(RecognizerIntent.EXTRA_PROMPT, "What emergency? (drowning, heart attack, choking, etc.)")
        }
        
        try {
            speechRecognizer.startListening(intent)
            isEmergencyMode = true
            Log.d(TAG, "Emergency listening started")
        } catch (e: Exception) {
            Toast.makeText(this, "Voice recognition error", Toast.LENGTH_SHORT).show()
            Log.e(TAG, "Emergency listening error", e)
        }
    }
    
    private fun activateEmergencyMode() {
        updateStatus("Emergency mode activated!")
        isEmergencyMode = true
        // Show emergency options
        binding.emergencyOptionsLayout.visibility = android.view.View.VISIBLE
        Log.d(TAG, "Emergency mode activated")
    }
    
    private fun connectSolanaWallet() {
        updateStatus("Connecting to Solana wallet...")
        
        lifecycleScope.launch {
            val connected = solanaMobile.connectWallet()
            
            if (connected) {
                val walletAddress = solanaMobile.getWalletAddress()
                updateStatus("Connected to Solana wallet: ${walletAddress?.take(8)}...")
                Toast.makeText(this@MainActivity, "Solana wallet connected!", Toast.LENGTH_SHORT).show()
                Log.d(TAG, "Solana wallet connected: $walletAddress")
            } else {
                updateStatus("Failed to connect to Solana wallet")
                Toast.makeText(this@MainActivity, "Wallet connection failed", Toast.LENGTH_SHORT).show()
                Log.w(TAG, "Solana wallet connection failed")
            }
        }
    }
    
    private fun openSettings() {
        updateStatus("Opening settings...")
        // Open settings activity
        val intent = Intent(this, SettingsActivity::class.java)
        startActivity(intent)
        Log.d(TAG, "Settings requested")
    }
    
    private fun updateStatus(message: String) {
        binding.tvStatus.text = message
        Log.d(TAG, "Status updated: $message")
    }
    
    private fun requestPermissions() {
        val permissions = arrayOf(
            Manifest.permission.RECORD_AUDIO,
            Manifest.permission.ACCESS_FINE_LOCATION,
            Manifest.permission.CALL_PHONE,
            Manifest.permission.SEND_SMS
        )
        
        val permissionsToRequest = permissions.filter {
            ContextCompat.checkSelfPermission(this, it) != PackageManager.PERMISSION_GRANTED
        }.toTypedArray()
        
        if (permissionsToRequest.isNotEmpty()) {
            ActivityCompat.requestPermissions(this, permissionsToRequest, PERMISSION_REQUEST_CODE)
            Log.d(TAG, "Requesting permissions: ${permissionsToRequest.joinToString()}")
        }
    }
    
    override fun onDestroy() {
        super.onDestroy()
        speechRecognizer.destroy()
        Log.d(TAG, "MainActivity destroyed")
    }
} 