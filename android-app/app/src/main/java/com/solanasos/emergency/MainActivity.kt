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
import kotlinx.coroutines.delay
import android.util.Log
import android.widget.TextView
import java.util.*
import androidx.appcompat.app.AlertDialog
import android.net.ConnectivityManager
import android.net.NetworkCapabilities
import android.content.Context
import android.widget.Button
import com.solanasos.emergency.R

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
        private const val TRAINING_REQUEST_CODE = 456
        private const val WAKE_WORD = "hey sos"
        private val EMERGENCY_PHRASES = listOf(
            "drowning", "heart attack", "choking", "bleeding", "unconscious",
            "stroke", "seizure", "poisoning", "burn", "diabetic", "allergic", "trauma"
        )
        private const val CHALLENGES_REQUEST_CODE = 789
    }
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Set status bar color to match the deep navy background
        window.statusBarColor = ContextCompat.getColor(this, R.color.deep_navy)
        
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        // Initialize Solana integration
        solanaMobile = SolanaMobileIntegration(this)
        
        // Initialize Rust bridge (optional for demo)
        try {
            rustBridge = RustBridge(this)
            val backendInitialized = rustBridge.initializeBackend()
            if (!backendInitialized) {
                Log.w(TAG, "Rust backend not available - running in demo mode")
                Toast.makeText(this, "Running in demo mode", Toast.LENGTH_SHORT).show()
            }
        } catch (e: UnsatisfiedLinkError) {
            Log.w(TAG, "Rust library not available - running in demo mode", e)
            Toast.makeText(this, "Running in demo mode", Toast.LENGTH_SHORT).show()
        } catch (e: Exception) {
            Log.e(TAG, "Error initializing Rust backend", e)
            Toast.makeText(this, "Running in demo mode", Toast.LENGTH_SHORT).show()
        }
        
        setupUI()
        setupVoiceRecognition()
        requestPermissions()
        updateSafetyFeaturesStatus()
        scheduleEngagingNotifications()
        
        // Show mission disclaimer on first launch
        showMissionDisclaimer()
        
        Log.d(TAG, "Solana SOS Android app initialized")
    }
    
    private fun showMissionDisclaimer() {
        val prefs = getSharedPreferences("app_settings", MODE_PRIVATE)
        val disclaimerShown = prefs.getBoolean("mission_disclaimer_shown", false)
        
        if (!disclaimerShown) {
            val dialog = AlertDialog.Builder(this)
                .setTitle("🚨 MISSION CRITICAL")
                .setMessage("""
                    **SERIOUS PURPOSE, LIFE-SAVING MISSION**
                    
                    Solana SOS is not a game. This is a serious emergency response application designed to prevent unnecessary deaths when lives could otherwise be saved.
                    
                    **OUR MISSION:**
                    • Train users to recognize and respond to life-threatening emergencies
                    • Provide immediate, authoritative guidance during critical moments
                    • Bridge the gap between emergency occurrence and professional help
                    • Empower ordinary people to save lives
                    
                    **WHY GAMIFICATION:**
                    The training and engagement features exist for one purpose: to ensure users develop and maintain the skills necessary to take responsibility for saving someone's life when they're nearby.
                    
                    • **Daily check-ins** → Maintain emergency readiness
                    • **Training modules** → Build life-saving skills
                    • **Achievement system** → Motivate continuous learning
                    • **Cross-app challenges** → Expand emergency response network
                    • **Token rewards** → Incentivize skill development
                    
                    **THE REALITY:**
                    Every day, people die from emergencies that could have been prevented with proper training and immediate response. Our gamification serves the serious purpose of ensuring you're ready when seconds count.
                    
                    **YOUR RESPONSIBILITY:**
                    By using this app, you commit to:
                    • Learning and maintaining emergency response skills
                    • Taking action when you witness life-threatening situations
                    • Contributing to a network of trained responders
                    • Using your skills to save lives
                    
                    This is not entertainment. This is preparation for moments when lives hang in the balance.
                    
                    Do you accept this mission?
                """.trimIndent())
                .setPositiveButton("I Accept This Mission") { dialog, _ ->
                    dialog.dismiss()
                    prefs.edit().putBoolean("mission_disclaimer_shown", true).apply()
                    Toast.makeText(this, "🚨 Mission accepted. Stay ready.", Toast.LENGTH_LONG).show()
                }
                .setNegativeButton("I Need More Time") { dialog, _ ->
                    dialog.dismiss()
                    // Don't mark as shown, so it appears again
                }
                .setCancelable(false)
                .create()
            
            dialog.show()
        }
    }
    
    private fun scheduleEngagingNotifications() {
        val prefs = getSharedPreferences("engagement", MODE_PRIVATE)
        val lastNotification = prefs.getLong("last_engagement_notification", 0)
        val currentTime = System.currentTimeMillis()
        
        // Show engagement notification if it's been more than 12 hours
        if (currentTime - lastNotification > 12 * 60 * 60 * 1000) {
            showEngagementNotification()
            prefs.edit().putLong("last_engagement_notification", currentTime).apply()
        }
    }
    
    private fun showEngagementNotification() {
        val notifications = listOf(
            "🫀 Your emergency response skills need practice. Complete CPR training to save lives!",
            "🎯 Test your emergency readiness. Practice the demo to stay sharp when seconds count!",
            "💎 Build your emergency response network. Daily check-ins keep you mission-ready!",
            "🚨 Test your voice recognition - say 'Hey SOS' to ensure you're ready for real emergencies!",
            "🏆 Level up your life-saving capabilities with more training modules!"
        )
        
        val randomNotification = notifications.random()
        
        val dialog = AlertDialog.Builder(this)
            .setTitle("🚨 MISSION READINESS CHECK")
            .setMessage(randomNotification)
            .setPositiveButton("Stay Mission Ready") { dialog, _ ->
                dialog.dismiss()
                // Give small XP bonus for engaging
                val trainingPrefs = getSharedPreferences("training_progress", MODE_PRIVATE)
                val currentXP = trainingPrefs.getInt("bonus_xp", 0)
                trainingPrefs.edit().putInt("bonus_xp", currentXP + 5).apply()
                updateSOSHeroStatus()
                Toast.makeText(this, "✅ +5 XP for maintaining readiness!", Toast.LENGTH_SHORT).show()
            }
            .setNegativeButton("Later") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }
    
    private fun setupUI() {
        // Demo button
        binding.btnDemo.setOnClickListener {
            showDemoModeDialog()
        }
        
        // Settings button
        binding.btnSettings.setOnClickListener {
            val intent = Intent(this, SettingsActivity::class.java)
            startActivity(intent)
        }
        
        // Help button
        binding.btnHelp.setOnClickListener {
            val intent = Intent(this, InfoActivity::class.java)
            startActivity(intent)
        }
        
        // Training Center button
        binding.btnTraining.setOnClickListener {
            openTrainingCenter()
        }
        
        // Cross-App Challenges button
        binding.btnChallenges.setOnClickListener {
            openCrossAppChallenges()
        }
        
        // Emergency activation button
        binding.btnEmergency.setOnClickListener {
            activateEmergencyMode()
        }
        
        // Update status
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
        
        // Start emergency service (commented out for demo - requires foreground service permissions)
        // val intent = Intent(this, EmergencyService::class.java).apply {
        //     putExtra("emergency_type", emergencyType)
        // }
        // startService(intent)
        
        // Demo: Simulate emergency service activation
        Log.d(TAG, "Demo: Emergency service would be activated for $emergencyType")
        
        // Show emergency UI with context-aware guidance
        showEmergencyUI(emergencyType)
        
        // Award Solana rewards for emergency activation using Rust backend
        lifecycleScope.launch {
            try {
                val rewarded = rustBridge.awardEmergencyTokens(emergencyType, "emergency_activation")
                if (rewarded) {
                    val totalRewards = rustBridge.getTotalRewards()
                    Toast.makeText(this@MainActivity, "Awarded SKR/BONK tokens! Total: $totalRewards", Toast.LENGTH_LONG).show()
                }
            } catch (e: Exception) {
                Log.w(TAG, "Demo mode: Simulating token rewards", e)
                Toast.makeText(this@MainActivity, "Demo: Awarded 100 BONK + 25 SKR tokens!", Toast.LENGTH_LONG).show()
            }
        }
        
        // Record emergency on blockchain using Rust backend
        lifecycleScope.launch {
            try {
                val recordId = rustBridge.recordEmergencyData(
                    emergencyType = emergencyType,
                    location = "Current location", // Would get from GPS
                    actions = listOf("emergency_activation", "voice_recognition"),
                    outcome = "In Progress"
                )
                if (recordId != null) {
                    Log.d(TAG, "Emergency recorded on blockchain: $recordId")
                }
            } catch (e: Exception) {
                Log.w(TAG, "Demo mode: Simulating blockchain recording", e)
                Log.d(TAG, "Demo: Emergency recorded on blockchain: demo_record_${System.currentTimeMillis()}")
            }
        }
    }
    
    private fun showEmergencyUI(emergencyType: String) {
        try {
            binding.emergencyLayout.visibility = android.view.View.VISIBLE
            binding.tvEmergencyType.text = emergencyType
            
            // Get emergency instructions from Rust library with context awareness
            val instructions = getEmergencyInstructionsWithContext(emergencyType)
            binding.tvInstructions.text = instructions
            
            Log.d(TAG, "Emergency UI shown for: $emergencyType")
        } catch (e: Exception) {
            Log.e(TAG, "Error showing emergency UI", e)
            // Fallback to basic emergency display
            updateStatus("Emergency: $emergencyType - App calls 911 immediately")
        }
    }
    
    private fun getEmergencyInstructionsWithContext(emergencyType: String): String {
        // Use Rust backend for context-aware guidance
        return try {
            val userPhrase = "emergency activation" // Would be the actual user phrase
            val location = "current location" // Would be actual GPS location
            rustBridge.getEmergencyInstructionsWithContext(emergencyType, userPhrase, location)
        } catch (e: Exception) {
            Log.w(TAG, "Demo mode: Using fallback instructions", e)
            getEmergencyInstructions(emergencyType) // Fallback to basic instructions
        }
    }
    
    private fun getEmergencyInstructions(emergencyType: String): String {
        // This would call your Rust library via JNI
        return when (emergencyType) {
            "drowning" -> "1. Remove victim from water\n2. Check breathing\n3. Begin CPR if needed\n4. App calls 911"
            "heart attack" -> "1. App calls 911 immediately\n2. Have victim sit down\n3. Loosen tight clothing\n4. Monitor breathing"
            "choking" -> "1. Perform Heimlich maneuver\n2. 5 back blows, 5 abdominal thrusts\n3. App calls 911 if not resolved"
            "bleeding" -> "1. Apply direct pressure\n2. Elevate if possible\n3. Use tourniquet if severe\n4. App calls 911"
            "unconscious" -> "1. Check breathing\n2. Begin CPR if needed\n3. App calls 911 immediately\n4. Monitor for changes"
            "stroke" -> "1. Remember FAST\n2. Face, Arm, Speech, Time\n3. App calls 911 immediately\n4. Note time of onset"
            "seizure" -> "1. Clear area of objects\n2. Don't restrain\n3. Time the seizure\n4. App calls 911 if >5 minutes"
            "poisoning" -> "1. Call Poison Control\n2. Don't induce vomiting\n3. Save container\n4. App calls 911 if severe"
            "burn" -> "1. Cool with water\n2. Don't use ice\n3. Cover with clean cloth\n4. App calls 911 if severe"
            "diabetic" -> "1. Check blood sugar\n2. Give sugar if low\n3. App calls 911 if unconscious\n4. Monitor breathing"
            "allergic" -> "1. Use EpiPen if available\n2. App calls 911 immediately\n3. Monitor breathing\n4. Lie flat if dizzy"
            "trauma" -> "1. Stop bleeding\n2. Immobilize injuries\n3. App calls 911\n4. Monitor consciousness"
            else -> "App calls 911 immediately and follow emergency dispatcher instructions"
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
        try {
            // Demo mode - simulate offline Vosk recognition
            updateStatus("Voice recognition active - Say 'Hey SOS'")
            isListening = true
            
            // Simulate wake word detection after 2 seconds
            lifecycleScope.launch {
                try {
                    delay(2000)
                    if (isListening) {
                        updateStatus("Wake word detected! Say emergency type...")
                        startEmergencyListening()
                    }
                } catch (e: Exception) {
                    Log.e(TAG, "Error in voice recognition simulation", e)
                    updateStatus("Voice recognition error - try again")
                    stopVoiceRecognition()
                }
            }
            
            Log.d(TAG, "Voice recognition started (demo mode)")
        } catch (e: Exception) {
            Log.e(TAG, "Error starting voice recognition", e)
            updateStatus("Voice recognition failed - try again")
        }
    }
    
    private fun stopVoiceRecognition() {
        updateStatus("Voice recognition stopped")
        isListening = false
        isEmergencyMode = false
        Log.d(TAG, "Voice recognition stopped")
    }
    
    private fun startEmergencyListening() {
        // Demo mode - simulate emergency phrase detection
        updateStatus("Listening for emergency type...")
        isEmergencyMode = true
        
        // Simulate emergency detection after 3 seconds
        lifecycleScope.launch {
            delay(3000)
            if (isEmergencyMode) {
                updateStatus("Emergency detected: Drowning - Providing guidance...")
                activateEmergency("drowning")
            }
        }
        
        Log.d(TAG, "Emergency listening started (demo mode)")
    }
    
    private fun startEmergencyListeningWithCountdown() {
        updateStatus("Say emergency type (e.g., 'drowning', 'heart attack')")
        isEmergencyMode = true
        isListening = true
        
        // Start a 5-second countdown
        lifecycleScope.launch {
            for (i in 5 downTo 1) {
                updateStatus("Listening for emergency type... ($i)")
                delay(1000)
            }
            // After countdown, stop listening and reset
            updateStatus("Emergency listening stopped. Ready for activation.")
            isEmergencyMode = false
            isListening = false
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
    
    private fun openHelp() {
        updateStatus("Opening help...")
        // Open info activity
        val intent = Intent(this, InfoActivity::class.java)
        startActivity(intent)
        Log.d(TAG, "Help requested")
    }

    private fun openTrainingCenter() {
        updateStatus("Opening training center...")
        val intent = Intent(this, TrainingActivity::class.java)
        startActivityForResult(intent, TRAINING_REQUEST_CODE)
        Log.d(TAG, "Training center requested")
    }
    
    private fun openCrossAppChallenges() {
        updateStatus("Opening cross-app challenges...")
        val intent = Intent(this, CrossAppChallengesActivity::class.java)
        startActivityForResult(intent, CHALLENGES_REQUEST_CODE)
        Log.d(TAG, "Cross-app challenges requested")
    }
    
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        
        if (requestCode == TRAINING_REQUEST_CODE && resultCode == RESULT_OK) {
            // Training was completed, refresh the SOS Hero Status
            updateSOSHeroStatus()
            Toast.makeText(this, "🎉 Training rewards updated!", Toast.LENGTH_SHORT).show()
        } else if (requestCode == CHALLENGES_REQUEST_CODE && resultCode == RESULT_OK) {
            // Cross-app challenge was completed, refresh the SOS Hero Status
            updateSOSHeroStatus()
            Toast.makeText(this, "🎉 Cross-app challenge rewards updated!", Toast.LENGTH_SHORT).show()
        }
    }
    
    override fun onResume() {
        super.onResume()
        // Reset status when returning from settings
        updateStatus("Ready for emergency activation")
        // Update safety features status when returning from settings
        updateSafetyFeaturesStatus()
        // Update SOS Hero Status with real data
        updateSOSHeroStatus()
        Log.d(TAG, "MainActivity resumed - updated safety features status")
    }
    
    private fun updateSOSHeroStatus() {
        val trainingPrefs = getSharedPreferences("training_progress", MODE_PRIVATE)
        val totalBonk = trainingPrefs.getInt("total_bonk", 0)
        val totalSkr = trainingPrefs.getInt("total_skr", 0)
        
        // Calculate XP based on completed training modules
        val cprCompleted = trainingPrefs.getBoolean("cpr_completed", false)
        val firstAidCompleted = trainingPrefs.getBoolean("first_aid_completed", false)
        val voiceCompleted = trainingPrefs.getBoolean("voice_completed", false)
        val featuresCompleted = trainingPrefs.getBoolean("features_completed", false)
        val emergencyTypesCompleted = trainingPrefs.getBoolean("emergency_types_completed", false)
        
        val completedModules = listOf(cprCompleted, firstAidCompleted, voiceCompleted, featuresCompleted, emergencyTypesCompleted).count { it }
        val totalXP = completedModules * 250 // 250 XP per completed module
        
        // Add daily check-in bonus
        val dailyCheckInXP = checkDailyCheckIn()
        val totalXPWithBonus = totalXP + dailyCheckInXP
        
        // Calculate hero level based on XP
        val heroLevel = when {
            totalXPWithBonus >= 1000 -> 5
            totalXPWithBonus >= 750 -> 4
            totalXPWithBonus >= 500 -> 3
            totalXPWithBonus >= 250 -> 2
            else -> 1
        }
        
        // Calculate progress to next level
        val progressToNext = when (heroLevel) {
            1 -> totalXPWithBonus.toFloat() / 250f
            2 -> (totalXPWithBonus - 250).toFloat() / 250f
            3 -> (totalXPWithBonus - 500).toFloat() / 250f
            4 -> (totalXPWithBonus - 750).toFloat() / 250f
            else -> 1.0f
        }
        
        // Update the UI
        binding.tvHeroLevel.text = "Hero Level: $heroLevel"
        binding.tvHeroXP.text = "XP: $totalXPWithBonus"
        binding.tvBonkBalance.text = "BONK: $totalBonk"
        binding.tvSkrBalance.text = "SKR: $totalSkr"
        
        // Update progress bar and next level text
        if (heroLevel < 5) {
            val nextLevelXP = when (heroLevel) {
                1 -> 250
                2 -> 500
                3 -> 750
                4 -> 1000
                else -> 1000
            }
            val xpNeeded = nextLevelXP - totalXPWithBonus
            val progressPercentage = ((totalXPWithBonus - (nextLevelXP - 250)) * 100 / 250).toInt()
            
            binding.tvNextLevel.text = "Next level: $xpNeeded XP needed"
            binding.progressBar.progress = progressPercentage.coerceIn(0, 100)
        } else {
            binding.tvNextLevel.text = "Maximum level reached!"
            binding.progressBar.progress = 100
        }
        
        Log.d(TAG, "SOS Hero Status updated - Level: $heroLevel, XP: $totalXPWithBonus, BONK: $totalBonk, SKR: $totalSkr")
    }
    
    private fun checkDailyCheckIn(): Int {
        val prefs = getSharedPreferences("engagement", MODE_PRIVATE)
        val today = System.currentTimeMillis() / (24 * 60 * 60 * 1000) // Days since epoch
        val lastCheckIn = prefs.getLong("last_check_in", 0)
        val currentStreak = prefs.getInt("check_in_streak", 0)
        
        if (today > lastCheckIn) {
            // New day, give check-in bonus
            val newStreak = if (today - lastCheckIn == 1L) currentStreak + 1 else 1
            val bonusXP = newStreak * 10 // 10 XP per day in streak
            
            prefs.edit()
                .putLong("last_check_in", today)
                .putInt("check_in_streak", newStreak)
                .apply()
            
            // Check for streak achievements
            checkStreakAchievements(newStreak)
            
            if (newStreak > 1) {
                Toast.makeText(this, "🔥 $newStreak day streak! +$bonusXP XP", Toast.LENGTH_SHORT).show()
            } else {
                Toast.makeText(this, "✅ Daily check-in! +$bonusXP XP", Toast.LENGTH_SHORT).show()
            }
            
            return bonusXP
        }
        
        return 0
    }
    
    private fun checkStreakAchievements(streak: Int) {
        val prefs = getSharedPreferences("achievements", MODE_PRIVATE)
        
        when (streak) {
            7 -> {
                if (!prefs.getBoolean("week_streak", false)) {
                    prefs.edit().putBoolean("week_streak", true).apply()
                    showAchievement("🏆 Week Warrior", "7-day check-in streak!", 50)
                }
            }
            30 -> {
                if (!prefs.getBoolean("month_streak", false)) {
                    prefs.edit().putBoolean("month_streak", true).apply()
                    showAchievement("👑 Monthly Master", "30-day check-in streak!", 200)
                }
            }
            100 -> {
                if (!prefs.getBoolean("century_streak", false)) {
                    prefs.edit().putBoolean("century_streak", true).apply()
                    showAchievement("💎 Century Hero", "100-day check-in streak!", 500)
                }
            }
        }
    }
    
    private fun showAchievement(title: String, description: String, bonusXP: Int) {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🎉 $title")
            .setMessage("$description\n\n+$bonusXP XP Bonus!")
            .setPositiveButton("Share Achievement") { dialog, _ ->
                dialog.dismiss()
                // Add bonus XP to training progress
                val trainingPrefs = getSharedPreferences("training_progress", MODE_PRIVATE)
                val currentXP = trainingPrefs.getInt("bonus_xp", 0)
                trainingPrefs.edit().putInt("bonus_xp", currentXP + bonusXP).apply()
                updateSOSHeroStatus()
                
                // Encourage sharing
                showShareDialog(title, description)
            }
            .setNeutralButton("Awesome!") { dialog, _ ->
                dialog.dismiss()
                // Add bonus XP to training progress
                val trainingPrefs = getSharedPreferences("training_progress", MODE_PRIVATE)
                val currentXP = trainingPrefs.getInt("bonus_xp", 0)
                trainingPrefs.edit().putInt("bonus_xp", currentXP + bonusXP).apply()
                updateSOSHeroStatus()
            }
            .setCancelable(false)
            .create()
        
        dialog.show()
    }
    
    private fun showShareDialog(title: String, description: String) {
        val shareText = "🎉 I just unlocked '$title' in Solana SOS! $description\n\nJoin me in becoming a life-saving hero! 🚨💪"
        
        val dialog = AlertDialog.Builder(this)
            .setTitle("Share Your Achievement")
            .setMessage("Inspire others to become emergency response heroes!")
            .setPositiveButton("Share") { dialog, _ ->
                dialog.dismiss()
                val intent = Intent(Intent.ACTION_SEND).apply {
                    type = "text/plain"
                    putExtra(Intent.EXTRA_TEXT, shareText)
                }
                startActivity(Intent.createChooser(intent, "Share Achievement"))
                
                // Give extra bonus for sharing
                val trainingPrefs = getSharedPreferences("training_progress", MODE_PRIVATE)
                val currentXP = trainingPrefs.getInt("bonus_xp", 0)
                trainingPrefs.edit().putInt("bonus_xp", currentXP + 10).apply()
                updateSOSHeroStatus()
                Toast.makeText(this, "✅ +10 XP for sharing!", Toast.LENGTH_SHORT).show()
            }
            .setNegativeButton("Maybe Later") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }
    
    private fun updateStatus(message: String) {
        binding.tvStatus.text = message
        Log.d(TAG, "Status updated: $message")
    }
    
    private fun updateSafetyFeaturesStatus() {
        val settings = getSharedPreferences("settings", MODE_PRIVATE)
        
        // Get toggle states from settings (using correct setting names)
        val silentSOSPowerEnabled = settings.getBoolean("silent_sos_power_enabled", false)
        val silentSOSVolumeEnabled = settings.getBoolean("silent_sos_volume_enabled", false)
        val crashDetectionEnabled = settings.getBoolean("crash_detection_enabled", false)
        val voiceRecognitionEnabled = settings.getBoolean("voice_recognition_enabled", true)
        val locationSharingEnabled = settings.getBoolean("location_sharing_enabled", true)
        val walletConnected = settings.getBoolean("wallet_connected", false)
        
        // Check internet connectivity for hybrid mode
        val isOnline = isNetworkAvailable()
        
        // Update safety features display with colors
        updateFeatureStatus(binding.tvSilentSOS, "• Silent SOS Power", silentSOSPowerEnabled)
        updateFeatureStatus(binding.tvSilentSOSVolume, "• Silent SOS Volume", silentSOSVolumeEnabled)
        updateFeatureStatus(binding.tvCrashDetection, "• Crash Detection", crashDetectionEnabled)
        updateFeatureStatus(binding.tvVoiceRecognition, "• Voice Recognition", voiceRecognitionEnabled)
        updateFeatureStatus(binding.tvTrustedNetwork, "• Personalized Emergency Contacts", true) // Always enabled by default
        updateFeatureStatus(binding.tvOfflineMode, "• Offline Only Mode", !isOnline) // Only active when offline
        updateFeatureStatus(binding.tvContextAware, "• Context-Aware", true) // Always enabled by default
        updateFeatureStatus(binding.tvHybridMode, "• Hybrid Offline/Online Mode", isOnline) // Only active when online
        updateFeatureStatus(binding.tvLocationSharing, "• Location Sharing", locationSharingEnabled)
        updateFeatureStatus(binding.tvBlockchain, "• Blockchain Secure", walletConnected) // Only enabled when wallet connected

        // Update wallet status
        // updateWalletStatus() // Removed wallet status

        Log.d(TAG, "Safety features status updated")
    }
    

    
    private fun showDemoModeDialog() {
        Log.d(TAG, "Demo button pressed - starting comprehensive demo")
        
        try {
            // Step 1: Show demo start
            Toast.makeText(this, "🎬 Demo Mode: Emergency simulation starting", Toast.LENGTH_SHORT).show()
            updateStatus("DEMO: Emergency simulation starting...")
            
            // Step 2: Show emergency selection with custom dialog
            val dialogView = layoutInflater.inflate(R.layout.demo_selection_dialog, null)
            
            val dialog = AlertDialog.Builder(this)
                .setTitle("🎬 Demo Mode - Select Emergency")
                .setView(dialogView)
                .setNegativeButton("Cancel") { dialog, _ ->
                    updateStatus("Demo cancelled. Ready for emergency activation.")
                    dialog.dismiss()
                }
                .create()
            
            // Set up button click listeners
            dialogView.findViewById<Button>(R.id.btnDrowning).setOnClickListener {
                dialog.dismiss()
                startComprehensiveDemo("Drowning")
            }
            
            dialogView.findViewById<Button>(R.id.btnHeartAttack).setOnClickListener {
                dialog.dismiss()
                startComprehensiveDemo("Heart Attack")
            }
            
            dialogView.findViewById<Button>(R.id.btnChoking).setOnClickListener {
                dialog.dismiss()
                startComprehensiveDemo("Choking")
            }
            
            dialogView.findViewById<Button>(R.id.btnBleeding).setOnClickListener {
                dialog.dismiss()
                startComprehensiveDemo("Bleeding")
            }
            
            dialogView.findViewById<Button>(R.id.btnStroke).setOnClickListener {
                dialog.dismiss()
                startComprehensiveDemo("Stroke")
            }
            
            dialog.show()
            Log.d(TAG, "Demo emergency selection dialog shown")
            
        } catch (e: Exception) {
            Log.e(TAG, "Error in demo", e)
            Toast.makeText(this, "Demo mode error: ${e.message}", Toast.LENGTH_SHORT).show()
        }
    }
    
    private fun startComprehensiveDemo(emergencyType: String) {
        Log.d(TAG, "Starting comprehensive demo for: $emergencyType")
        Toast.makeText(this, "🎬 Starting demo for: $emergencyType", Toast.LENGTH_SHORT).show()
        
        lifecycleScope.launch {
            try {
                // Step 1: Voice recognition simulation
                Log.d(TAG, "Demo Step 1: Voice recognition")
                updateStatus("DEMO: 'Hey SOS' detected")
                delay(1000)
                
                updateStatus("DEMO: Emergency type recognized: $emergencyType")
                delay(1000)
                
                // Step 2: Immediate emergency response (concurrent actions)
                Log.d(TAG, "Demo Step 2: Immediate emergency response")
                updateStatus("DEMO: Calling 911 with location...")
                delay(500)
                
                updateStatus("DEMO: Sharing precise location with emergency services")
                delay(500)
                
                // Step 3: Show emergency instructions with 10-second countdown
                Log.d(TAG, "Demo Step 3: Showing instructions")
                showEmergencyInstructions(emergencyType)
                
                // 10-second countdown for emergency response
                
            } catch (e: Exception) {
                Log.e(TAG, "Error in comprehensive demo", e)
                updateStatus("Demo completed with errors")
            }
        }
    }
    
    private fun showEmergencyInstructions(emergencyType: String) {
        val instructions = when (emergencyType) {
            "Drowning" -> """
                Step 1: Assess the situation
                • Is the person still in water?
                • Are they conscious?
                
                Step 2: If still in water
                • Call for help immediately
                • Reach with a pole or throw a flotation device
                • Don't enter water unless trained
                
                Step 3: If out of water
                • Check breathing and pulse
                • Begin CPR if needed
                • Keep warm and monitor
                
                DEMO: App automatically calls 911 and shares location
                DEMO: 911 operator stays on standby for when you are ready
            """.trimIndent()
            
            "Heart Attack" -> """
                Step 1: Recognize symptoms
                • Chest pain or pressure
                • Shortness of breath
                • Pain in arms, neck, jaw
                
                Step 2: Immediate actions
                • App calls 911 immediately
                • Have person sit or lie down
                • Loosen tight clothing
                
                Step 3: Monitor and support
                • Stay calm and reassuring
                • Monitor breathing
                • Be ready for CPR if needed
                
                DEMO: App automatically calls 911 and shares location
                DEMO: 911 operator stays on standby for when you are ready
            """.trimIndent()
            
            "Choking" -> """
                Step 1: Assess severity
                • Can they speak or cough?
                • Are they making noise?
                
                Step 2: If severe choking
                • Perform Heimlich maneuver
                • 5 back blows, 5 abdominal thrusts
                • App calls 911 if not resolved
                
                Step 3: If mild choking
                • Encourage coughing
                • Monitor breathing
                • Seek medical attention
                
                DEMO: App automatically calls 911 and shares location
                DEMO: 911 operator stays on standby for when you are ready
            """.trimIndent()
            
            "Bleeding" -> """
                Step 1: Assess the bleeding
                • Is it severe or minor?
                • Is blood spurting or flowing?
                
                Step 2: Apply direct pressure
                • Use clean cloth or gauze
                • Apply firm pressure
                • Elevate if possible
                
                Step 3: Severe bleeding
                • App calls 911 immediately
                • Apply tourniquet if needed
                • Keep pressure until help arrives
                
                DEMO: App automatically calls 911 and shares location
                DEMO: 911 operator stays on standby for when you are ready
            """.trimIndent()
            
            "Stroke" -> """
                Step 1: FAST assessment
                • Face: Is one side drooping?
                • Arms: Can they raise both arms?
                • Speech: Is speech slurred?
                • Time: App calls 911 immediately
                
                Step 2: Immediate actions
                • App calls 911 right away
                • Note time symptoms started
                • Keep person calm and still
                
                Step 3: While waiting
                • Monitor breathing
                • Don't give food or drink
                • Be ready to provide information
                
                DEMO: App automatically calls 911 and shares location
                DEMO: 911 operator stays on standby for when you are ready
            """.trimIndent()
            
            else -> "DEMO: Emergency response simulation"
        }
        
        AlertDialog.Builder(this)
            .setTitle("🚨 DEMO: $emergencyType Emergency")
            .setMessage(instructions)
            .setPositiveButton("Continue Demo") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(false)
            .show()
    }
    
    private fun showDemoCompletion(emergencyType: String) {
        val completionMessage = """
            ✅ DEMO COMPLETED: $emergencyType Emergency Response
            
            Actions Simulated:
            • Voice recognition: "Hey SOS" detected
            • Emergency type: $emergencyType identified
            • Immediate 911 call: Automatic emergency services contact
            • Location sharing: Precise coordinates sent to emergency services
            • Life-saving instructions: Provided step-by-step guidance
            • Blockchain logging: Emergency recorded on Solana
            • Token rewards: SOS Hero tokens awarded
            
            Real App Features:
            • Offline voice recognition (Vosk)
            • Context-aware guidance
            • Automatic 911 dialing
            • Precise location sharing
            • Solana blockchain verification
            • SOS Hero token rewards
            • Offline mode: Life-saving instructions available without internet
            • Hybrid mode: Seamless online/offline transition
            
            DEMO: This simulation shows the complete emergency response flow
        """.trimIndent()
        
        AlertDialog.Builder(this)
            .setTitle("🎬 Demo Complete")
            .setMessage(completionMessage)
            .setPositiveButton("End Demo") { dialog, _ ->
                Toast.makeText(this, "DEMO: Awarded SKR/BONK tokens! Total: 1,250", Toast.LENGTH_LONG).show()
                updateStatus("Demo completed. Ready for emergency activation.")
                dialog.dismiss()
            }
            .setCancelable(false)
            .show()
    }
    
    private fun updateFeatureStatus(textView: TextView, text: String, isEnabled: Boolean) {
        val statusIcon = if (isEnabled) "•" else "✗"
        textView.text = "$statusIcon ${text.substring(2)}" // Remove the bullet point from the text
        textView.setTextColor(
            ContextCompat.getColor(
                this,
                if (isEnabled) R.color.status_success else R.color.status_inactive
            )
        )
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

    private fun isNetworkAvailable(): Boolean {
        val connectivityManager = getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
        val network = connectivityManager.activeNetwork
        val capabilities = connectivityManager.getNetworkCapabilities(network)
        return capabilities != null && (
            capabilities.hasTransport(NetworkCapabilities.TRANSPORT_WIFI) ||
            capabilities.hasTransport(NetworkCapabilities.TRANSPORT_CELLULAR)
        )
    }
} 
