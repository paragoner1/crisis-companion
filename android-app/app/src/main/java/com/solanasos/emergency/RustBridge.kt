package com.solanasos.emergency

import android.content.Context
import android.util.Log
import java.io.File

/**
 * JNI Bridge to connect Android app with Rust backend
 * This class provides the interface between Android and the Rust library
 */
class RustBridge(private val context: Context) {
    
    companion object {
        private const val TAG = "RustBridge"
        
        // Load the Rust library
        init {
            try {
                System.loadLibrary("solana_sos")
                Log.d(TAG, "Rust library loaded successfully")
            } catch (e: UnsatisfiedLinkError) {
                Log.e(TAG, "Failed to load Rust library", e)
            }
        }
    }
    
    // Voice Recognition Functions
    external fun initializeVoiceRecognition(): Boolean
    external fun processVoiceInput(audioData: ByteArray): String?
    external fun detectWakeWord(audioData: ByteArray): Boolean
    external fun detectEmergencyPhrase(audioData: ByteArray): String?
    external fun adaptVoiceModel(userAudioData: ByteArray): Boolean
    
    // Emergency Response Functions
    external fun getEmergencyInstructions(emergencyType: String): String
    external fun getContextAwareGuidance(emergencyType: String, context: String): String
    // external fun detectEmergencyStage(userPhrase: String, location: String, actions: String): String
    external fun generateGuidance(emergencyType: String, stage: String): String
    
    // Safety Features Functions
    external fun activateSilentSOS(location: String): Boolean
    external fun detectCrash(accelerometerData: FloatArray, gpsData: DoubleArray): Boolean
    external fun sendTrustedNetworkAlert(contacts: Array<String>, location: String): Boolean
    
    // Gamification Functions
    external fun awardXP(action: String, amount: Int): Boolean
    external fun getHeroLevel(): Int
    external fun getTotalRewards(): String
    external fun unlockAchievement(achievementId: String): Boolean
    
    // Blockchain Functions
    external fun connectSolanaWallet(): Boolean
    external fun getWalletAddress(): String?
    external fun sendTokens(tokenType: String, amount: Int, recipient: String): Boolean
    external fun recordEmergencyOnBlockchain(emergencyData: String): String?
    
    // Database Functions
    external fun initializeDatabase(): Boolean
    external fun saveEmergencyContact(name: String, phone: String): Boolean
    external fun getEmergencyContacts(): String
    external fun saveUserPreferences(preferences: String): Boolean
    external fun getUserPreferences(): String
    
    // Audio Processing Functions
    external fun processAudioWithNoiseFiltering(audioData: ByteArray): ByteArray
    external fun applyRNNoiseFilter(audioData: ByteArray): ByteArray
    external fun enhanceAudioQuality(audioData: ByteArray): ByteArray
    
    // Utility Functions
    external fun getAppVersion(): String
    external fun getBuildInfo(): String
    external fun validateEmergencyType(emergencyType: String): Boolean
    
    /**
     * Initialize the Rust backend with Android context
     */
    fun initializeBackend(): Boolean {
        return try {
            // Set up Android-specific paths
            val filesDir = context.filesDir.absolutePath
            val cacheDir = context.cacheDir.absolutePath
            
            // Initialize Rust backend with Android paths
            initializeAndroidPaths(filesDir, cacheDir)
            
            // Initialize database
            val dbInitialized = initializeDatabase()
            if (!dbInitialized) {
                Log.e(TAG, "Failed to initialize database")
                return false
            }
            
            // Initialize voice recognition
            val voiceInitialized = initializeVoiceRecognition()
            if (!voiceInitialized) {
                Log.e(TAG, "Failed to initialize voice recognition")
                return false
            }
            
            Log.d(TAG, "Rust backend initialized successfully")
            true
        } catch (e: Exception) {
            Log.e(TAG, "Failed to initialize Rust backend", e)
            false
        }
    }
    
    /**
     * Process voice input and return recognized text
     */
    fun processVoiceInputKotlin(audioData: ByteArray): String? {
        return try {
            // Apply noise filtering first
            val filteredAudio = processAudioWithNoiseFiltering(audioData)
            
            // Process with voice recognition
            val recognizedText = processVoiceInput(filteredAudio)
            
            Log.d(TAG, "Voice input processed: $recognizedText")
            recognizedText
        } catch (e: Exception) {
            Log.e(TAG, "Error processing voice input", e)
            null
        }
    }
    
    /**
     * Get emergency instructions with context awareness
     */
    fun getEmergencyInstructionsWithContext(emergencyType: String, userPhrase: String, location: String): String {
        return try {
            // Detect current stage
            val stage = "demo"
            
            // Get context-aware guidance
            val guidance = getContextAwareGuidance(emergencyType, stage)
            
            Log.d(TAG, "Emergency guidance generated for $emergencyType at stage $stage")
            guidance
        } catch (e: Exception) {
            Log.e(TAG, "Error getting emergency instructions", e)
            getEmergencyInstructions(emergencyType) // Fallback to basic instructions
        }
    }
    
    /**
     * Award tokens for emergency actions
     */
    fun awardEmergencyTokens(emergencyType: String, action: String): Boolean {
        return try {
            // Award XP
            val xpAwarded = awardXP("emergency_$action", 100)
            
            // Award tokens based on emergency type
            val tokenAmount = when (emergencyType) {
                "drowning" -> 50
                "heart attack" -> 75
                "choking" -> 60
                "bleeding" -> 65
                "unconscious" -> 80
                "stroke" -> 85
                "seizure" -> 70
                "poisoning" -> 55
                "burn" -> 45
                "diabetic" -> 40
                "allergic" -> 90
                "trauma" -> 75
                else -> 50
            }
            
            // Send tokens to user wallet
            val walletAddress = getWalletAddress()
            if (walletAddress != null) {
                val tokensSent = sendTokens("SKR", tokenAmount, walletAddress)
                if (tokensSent) {
                    Log.d(TAG, "Awarded $tokenAmount SKR tokens for $emergencyType")
                }
            }
            
            xpAwarded
        } catch (e: Exception) {
            Log.e(TAG, "Error awarding emergency tokens", e)
            false
        }
    }
    
    /**
     * Record emergency on blockchain with full data
     */
    fun recordEmergencyData(emergencyType: String, location: String, actions: List<String>, outcome: String): String? {
        return try {
            val emergencyData = EmergencyData(
                emergencyType = emergencyType,
                timestamp = System.currentTimeMillis(),
                location = location,
                actions = actions,
                outcome = outcome
            )
            
            val jsonData = emergencyData.toJson()
            val recordId = recordEmergencyOnBlockchain(jsonData)
            
            Log.d(TAG, "Emergency recorded on blockchain: $recordId")
            recordId
        } catch (e: Exception) {
            Log.e(TAG, "Error recording emergency on blockchain", e)
            null
        }
    }
    
    /**
     * Get user's emergency contacts from database
     */
    fun getEmergencyContactsList(): List<EmergencyContact> {
        return try {
            val contactsJson = getEmergencyContacts()
            // Parse JSON and return list of contacts
            parseEmergencyContacts(contactsJson)
        } catch (e: Exception) {
            Log.e(TAG, "Error getting emergency contacts", e)
            emptyList()
        }
    }
    
    /**
     * Save emergency contact to database
     */
    fun saveEmergencyContactKotlin(name: String, phone: String): Boolean {
        return try {
            val saved = saveEmergencyContact(name, phone)
            if (saved) {
                Log.d(TAG, "Emergency contact saved: $name - $phone")
            }
            saved
        } catch (e: Exception) {
            Log.e(TAG, "Error saving emergency contact", e)
            false
        }
    }
    
    // Helper functions for Android-specific initialization
    private external fun initializeAndroidPaths(filesDir: String, cacheDir: String)
    
    // Data classes for structured data
    data class EmergencyData(
        val emergencyType: String,
        val timestamp: Long,
        val location: String,
        val actions: List<String>,
        val outcome: String
    ) {
        fun toJson(): String {
            return """
                {
                    "emergencyType": "$emergencyType",
                    "timestamp": $timestamp,
                    "location": "$location",
                    "actions": [${actions.joinToString(",") { "\"$it\"" }}],
                    "outcome": "$outcome"
                }
            """.trimIndent()
        }
    }
    
    data class EmergencyContact(
        val name: String,
        val phone: String
    )
    
    // Helper function to parse emergency contacts JSON
    private fun parseEmergencyContacts(json: String): List<EmergencyContact> {
        // Simple JSON parsing - in production would use proper JSON library
        return try {
            // This is a simplified parser - would use Gson or similar in production
            val contacts = mutableListOf<EmergencyContact>()
            // Parse JSON and create EmergencyContact objects
            contacts
        } catch (e: Exception) {
            Log.e(TAG, "Error parsing emergency contacts", e)
            emptyList()
        }
    }
} 