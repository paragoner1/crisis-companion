package com.solanasos.emergency

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import com.solanasos.emergency.databinding.ActivityInfoBinding
import androidx.core.content.ContextCompat
import android.content.Intent
import androidx.appcompat.app.AlertDialog

class InfoActivity : AppCompatActivity() {
    
    private lateinit var binding: ActivityInfoBinding
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // InfoActivity onCreate
        
        // Set status bar color to match the deep navy background
        window.statusBarColor = ContextCompat.getColor(this, R.color.deep_navy)
        
        binding = ActivityInfoBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        setupUI()
    }
    
    private fun setupUI() {
        // Back button
        binding.btnBack.setOnClickListener {
            finish()
        }

        // Mission Statement section click handler
        binding.missionStatementSection.setOnClickListener {
            showVisualFeedback(binding.missionStatementSection)
            showAppDisclaimer()
        }

        // Emergency Types section click handler
        binding.emergencyTypesSection.setOnClickListener {
            showVisualFeedback(binding.emergencyTypesSection)
            showEmergencyTypesDetails()
        }

        // Safety Features section click handler
        binding.safetyFeaturesSection.setOnClickListener {
            showVisualFeedback(binding.safetyFeaturesSection)
            showSafetyFeaturesDetails()
        }

        // Gamification section click handler
        binding.gamificationSection.setOnClickListener {
            showVisualFeedback(binding.gamificationSection)
            showGamificationDetails()
        }

        // Permissions section click handler
        binding.permissionsSection.setOnClickListener {
            showVisualFeedback(binding.permissionsSection)
            showPermissionsDetails()
        }

        // Hybrid Mode section click handler
        binding.hybridModeSection.setOnClickListener {
            showVisualFeedback(binding.hybridModeSection)
            showHybridModeDetails()
        }

        // Solana Integration section click handler
        binding.solanaIntegrationSection.setOnClickListener {
            showVisualFeedback(binding.solanaIntegrationSection)
            showSolanaIntegrationDetails()
        }

        // Impact section click handler
        binding.impactSection.setOnClickListener {
            showVisualFeedback(binding.impactSection)
            showImpactDetails()
        }

        // Training section click handler
        binding.trainingSection.setOnClickListener {
            showVisualFeedback(binding.trainingSection)
            val intent = Intent(this, TrainingActivity::class.java)
            startActivity(intent)
        }

        // Challenges section click handler
        binding.challengesSection.setOnClickListener {
            showVisualFeedback(binding.challengesSection)
            val intent = Intent(this, CrossAppChallengesActivity::class.java)
            startActivity(intent)
        }

        // Pitch Deck section click handler
        binding.pitchDeckSection.setOnClickListener {
            showVisualFeedback(binding.pitchDeckSection)
        }

        // Hackathon Alignment section click handler
        binding.hackathonAlignmentSection.setOnClickListener {
            showVisualFeedback(binding.hackathonAlignmentSection)
        }
    }
    
    private fun showVisualFeedback(view: android.view.View) {
        // Store original background
        val originalBackground = view.background
        
        // Flash with teal color to match mission statement font
        view.setBackgroundColor(ContextCompat.getColor(this, R.color.soft_green))
        
        // Reset after 200ms
        android.os.Handler(android.os.Looper.getMainLooper()).postDelayed({
            view.background = originalBackground
        }, 200)
    }
    
    private fun showAppDisclaimer() {
        val scrollView = android.widget.ScrollView(this).apply {
            layoutParams = android.view.ViewGroup.LayoutParams(
                android.view.ViewGroup.LayoutParams.MATCH_PARENT,
                500
            )
        }

        val messageView = TextView(this).apply {
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
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        scrollView.addView(messageView)

        val dialog = AlertDialog.Builder(this)
            .setTitle("🚨 MISSION CRITICAL: SERIOUS PURPOSE, LIFE-SAVING MISSION")
            .setView(scrollView)
            .setPositiveButton("I Accept This Mission") { dialog, _ ->
                dialog.dismiss()
            }
            .setNegativeButton("I Need More Time") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(false)
            .create()

        // Set dialog window parameters to ensure buttons are visible
        dialog.setOnShowListener {
            dialog.window?.setLayout(
                android.view.ViewGroup.LayoutParams.MATCH_PARENT,
                android.view.ViewGroup.LayoutParams.WRAP_CONTENT
            )
        }

        dialog.show()
    }

    private fun showEmergencyTypesDetails() {
        val messageView = TextView(this).apply {
            text = """
                EMERGENCY TYPES SUPPORTED

                MEDICAL EMERGENCIES:
                • Heart Attack - Immediate CPR guidance
                • Stroke - FAST test and emergency care
                • Choking - Heimlich maneuver step-by-step
                • Allergic Reaction - EpiPen administration
                • Diabetic Emergency - Blood sugar management

                INJURY EMERGENCIES:
                • Bleeding - Direct pressure and tourniquet
                • Severe Burns - Cooling and emergency treatment
                • Unconscious - Assessment and basic life support
                • Trauma - Assessment and stabilization

                ENVIRONMENTAL EMERGENCIES:
                • Drowning - Water rescue and post-extraction care
                • Poisoning - Poison control and emergency care
                • Seizure - Safety measures and monitoring

                DIRECT ACTION PHRASES

                Immediate Actions (Skip Context Assessment):
                • CPR - Immediate chest compressions and rescue breathing
                • Heimlich - Abdominal thrusts for choking
                • AED - Automated external defibrillator usage
                • Tourniquet - Bleeding control for severe injuries
                • EpiPen - Emergency epinephrine administration
                • Rescue Breathing - Artificial ventilation
                • First Aid - Basic emergency care
                • FAST Test - Stroke assessment (Face, Arm, Speech, Time)
                • Poison Control - Emergency poison management
                • Cool Burn - Burn treatment and cooling
                • Medical Alert - Medical identification check

                USAGE: Say "Hey SOS" followed by any emergency type or direct action phrase for immediate guidance.
                """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("🚨 Complete Emergency Types Guide")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }

    private fun showSafetyFeaturesDetails() {
        val messageView = TextView(this).apply {
            text = """
                SILENT SOS (DANGEROUS SITUATIONS)

                ACTIVATION METHODS:
                • Hold power button for 3 seconds
                • Power button sequence: 5 rapid presses
                • Volume button sequence: Up + down + up + down + up

                FEATURES:
                • No visible indication on screen
                • Trusted contacts alerted immediately
                • 911 called automatically with location
                • Perfect for rideshares, domestic violence, abduction scenarios

                CRASH DETECTION

                TECHNOLOGY:
                • Uses device sensors to detect severe impacts
                • Automatically calls 911 with GPS coordinates
                • Alerts trusted contacts immediately
                • 30-second window to cancel false alarms

                TRUSTED NETWORK

                SETUP:
                • Add family, friends, caregivers as trusted contacts
                • Granular permissions (who gets what alerts)
                • Location sharing controls
                • Community safety - friends arrive before EMS

                BENEFITS:
                • 5-10 minute advantage over emergency services
                • Real-time location tracking
                • Blockchain-secured emergency records
                • Community safety network
                """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("🛡️ Safety Features Deep Dive")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }

    private fun showGamificationDetails() {
        val messageView = TextView(this).apply {
            text = """
                LEVEL SYSTEM

                LEVEL 1: NOVICE HERO
                • Just getting started
                • Complete basic setup
                • Add trusted contacts
                • Earn first XP and tokens

                LEVEL 5: LIFE PROTECTOR
                • Completed emergency training
                • Responded to real emergencies
                • Built safety network
                • Earned "Life Saver" achievements

                LEVEL 10: LIFE-SAVING LEGEND
                • Master of emergency response
                • Helped save multiple lives
                • Built large safety network
                • Recognized as community safety leader

                TRAINING MODULES

                CPR TRAINING (50 BONK + 25 SKR)
                • Adult, child, and infant CPR techniques
                • AED usage and emergency response
                • When to perform CPR and safety protocols
                • Duration: ~10 minutes

                FIRST AID BASICS (30 BONK + 15 SKR)
                • Emergency assessment and response
                • Basic life support techniques
                • Injury treatment and bandaging
                • Emergency communication protocols
                • Duration: ~15 minutes

                VOICE RECOGNITION (25 BONK + 10 SKR)
                • Practice emergency voice commands
                • Clear communication during stress
                • Voice activation optimization
                • Duration: ~5 minutes

                APP FEATURES (20 BONK + 10 SKR)
                • Settings configuration mastery
                • Demo mode usage and practice
                • Safety features optimization
                • Emergency contact management
                • Duration: ~8 minutes

                EMERGENCY TYPES (40 BONK + 20 SKR)
                • Recognition of 12 emergency types
                • Context-aware response protocols
                • Future emergency type expansion
                • Duration: ~12 minutes

                TOTAL POSSIBLE REWARDS: 165 BONK + 80 SKR

                ACHIEVEMENTS
                • CPR Master - Complete CPR training
                • Quick Responder - Fast emergency activation
                • Community Hero - Build large trusted network
                • Life Saver - Successful emergency intervention
                • Training Champion - Complete all modules
                """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("🎮 SOS Hero Gamification System")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }

    private fun showPermissionsDetails() {
        val messageView = TextView(this).apply {
            text = """
                REQUIRED PERMISSIONS:

                MICROPHONE:
                • Purpose: Voice recognition and emergency activation
                • Used for: "Hey SOS" wake word detection
                • Privacy: Only activates during emergencies
                • Security: Audio processed locally when possible

                LOCATION:
                • Purpose: GPS coordinates during emergencies
                • Used for: Automatic 911 calling with location
                • Privacy: Only shared during emergency activation
                • Security: Encrypted location transmission

                CONTACTS:
                • Purpose: Trusted network setup
                • Used for: Emergency contact notifications
                • Privacy: Only contacts you explicitly add
                • Security: Encrypted contact storage

                PHONE:
                • Purpose: Automatic 911 calling
                • Used for: Emergency services integration
                • Privacy: Only during emergency activation
                • Security: Direct emergency services connection

                SETUP PROCESS:

                STEP 1: DOWNLOAD & INSTALL
                • Download from Solana Mobile dApp store
                • Install and launch app
                • Grant essential permissions when prompted

                STEP 2: CREATE HERO PROFILE
                • Choose your starting level
                • Set up emergency preferences
                • Configure notification settings

                STEP 3: ADD TRUSTED CONTACTS
                • Select family, friends, caregivers
                • Set notification preferences per contact
                • Test emergency contact system

                STEP 4: COMPLETE INITIAL TRAINING
                • Take basic emergency training
                • Practice voice commands
                • Earn first rewards

                DAILY USE:
                • App runs in background (like Siri)
                • No battery drain - efficient power usage
                • Privacy focused - only activates during emergencies
                • Works offline - no internet needed for emergencies
                """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("📱 App Permissions & Setup Guide")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }

    private fun showHybridModeDetails() {
        val messageView = TextView(this).apply {
            text = """
                OFFLINE MODE (ALWAYS AVAILABLE):

                VOICE RECOGNITION:
                • Vosk offline voice recognition
                • Works without internet connection
                • Sub-100ms response time
                • Noise filtering via RNNoise

                EMERGENCY GUIDANCE:
                • Local database with 12 emergency types
                • Step-by-step instructions stored locally
                • Context-aware guidance algorithms
                • Direct action phrases available

                SAFETY FEATURES:
                • Silent SOS activation
                • Crash detection via device sensors
                • Trusted network notifications
                • Location tracking and sharing

                ONLINE MODE (ENHANCED):

                AI-POWERED FEATURES:
                • Unlimited emergency types
                • Real-time context analysis
                • Advanced voice recognition
                • Cloud-based emergency protocols

                ENHANCED SERVICES:
                • Smart 911 operator integration
                • Real-time location and contact services
                • Dynamic emergency response adaptation
                • Advanced noise filtering

                HYBRID ARCHITECTURE:

                SEAMLESS HANDOFF:
                • Automatic mode switching based on connectivity
                • Context preservation between modes
                • No loss of critical information
                • Fallback to offline if network lost

                BEST OF BOTH WORLDS:
                • Offline reliability for critical functions
                • Online enhancement when available
                • Continuous emergency response capability
                • Optimal user experience

                TECHNOLOGY STACK:
                • Rust backend for reliability
                • Android JNI for native integration
                • SQLite for local database
                • Solana blockchain for records
                """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("🌐 Offline vs Online Mode Architecture")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }

    private fun showSolanaIntegrationDetails() {
        val messageView = TextView(this).apply {
            text = """
                MOBILE WALLET ADAPTER:

                SEAMLESS INTEGRATION:
                • Direct connection to Solana Mobile wallet
                • Secure token transactions
                • No additional wallet setup required
                • Native Seeker device optimization

                TOKEN REWARDS:
                • SKR tokens for emergency preparedness
                • BONK tokens for community safety actions
                • Automatic reward distribution
                • Transparent blockchain records

                BLOCKCHAIN FEATURES:

                EMERGENCY RECORDS:
                • Tamper-proof documentation on Solana
                • Real-time emergency response logs
                • Community safety coordination
                • Verifiable emergency interventions

                SMART CONTRACTS:
                • Automated reward distribution
                • Community safety incentives
                • Emergency response verification
                • Transparent token economics

                SEEKER DEVICE OPTIMIZATION:

                HARDWARE ACCELERATION:
                • Built specifically for Solana Mobile Seeker
                • Optimized for Seeker hardware
                • Native performance integration
                • Battery-efficient operation

                DAPP STORE READY:
                • Designed for immediate deployment
                • Seeker device compatibility
                • Default app potential
                • Community safety focus

                SOLANA ECOSYSTEM BENEFITS:
                • Fast, low-cost transactions
                • Community-driven development
                • Transparent token economics
                • Scalable safety network
                """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("⚡ Solana Integration Details")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }

    private fun showImpactDetails() {
        val messageView = TextView(this).apply {
            text = """
                POTENTIAL TIME SAVINGS:

                EMERGENCY RESPONSE:
                • Voice activation eliminates manual app opening
                • Context-aware guidance skips irrelevant steps
                • Direct action phrases provide immediate guidance
                • Every second counts in life-threatening situations

                TRAINING BENEFITS:
                • Gamification encourages regular practice
                • Structured training modules improve skills
                • Regular practice reduces panic in emergencies
                • Prepared users are more confident responders

                COMMUNITY SAFETY:

                TRUSTED NETWORK ADVANTAGE:
                • Personal contacts can arrive before emergency services
                • Real-time location sharing with trusted contacts
                • Community safety network activation
                • Multiple responders can coordinate

                NETWORK EFFECTS:
                • Each user makes the community safer
                • Trusted contacts can respond to multiple emergencies
                • Community safety coordination
                • Blockchain-verified emergency responses

                TECHNICAL CAPABILITIES:

                VOICE RECOGNITION:
                • Sub-100ms voice activation time
                • Works in noisy environments
                • Offline functionality for reliability
                • 12 emergency types supported

                SAFETY FEATURES:
                • 11 direct action phrases available
                • Silent SOS for dangerous situations
                • Crash detection for automatic response
                • Real-time location tracking

                COMMUNITY IMPACT:
                • Trusted network activation in seconds
                • Real-time location sharing
                • Blockchain-secured emergency records
                • Solana ecosystem integration

                NOTE: Specific impact statistics will be measured during real-world deployment and user testing.
                """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("📊 Real-World Impact & Benefits")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }

    private fun showTrainingDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🎓 Training & Skill Development")
            .setMessage("""
                **SERIOUS TRAINING FOR LIFE-SAVING SKILLS**
                
                **Available Training Modules:**
                
                **🫀 CPR Training (50 BONK + 25 SKR)**
                • Adult, child, and infant CPR techniques
                • AED usage and emergency response
                • When to perform CPR and safety protocols
                • Duration: ~10 minutes
                
                **🩹 First Aid Basics (30 BONK + 15 SKR)**
                • Emergency assessment and response
                • Basic life support techniques
                • Injury treatment and bandaging
                • Emergency communication protocols
                • Duration: ~15 minutes
                
                **🎤 Voice Recognition (25 BONK + 10 SKR)**
                • Practice emergency voice commands
                • Clear communication during stress
                • Voice activation optimization
                • Duration: ~5 minutes
                
                **⚙️ App Features (20 BONK + 10 SKR)**
                • Settings configuration mastery
                • Demo mode usage and practice
                • Safety features optimization
                • Emergency contact management
                • Duration: ~8 minutes
                
                **🚨 Emergency Types (40 BONK + 20 SKR)**
                • Recognition of 12 emergency types
                • Context-aware response protocols
                • Future emergency type expansion
                • Duration: ~12 minutes
                
                **Total Possible Rewards:** 165 BONK + 80 SKR
                
                **Why Training Matters:**
                Every completed training module means you're better prepared to save a life when seconds count. The gamification exists solely to ensure you develop and maintain these critical skills.
                
                **Daily Check-ins:** Maintain emergency readiness
                **Achievement System:** Motivate continuous learning
                **Progress Tracking:** Monitor skill development
                **Hero Levels:** Recognize life-saving expertise
            """.trimIndent())
            .setPositiveButton("Open Training Center") { dialog, _ ->
                dialog.dismiss()
                val intent = Intent(this, TrainingActivity::class.java)
                startActivity(intent)
            }
            .setNegativeButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showChallengesDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🔄 Cross-App Challenges")
            .setMessage("""
                **COMMUNITY INTEGRATION CHALLENGES**
                
                **Available Challenges:**
                
                **CPR ↔ DeFi Challenge (100 BONK + 50 SKR)**
                • Complete CPR training + DeFi transaction
                • Bridge emergency skills with financial literacy
                • Duration: ~15 minutes
                
                **Emergency ↔ Gaming Challenge (150 BONK + 75 SKR)**
                • Emergency response + gaming achievement
                • Gamify life-saving skills
                • Duration: ~20 minutes
                
                **Safety ↔ Trading Challenge (120 BONK + 60 SKR)**
                • Safety training + trading activity
                • Risk management in both domains
                • Duration: ~18 minutes
                
                **Hero ↔ DeFi Challenge (200 BONK + 100 SKR)**
                • Hero level achievement + DeFi interaction
                • Advanced skill combination
                • Duration: ~25 minutes
                
                **Community Swap Challenge (300 BONK + 150 SKR)**
                • Community engagement + token swapping
                • Build emergency response network
                • Duration: ~30 minutes
                
                **Total Possible Rewards:** 870 BONK + 435 SKR
                
                **Purpose:** Expand emergency response network across Solana ecosystem
                **Goal:** More trained responders = more lives saved
                
                **Why Cross-App Integration:**
                By connecting emergency preparedness with other Solana activities, we create a comprehensive ecosystem where safety becomes a natural part of daily digital life.
            """.trimIndent())
            .setPositiveButton("Open Challenges") { dialog, _ ->
                dialog.dismiss()
                val intent = Intent(this, CrossAppChallengesActivity::class.java)
                startActivity(intent)
            }
            .setNegativeButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }

    private fun showPitchDeckDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🎯 HACKATHON PITCH DECK")
            .setMessage("""
                **SOLANA SOS - BE A HERO**
                Voice-Activated Emergency Response
                Paragoner Founder | Developer
                
                **💔 MOTIVATION**
                A few years ago, my 4-year-old son almost drowned. I performed CPR without knowing what I was doing. That's why I built Solana SOS.
                
                **🚨 THE PROBLEM**
                3.8 million people die from preventable emergencies every year
                • Traditional apps fail in crisis
                • Manual input required / no voice activated steps
                • Internet dependent
                • 7-14 minute EMS delay
                • 10% survival drop per minute
                
                **💡 THE SOLUTION**
                Solana SOS responds to your voice in under 100 milliseconds, even without internet
                • Voice-activated - "Hey SOS, drowning help!"
                • Enterprise-grade noise filtering
                • Enterprise-grade code architecture and reliability
                • Expandable offline emergency database
                
                **🎬 DEMO**
                Watch how "drowning help!" triggers instant CPR guidance
                Context-aware: Skips rescue steps if "out of water"
                
                **Other Features:**
                • Silent SOS: discreet location sharing and 911 calling
                • Crash detection: auto 911 with GPS
                • Trusted Network: Emergency contacts that can beat EMS
                • SOS Hero: BONK/SKR rewards
                
                **📈 MARKET**
                This is for everyone
                • Safety apps: 1.5B → $5.2B by 2033
                • 76% parents prioritize safety when buying phones
                • 7.3B+ smartphone users in 2025
                
                **💰 BUSINESS MODEL**
                Default app on every mobile Seeker device
                • 40% Seeker sales uplift
                • $50M revenue by Q4 2026
                • Family subscriptions
                • Enterprise
                • Government
                
                **🏆 COMPETITIVE ADVANTAGE**
                The only mobile device that will save your life by default
                • Hybrid online-offline
                • Blockchain secured
                • Context-aware guidance
                • Gamification for viral growth
                
                **📊 TRACTION**
                Working prototype ready for Solana Mobile deployment
                • Core tech: voice & response
                • 12 emergency types: offline coverage
                • Features: silent SOS, crash, gamification
                • Solana: verification & rewards
                • Multi-device sync: bluetooth
                
                **🗓️ ROADMAP**
                FROM HACKATHON PROTOTYPE TO SAFETY REVOLUTION
                • AUG 2025: HACKATHON SUBMISSION
                • Q1 2026: APP LAUNCH ON SEEKER
                • Q2 2026: EMERGENCY PARTNERSHIPS
                • Q3 2026: GLOBAL EXPANSION & REWARDS
                • Q4 2026: $50M+ REVENUE & CONTRACTS
                
                **🎯 CALL TO ACTION**
                ARE YOU READY TO SAVE LIVES WITH SOLANA SOS?
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }

    private fun showHackathonAlignmentDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🏆 HACKATHON ALIGNMENT")
            .setMessage("""
                **Solana Ecosystem Integration**
                
                • SKR Token Rewards - Users earn Seeker ecosystem tokens for emergency preparedness and community safety actions
                • Mobile Wallet Adapter - Seamless integration with Solana Mobile wallet for secure token transactions
                • Seeker Device Optimization - Built specifically for Solana Mobile Seeker with hardware acceleration
                • dApp Store Ready - Designed for immediate deployment on Solana Mobile dApp store
                
                **Solana-Specific Benefits**
                
                • Default App Potential - Positioned to become the default safety app on every Seeker device
                • Blockchain Verification - Emergency records stored on Solana blockchain for tamper-proof documentation
                • Community Safety Network - Leverages Solana fast, low-cost transactions for real-time safety coordination
                • Token Economics - BONK and SKR rewards create sustainable engagement and community building
                
                This app is specifically designed for the Solana ecosystem, leveraging blockchain technology to create a more secure and community-driven emergency response system.
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }
} 