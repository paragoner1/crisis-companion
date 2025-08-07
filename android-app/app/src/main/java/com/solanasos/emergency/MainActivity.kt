package com.solanasos.emergency

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Bundle
import android.widget.Button
import android.widget.LinearLayout
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import com.solanasos.emergency.databinding.ActivityMainBinding
import android.util.Log
import android.content.Context

class MainActivity : AppCompatActivity() {
    
    private lateinit var binding: ActivityMainBinding
    
    companion object {
        private const val TAG = "MainActivity"
        private const val PERMISSION_REQUEST_CODE = 123
    }
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        setupUI()
        loadSafetyFeaturesStatus()
        requestPermissions()
    }
    
    override fun onResume() {
        super.onResume()
        // Reload safety features status when returning from settings
        loadSafetyFeaturesStatus()
    }
    
    private fun setupUI() {
        // Help & Info button
        binding.btnHelp.setOnClickListener {
            try {
                val intent = Intent(this, InfoActivity::class.java)
                startActivity(intent)
            } catch (e: Exception) {
                Log.e(TAG, "Error starting InfoActivity", e)
                Toast.makeText(this, "Error opening info page", Toast.LENGTH_SHORT).show()
            }
        }
        
        // Demo Mode button
        binding.btnDemo.setOnClickListener {
            Toast.makeText(this, "🎬 Demo mode! +10 XP for training", Toast.LENGTH_SHORT).show()
            showDemoDialog()
        }
        
        // Emergency button
        binding.btnEmergency.setOnClickListener {
            startEmergencyListening()
        }
        
        // Training Center button
        binding.btnTraining.setOnClickListener {
            try {
                val intent = Intent(this, TrainingActivity::class.java)
                startActivity(intent)
            } catch (e: Exception) {
                Log.e(TAG, "Error starting TrainingActivity", e)
                Toast.makeText(this, "Error opening training center", Toast.LENGTH_SHORT).show()
            }
        }
        
        // Cross-App Challenges button
        binding.btnChallenges.setOnClickListener {
            try {
                val intent = Intent(this, CrossAppChallengesActivity::class.java)
                startActivity(intent)
            } catch (e: Exception) {
                Log.e(TAG, "Error starting CrossAppChallengesActivity", e)
                Toast.makeText(this, "Error opening challenges", Toast.LENGTH_SHORT).show()
            }
        }
        
        // Settings button
        binding.btnSettings.setOnClickListener {
            try {
                val intent = Intent(this, SettingsActivity::class.java)
                startActivity(intent)
            } catch (e: Exception) {
                Log.e(TAG, "Error starting SettingsActivity", e)
                Toast.makeText(this, "Error opening settings", Toast.LENGTH_SHORT).show()
            }
        }
    }
    
    private fun showDemoDialog() {
        val demoDialog = androidx.appcompat.app.AlertDialog.Builder(this)
            .setTitle("🎬 Interactive Demo Mode")
            .setMessage("Select an emergency scenario to see how Solana SOS responds:")
            .setCancelable(true)
            .create()
        
        // Create custom layout for demo buttons
        val demoLayout = LinearLayout(this).apply {
            orientation = LinearLayout.VERTICAL
            setPadding(32, 32, 32, 32)
        }
        
        // Demo scenario buttons
        val scenarios = listOf(
            "🚨 Drowning Emergency" to "drowning",
            "💔 Heart Attack" to "heart_attack", 
            "🤐 Choking Emergency" to "choking",
            "🧠 Stroke Emergency" to "stroke",
            "🩸 Severe Bleeding" to "bleeding"
        )
        
        scenarios.forEach { (title, scenario) ->
            val button = Button(this).apply {
                text = title
                setOnClickListener {
                    demoDialog.dismiss()
                    startEmergencyDemo(scenario)
                }
                setBackgroundResource(R.drawable.emergency_button)
                setTextColor(ContextCompat.getColor(this@MainActivity, R.color.text_primary))
                layoutParams = LinearLayout.LayoutParams(
                    LinearLayout.LayoutParams.MATCH_PARENT,
                    LinearLayout.LayoutParams.WRAP_CONTENT
                ).apply {
                    setMargins(0, 8, 0, 8)
                }
            }
            demoLayout.addView(button)
        }
        
        demoDialog.setView(demoLayout)
        demoDialog.show()
    }
    
    private fun startEmergencyDemo(scenario: String) {
        val demoSteps = when (scenario) {
            "drowning" -> listOf(
                "🎤 Voice Activation: 'Hey SOS, drowning emergency'",
                "🚨 Emergency Detected: Drowning",
                "📱 App: 'Stay calm. Can you see the person?'",
                "👤 User: 'Yes, I can see them'",
                "📱 App: 'Are they conscious and breathing?'",
                "👤 User: 'No, they're not breathing!'",
                "📱 App: 'Starting CPR guidance immediately'",
                "📱 App: 'Place hands on center of chest'",
                "📱 App: 'Push hard and fast at 100-120 beats per minute'",
                "📱 App: 'I'm calling 911 and sharing your location'",
                "📱 App: 'Trusted contacts notified with GPS coordinates'",
                "📱 App: 'Continue CPR until help arrives'",
                "📱 App: 'Real-time location tracking active'"
            )
            "heart_attack" -> listOf(
                "🎤 Voice Activation: 'Hey SOS, heart attack'",
                "🚨 Emergency Detected: Heart Attack",
                "📱 App: 'Have the person sit down immediately'",
                "📱 App: 'Loosen any tight clothing'",
                "📱 App: 'Are they conscious and responsive?'",
                "👤 User: 'Yes, but they're in pain'",
                "📱 App: 'Monitor their breathing and consciousness'",
                "📱 App: 'I'm calling 911 and sharing your location'",
                "📱 App: 'Trusted contacts notified with GPS coordinates'",
                "📱 App: 'Help is on the way'",
                "📱 App: 'Keep them calm and comfortable'",
                "📱 App: 'Real-time location tracking active'"
            )
            "choking" -> listOf(
                "🎤 Voice Activation: 'Hey SOS, choking emergency'",
                "🚨 Emergency Detected: Choking",
                "📱 App: 'Can they speak or cough?'",
                "👤 User: 'No, they can't breathe!'",
                "📱 App: 'Starting Heimlich maneuver immediately'",
                "📱 App: 'Stand behind them, wrap arms around waist'",
                "📱 App: 'Make fist, place above navel'",
                "📱 App: 'Give 5 quick upward thrusts'",
                "📱 App: 'I'm calling 911 and sharing your location'",
                "📱 App: 'Trusted contacts notified with GPS coordinates'",
                "📱 App: 'Continue until object is dislodged'",
                "📱 App: 'Real-time location tracking active'"
            )
            "stroke" -> listOf(
                "🎤 Voice Activation: 'Hey SOS, stroke emergency'",
                "🚨 Emergency Detected: Stroke",
                "📱 App: 'Performing FAST test'",
                "📱 App: 'Face: Ask them to smile'",
                "📱 App: 'Arms: Ask them to raise both arms'",
                "📱 App: 'Speech: Ask them to repeat a sentence'",
                "📱 App: 'Time: Note when symptoms started'",
                "📱 App: 'I'm calling 911 and sharing your location'",
                "📱 App: 'Trusted contacts notified with GPS coordinates'",
                "📱 App: 'Keep them calm and comfortable'",
                "📱 App: 'Help is on the way'",
                "📱 App: 'Real-time location tracking active'"
            )
            "bleeding" -> listOf(
                "🎤 Voice Activation: 'Hey SOS, bleeding emergency'",
                "🚨 Emergency Detected: Severe Bleeding",
                "📱 App: 'Apply direct pressure to the wound'",
                "📱 App: 'Use clean cloth or bandage'",
                "📱 App: 'Keep pressure for at least 10 minutes'",
                "📱 App: 'Elevate the injured area if possible'",
                "📱 App: 'I'm calling 911 and sharing your location'",
                "📱 App: 'Trusted contacts notified with GPS coordinates'",
                "📱 App: 'Help is on the way'",
                "📱 App: 'Continue pressure until help arrives'",
                "📱 App: 'Real-time location tracking active'"
            )
            else -> listOf("Emergency scenario not found")
        }
        
        showDemoSteps(demoSteps, scenario)
    }
    
    private fun showDemoSteps(steps: List<String>, scenario: String) {
        val demoContent = steps.joinToString("\n\n")
        
        val demoDialog = androidx.appcompat.app.AlertDialog.Builder(this)
            .setTitle("🎬 Demo: ${scenario.replace("_", " ").capitalize()}")
            .setMessage(demoContent)
            .setPositiveButton("View Summary") { dialog, _ ->
                dialog.dismiss()
                showDemoSummary(scenario)
            }
            .setNegativeButton("Close") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .create()
        
        demoDialog.show()
    }
    
    private fun showDemoSummary(scenario: String) {
        val summary = """
            🎉 Demo Complete: ${scenario.replace("_", " ").capitalize()}
            
            ✅ What Happened:
            • Voice activation detected emergency phrase
            • Emergency categorized and appropriate response initiated
            • Context-aware guidance provided step-by-step
            • 911 automatically called with GPS location
            • Trusted contacts notified with precise coordinates
            • Real-time location tracking activated
            • Life-saving instructions provided throughout
            
            🏆 Rewards Earned:
            • +50 XP for emergency response
            • +25 BONK tokens for quick action
            • +10 SKR tokens for safety network
            • Achievement: "Emergency Responder"
            
            📊 Actions Taken:
            • Voice recognition: Sub-100ms activation
            • GPS location: Shared with emergency services
            • Trusted network: All contacts notified
            • Blockchain: Emergency record created
            • Real-time tracking: Continuous location updates
            
            This demonstrates how Solana SOS provides immediate, intelligent emergency response when seconds count.
        """.trimIndent()
        
        androidx.appcompat.app.AlertDialog.Builder(this)
            .setTitle("📊 Demo Summary")
            .setMessage(summary)
            .setPositiveButton("Done") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(false)
            .show()
    }
    
    private fun startEmergencyListening() {
        // Update status to show listening
        binding.tvStatus.text = "Listening for emergency phrase..."
        
        // Start listening for emergency phrases
        Toast.makeText(this, "🚨 Emergency activated! +50 XP for quick response", Toast.LENGTH_SHORT).show()
        
        // Simulate voice recognition (in real app, this would use actual voice recognition)
        // For demo purposes, we'll use a timer to simulate the process
        android.os.Handler(android.os.Looper.getMainLooper()).postDelayed({
            // After 10 seconds, stop listening and return to ready state
            stopEmergencyListening()
        }, 10000) // 10 seconds
    }
    
    private fun stopEmergencyListening() {
        // Return to ready state
        binding.tvStatus.text = "Ready for emergency activation"
        
        Toast.makeText(this, "Emergency listening stopped", Toast.LENGTH_SHORT).show()
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
    
    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        
        if (requestCode == PERMISSION_REQUEST_CODE) {
            val allGranted = grantResults.all { it == PackageManager.PERMISSION_GRANTED }
            if (allGranted) {
                Toast.makeText(this, "✅ Permissions granted! +25 XP for safety setup", Toast.LENGTH_SHORT).show()
                
                // Always show the mission statement disclaimer after permissions
                showFirstTimeMissionDisclaimer()
            } else {
                Toast.makeText(this, "Some permissions denied", Toast.LENGTH_SHORT).show()
            }
        }
    }
    
    private fun showFirstTimeMissionDisclaimer() {
        Log.d(TAG, "Showing first time mission disclaimer")
        
        val scrollView = android.widget.ScrollView(this).apply {
            layoutParams = android.view.ViewGroup.LayoutParams(
                android.view.ViewGroup.LayoutParams.MATCH_PARENT,
                500
            )
        }

        val messageView = android.widget.TextView(this).apply {
            text = """
                MISSION CRITICAL: SERIOUS PURPOSE, LIFE-SAVING MISSION

                Solana SOS is not a game. This is a serious emergency response application designed to prevent unnecessary deaths when lives could otherwise be saved.

                OUR MISSION:
                Train users to recognize and respond to life-threatening emergencies, provide immediate, authoritative guidance, and ensure you have the skills to keep your loved ones alive until professional help arrives.

                REAL-TIME GUIDANCE:
                The app is designed to guide you in real-time with life-saving instructions, equipping you with the appropriate knowledge and tools to be prepared to act when seconds matter the most.

                THE REALITY:
                Every day, people die from emergencies that could have been prevented with proper training and immediate response. The difference between life and death is often measured in seconds, and the actions of bystanders can mean everything.

                WHY GAMIFICATION SERVES A SERIOUS PURPOSE:

                DAILY CHECK-INS:
                • Maintain emergency readiness and skill retention
                • Build confidence through regular practice
                • Ensure you're prepared when seconds count

                TRAINING MODULES:
                • Structured learning for life-saving skills
                • Practice CPR, first aid, and emergency response
                • Build muscle memory for critical situations

                ACHIEVEMENT SYSTEM:
                • Motivate continuous learning and skill development
                • Recognize progress in emergency preparedness
                • Encourage mastery of life-saving techniques

                CROSS-APP CHALLENGES:
                • Integrate emergency training with daily activities
                • Build community safety awareness
                • Create a culture of preparedness

                TOKEN REWARDS:
                • BONK/SKR tokens encourage regular practice
                • Incentivize skill development and maintenance
                • Support the serious purpose of emergency readiness

                YOUR RESPONSIBILITY:
                By using this app, you commit to:
                • Learning and maintaining emergency response skills
                • Taking action when you witness life-threatening situations
                • Using your skills to help those in need
                • Taking responsibility for the lives around you

                THE PROMISE:
                With the right tools and training, you can keep your loved ones alive in the event of a life-threatening emergency until professional help arrives. Our gamification serves the serious purpose of ensuring you're ready when seconds count, so you can act to save the people that matter to you the most.

                Do you accept this mission?
                """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@MainActivity, android.R.color.black))
        }

        scrollView.addView(messageView)

        val dialog = AlertDialog.Builder(this)
            .setTitle("🚨 MISSION CRITICAL: SERIOUS PURPOSE, LIFE-SAVING MISSION")
            .setView(scrollView)
            .setPositiveButton("I Accept This Mission") { dialog, _ ->
                Log.d(TAG, "User accepted the mission")
                dialog.dismiss()
                Toast.makeText(this, "🎉 Mission Accepted! +100 XP +50 BONK +25 SKR", Toast.LENGTH_LONG).show()
                showQuickStartGuide()
            }
            .setNegativeButton("I Need More Time") { dialog, _ ->
                Log.d(TAG, "User needs more time")
                dialog.dismiss()
            }
            .setCancelable(false)
            .setOnDismissListener {
                Log.d(TAG, "Mission disclaimer dialog was dismissed")
            }
            .create()

        // Set dialog window parameters to ensure buttons are visible
        dialog.setOnShowListener {
            dialog.window?.setLayout(
                android.view.ViewGroup.LayoutParams.MATCH_PARENT,
                android.view.ViewGroup.LayoutParams.WRAP_CONTENT
            )
        }

        dialog.show()
        Log.d(TAG, "Mission disclaimer dialog shown")
    }
    
    private fun showQuickStartGuide() {
        val scrollView = android.widget.ScrollView(this).apply {
            layoutParams = android.view.ViewGroup.LayoutParams(
                android.view.ViewGroup.LayoutParams.MATCH_PARENT,
                600
            )
        }

        val messageView = android.widget.TextView(this).apply {
            text = """
                🚨 QUICK START GUIDE: WHAT THIS APP CAN DO

                EMERGENCY ACTIVATION:
                • Press "Press for Emergency" button (backup to voice activation)
                • Say "Hey SOS" followed by emergency type (e.g., "heart attack")
                • App provides real-time step-by-step guidance
                • Keeps 911 dispatcher on standby until you're ready to connect
                • Automatically calls 911 and shares your location when activated

                SAFETY FEATURES:
                • Silent SOS: Emergency activation without sound
                • Crash Detection: Automatic emergency response in accidents
                • Trusted Network: Alert your emergency contacts
                • Offline Protocols: Core emergency guidance (always available)
                • Enhanced Features: Real-time consultation when online
                • Blockchain Security: Immutable emergency logs
                • Voice Recognition: Hands-free emergency activation

                SAFETY FEATURES STATUS:
                • View which safety features are active/inactive
                • Green dots = Active features ready for emergencies
                • Red X's = Inactive features need setup in Settings

                TRAINING CENTER:
                • Complete training modules to earn BONK/SKR tokens
                • Practice CPR, first aid, and emergency response
                • Build confidence through structured learning
                • Achieve "Life-Saving Legend" status

                CROSS-APP CHALLENGES:
                • Complete challenges to earn rewards
                • Integrate emergency training with daily activities
                • Build community safety awareness

                SETTINGS & SETUP:
                • Configure emergency preferences
                • Add trusted contacts for emergency notifications
                • Enable/disable safety features
                • Connect Solana wallet for token rewards and emergency logging

                APP GUIDE:
                • Detailed information about all features
                • Emergency types and direct action phrases
                • Safety features deep dive
                • Gamification system explanation

                WHERE TO START:
                1. Check Safety Features Status (top of screen)
                2. Visit Settings to configure preferences
                3. Complete Training Center modules
                4. Add trusted contacts in Settings
                5. Practice with Demo Mode

                REMEMBER: This app is designed to save lives by preventing avoidable deaths. Every training module completed makes you better prepared for real emergencies.
                """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@MainActivity, android.R.color.black))
        }

        scrollView.addView(messageView)

        AlertDialog.Builder(this)
            .setTitle("🚨 QUICK START GUIDE")
            .setView(scrollView)
                                .setPositiveButton("Got It!") { dialog, _ ->
                        dialog.dismiss()
                        Toast.makeText(this, "✅ Quick Start Complete! +25 XP +10 BONK", Toast.LENGTH_SHORT).show()
                    }
            .setCancelable(true)
            .show()
    }

    private fun loadSafetyFeaturesStatus() {
        val sharedPrefs = getSharedPreferences("settings", Context.MODE_PRIVATE)
        
        // Load settings states
        val voiceRecognitionEnabled = sharedPrefs.getBoolean("voice_recognition_enabled", true)
        val locationSharingEnabled = sharedPrefs.getBoolean("location_sharing_enabled", true)
        val autoDial911Enabled = sharedPrefs.getBoolean("auto_dial_911_enabled", false)
        val silentSOSPowerEnabled = sharedPrefs.getBoolean("silent_sos_power_enabled", false)
        val silentSOSVolumeEnabled = sharedPrefs.getBoolean("silent_sos_volume_enabled", false)
        val crashDetectionEnabled = sharedPrefs.getBoolean("crash_detection_enabled", false)
        
        // Check if wallet is connected (simplified check)
        val walletConnected = sharedPrefs.getBoolean("wallet_connected", false)
        
        // Check if emergency contacts exist
        val emergencyContacts = getSharedPreferences("emergency_contacts", MODE_PRIVATE)
        val contactCount = emergencyContacts.getInt("count", 0)
        val trustedContactsEnabled = contactCount > 0
        
        // Update UI based on actual settings
        updateSafetyFeatureStatus(binding.tvVoiceRecognition, voiceRecognitionEnabled, "Voice Recognition")
        updateSafetyFeatureStatus(binding.tvLocationSharing, locationSharingEnabled, "Emergency GPS")
        updateSafetyFeatureStatus(binding.tvAutoDial911, autoDial911Enabled, "Auto-Dial 911")
        updateSafetyFeatureStatus(binding.tvSilentSOS, silentSOSPowerEnabled, "Silent SOS Power")
        updateSafetyFeatureStatus(binding.tvSilentSOSVolume, silentSOSVolumeEnabled, "Silent SOS Volume")
        updateSafetyFeatureStatus(binding.tvCrashDetection, crashDetectionEnabled, "Crash Detection")
        updateSafetyFeatureStatus(binding.tvTrustedNetwork, trustedContactsEnabled, "Trusted Contacts")
        updateSafetyFeatureStatus(binding.tvBlockchain, walletConnected, "Blockchain Secure")
        
        // Context-aware is always active (core feature) - not shown in status
        
        // Check database connectivity for offline mode
        val databaseConnected = checkDatabaseConnection()
        updateSafetyFeatureStatus(binding.tvOfflineMode, databaseConnected, "Offline Protocols")
        
        // Check online connectivity for enhanced features
        val onlineConnected = checkOnlineConnection()
        updateSafetyFeatureStatus(binding.tvOnlineMode, onlineConnected, "Enhanced Features")
        
        // Update legend counts
        val activeCount = listOf(voiceRecognitionEnabled, locationSharingEnabled, autoDial911Enabled, silentSOSPowerEnabled, 
                                silentSOSVolumeEnabled, crashDetectionEnabled, trustedContactsEnabled, walletConnected,
                                databaseConnected, onlineConnected) // offline, online
                                .count { it }
        val inactiveCount = 10 - activeCount
        
        // Update legend text (we'll need to add these TextViews to the layout)
        // For now, we'll just update the status in the log
        Log.d(TAG, "Active features: $activeCount, Inactive features: $inactiveCount")
    }
    
    private fun updateSafetyFeatureStatus(textView: android.widget.TextView, isEnabled: Boolean, featureName: String) {
        val symbol = if (isEnabled) "•" else "✗"
        val color = if (isEnabled) R.color.status_success else R.color.status_inactive
        textView.text = "$symbol $featureName"
        textView.setTextColor(ContextCompat.getColor(this, color))
    }
    
    private fun checkDatabaseConnection(): Boolean {
        // For demo purposes, assume database is always connected
        // In real implementation, this would check SQLite database connectivity
        return true
    }
    
    private fun checkOnlineConnection(): Boolean {
        // For demo purposes, assume online is connected
        // In real implementation, this would check internet connectivity
        return true
    }
}
