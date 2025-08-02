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
import com.solanasos.emergency.databinding.ActivityMainBinding
import java.util.*

class MainActivity : AppCompatActivity() {
    
    private lateinit var binding: ActivityMainBinding
    private lateinit var speechRecognizer: SpeechRecognizer
    private var isListening = false
    
    companion object {
        private const val PERMISSION_REQUEST_CODE = 123
        private const val WAKE_WORD = "hey sos"
    }
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        setupUI()
        setupVoiceRecognition()
        requestPermissions()
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
    }
    
    private fun setupVoiceRecognition() {
        speechRecognizer = SpeechRecognizer.createSpeechRecognizer(this)
        speechRecognizer.setRecognitionListener(object : RecognitionListener {
            override fun onReadyForSpeech(params: Bundle?) {
                binding.tvStatus.text = "Listening for 'Hey SOS'..."
                isListening = true
            }
            
            override fun onResults(results: Bundle?) {
                val matches = results?.getStringArrayList(SpeechRecognizer.RESULTS_RECOGNITION)
                matches?.let { handleVoiceInput(it) }
                isListening = false
            }
            
            override fun onError(error: Int) {
                binding.tvStatus.text = "Voice recognition error"
                isListening = false
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
        
        if (input.contains(WAKE_WORD)) {
            // Wake word detected - start listening for emergency
            binding.tvStatus.text = "Wake word detected! Say emergency type..."
            startEmergencyListening()
        } else if (isListening) {
            // Check for emergency phrases
            when {
                input.contains("drowning") -> activateEmergency("Drowning")
                input.contains("heart attack") -> activateEmergency("Heart Attack")
                input.contains("choking") -> activateEmergency("Choking")
                input.contains("bleeding") -> activateEmergency("Bleeding")
                input.contains("unconscious") -> activateEmergency("Unconscious")
                input.contains("stroke") -> activateEmergency("Stroke")
                input.contains("seizure") -> activateEmergency("Seizure")
                input.contains("poisoning") -> activateEmergency("Poisoning")
                input.contains("burn") -> activateEmergency("Severe Burns")
                input.contains("diabetic") -> activateEmergency("Diabetic Emergency")
                input.contains("allergic") -> activateEmergency("Allergic Reaction")
                input.contains("trauma") -> activateEmergency("Trauma")
                else -> {
                    binding.tvStatus.text = "Emergency not recognized. Try again."
                    startVoiceRecognition()
                }
            }
        }
    }
    
    private fun activateEmergency(emergencyType: String) {
        binding.tvStatus.text = "EMERGENCY: $emergencyType - Calling 911..."
        
        // Start emergency service
        val intent = Intent(this, EmergencyService::class.java).apply {
            putExtra("emergency_type", emergencyType)
        }
        startService(intent)
        
        // Show emergency UI
        showEmergencyUI(emergencyType)
    }
    
    private fun showEmergencyUI(emergencyType: String) {
        binding.emergencyLayout.visibility = android.view.View.VISIBLE
        binding.tvEmergencyType.text = emergencyType
        
        // Get emergency instructions from Rust library
        val instructions = getEmergencyInstructions(emergencyType)
        binding.tvInstructions.text = instructions
    }
    
    private fun getEmergencyInstructions(emergencyType: String): String {
        // This would call your Rust library via JNI
        return when (emergencyType) {
            "Drowning" -> "1. Remove victim from water\n2. Check breathing\n3. Begin CPR if needed\n4. Call 911"
            "Heart Attack" -> "1. Call 911 immediately\n2. Have victim sit down\n3. Loosen tight clothing\n4. Monitor breathing"
            "Choking" -> "1. Perform Heimlich maneuver\n2. 5 back blows, 5 abdominal thrusts\n3. Call 911 if not resolved"
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
        } catch (e: Exception) {
            Toast.makeText(this, "Voice recognition not available", Toast.LENGTH_SHORT).show()
        }
    }
    
    private fun stopVoiceRecognition() {
        speechRecognizer.stopListening()
        binding.btnVoiceToggle.text = "Start Voice Recognition"
        binding.tvStatus.text = "Voice recognition stopped"
        isListening = false
    }
    
    private fun startEmergencyListening() {
        val intent = Intent(RecognizerIntent.ACTION_RECOGNIZE_SPEECH).apply {
            putExtra(RecognizerIntent.EXTRA_LANGUAGE_MODEL, RecognizerIntent.LANGUAGE_MODEL_FREE_FORM)
            putExtra(RecognizerIntent.EXTRA_LANGUAGE, Locale.getDefault())
            putExtra(RecognizerIntent.EXTRA_PROMPT, "What emergency? (drowning, heart attack, choking, etc.)")
        }
        
        try {
            speechRecognizer.startListening(intent)
        } catch (e: Exception) {
            Toast.makeText(this, "Voice recognition error", Toast.LENGTH_SHORT).show()
        }
    }
    
    private fun activateEmergencyMode() {
        binding.tvStatus.text = "Emergency mode activated!"
        // Show emergency options
        binding.emergencyOptionsLayout.visibility = android.view.View.VISIBLE
    }
    
    private fun connectSolanaWallet() {
        binding.tvStatus.text = "Connecting to Solana wallet..."
        // Implement Solana Mobile Stack integration
        // This would connect to the user's Solana wallet
    }
    
    private fun openSettings() {
        binding.tvStatus.text = "Opening settings..."
        // Open settings activity
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
        }
    }
    
    override fun onDestroy() {
        super.onDestroy()
        speechRecognizer.destroy()
    }
} 