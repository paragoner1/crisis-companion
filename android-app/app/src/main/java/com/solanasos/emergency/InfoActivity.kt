package com.solanasos.emergency

import android.content.Intent
import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.ContextCompat
import com.solanasos.emergency.databinding.ActivityInfoBinding

class InfoActivity : AppCompatActivity() {
    
    private lateinit var binding: ActivityInfoBinding
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityInfoBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        setupClickListeners()
    }
    
    private fun setupClickListeners() {
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
        
        // App Permissions section click handler
        binding.permissionsSection.setOnClickListener {
            showVisualFeedback(binding.permissionsSection)
            showPermissionsDetails()
        }
        
        // SOS Hero Gamification section click handler
        binding.gamificationSection.setOnClickListener {
            showVisualFeedback(binding.gamificationSection)
            showGamificationDetails()
        }
        
        // Cross-App Challenges section click handler
        binding.crossAppChallengesSection.setOnClickListener {
            showVisualFeedback(binding.crossAppChallengesSection)
            val intent = Intent(this, CrossAppChallengesActivity::class.java)
            startActivity(intent)
        }
        
        // Technical Deep Dive section click handler
        binding.technicalDeepDiveSection.setOnClickListener {
            showVisualFeedback(binding.technicalDeepDiveSection)
            showTechnicalDetails()
        }
        
        // Medical Standards section click handler
        binding.medicalStandardsSection.setOnClickListener {
            showVisualFeedback(binding.medicalStandardsSection)
            showMedicalStandardsDetails()
        }
        
        // Development Status section click handler
        binding.developmentStatusSection.setOnClickListener {
            showVisualFeedback(binding.developmentStatusSection)
            showDevelopmentDetails()
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

                DISCLAIMER:
                This app provides emergency guidance but is not a substitute for professional medical care. Always call 911 for life-threatening emergencies. The app's guidance is based on established medical protocols but should be used in conjunction with professional emergency services.

                YOUR RESPONSIBILITY:
                Use this app responsibly and in accordance with local emergency protocols. The app is designed to help, but your judgment and the guidance of emergency professionals should always take precedence.
            """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        scrollView.addView(messageView)

        AlertDialog.Builder(this)
            .setTitle("⚠️ IMPORTANT DISCLAIMER")
            .setView(scrollView)
            .setPositiveButton("I Understand") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }
    
    private fun showEmergencyTypesDetails() {
        val messageView = TextView(this).apply {
            text = """
                COMPLETE EMERGENCY GUIDE:

                15 EMERGENCY PROTOCOLS (OFFLINE):

                1. DROWNING:
                • Remove from water immediately
                • Check breathing and consciousness
                • Begin CPR if not breathing
                • Call 911 immediately

                2. HEART ATTACK:
                • Recognize symptoms (chest pain, shortness of breath)
                • Call 911 immediately
                • Help person sit or lie down
                • Monitor breathing and consciousness

                3. STROKE:
                • Use FAST test (Face, Arm, Speech, Time)
                • Call 911 immediately
                • Note time of onset
                • Keep person calm and comfortable

                4. CHOKING:
                • Perform Heimlich maneuver
                • Back blows for infants
                • Call 911 if severe
                • Continue until object is dislodged

                5. BLEEDING:
                • Apply direct pressure
                • Elevate if possible
                • Use clean cloth or bandage
                • Call 911 for severe bleeding

                6. UNCONSCIOUS:
                • Check breathing and pulse
                • Call 911 immediately
                • Begin CPR if needed
                • Place in recovery position if breathing

                7. SEIZURE:
                • Clear area of hazards
                • Don't restrain person
                • Time the seizure
                • Call 911 if longer than 5 minutes

                8. POISONING:
                • Call Poison Control (1-800-222-1222)
                • Don't induce vomiting
                • Save container if possible
                • Call 911 if severe symptoms

                9. SEVERE BURNS:
                • Cool with room temperature water
                • Don't use ice
                • Cover with clean cloth
                • Call 911 for severe burns

                10. DIABETIC EMERGENCY:
                • Check blood sugar if possible
                • Give sugar if low blood sugar
                • Call 911 if unconscious
                • Monitor breathing

                11. ALLERGIC REACTION:
                • Use EpiPen if prescribed
                • Call 911 immediately
                • Monitor breathing
                • Watch for severe symptoms

                12. TRAUMA:
                • Don't move if spinal injury suspected
                • Control bleeding
                • Call 911 immediately
                • Keep person warm

                13. SUICIDE PREVENTION:
                • Stay with the person
                • Remove means of harm
                • Call 911 or crisis hotline
                • Listen without judgment

                14. OVERDOSE REVERSAL:
                • Call 911 immediately
                • Administer naloxone if available
                • Monitor breathing
                • Stay with person

                15. HYPOTHERMIA:
                • Move to warm area
                • Remove wet clothing
                • Warm gradually
                • Call 911 if severe

                11 DIRECT ACTION PHRASES:
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

                Remember: Always call 911 for life-threatening emergencies. This guidance is supplementary to professional emergency services.
            """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("🚨 Complete Emergency Guide")
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
                COMPREHENSIVE SAFETY FEATURES:

                SILENT SOS (DANGEROUS SITUATIONS):
                • Hold power button for 3 seconds
                • No visible indication on screen
                • Trusted contacts alerted immediately
                • 911 called automatically
                • Perfect for rideshares, domestic violence, kidnapping
                • Works even if phone is locked

                CRASH DETECTION:
                • Automatically detects severe crashes
                • Uses accelerometer, GPS, and gyroscope
                • Calls 911 with GPS coordinates
                • Alerts trusted contacts immediately
                • 30-second window to cancel false alarms
                • Works offline

                TRUSTED NETWORK:
                • Personal emergency contacts notified
                • Real-time location tracking
                • Contacts arrive 5-10 minutes before EMS
                • Blockchain-secured emergency records
                • Multiple contact levels (primary, secondary)
                • Custom alert messages

                LOCATION SERVICES:
                • GPS coordinates during emergencies
                • Real-time location sharing with contacts
                • Emergency service location transmission
                • Offline location caching
                • Privacy-focused location handling

                EMERGENCY CONTACTS:
                • Multiple trusted contacts
                • Custom alert messages
                • Contact priority levels
                • Automatic notification system
                • Emergency response coordination

                PRIVACY PROTECTION:
                • Location only shared during emergencies
                • Voice data processed locally
                • No personal data stored in cloud
                • End-to-end encryption
                • User control over data

                The safety features work together to provide comprehensive protection in any emergency situation.
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
    
    private fun showPermissionsDetails() {
        val messageView = TextView(this).apply {
            text = """
                APP PERMISSIONS & SETUP GUIDE:

                REQUIRED PERMISSIONS:

                MICROPHONE:
                • Voice recognition for emergency activation
                • Offline processing via Vosk
                • No voice data stored in cloud
                • Privacy-focused voice handling

                LOCATION:
                • GPS coordinates during emergencies
                • Emergency service location transmission
                • Real-time location sharing with contacts
                • Only active during emergency situations

                CONTACTS:
                • Trusted network setup
                • Emergency contact management
                • Contact notification system
                • No contact data shared with third parties

                PHONE:
                • Automatic 911 calling
                • Emergency service communication
                • Trusted contact calling
                • Emergency coordination

                STORAGE:
                • Offline emergency database
                • Local emergency protocols
                • App configuration data
                • Emergency response logs

                SETUP INSTRUCTIONS:

                1. DOWNLOAD & INSTALL:
                • Download from Solana Mobile dApp store
                • Install and launch app
                • Grant essential permissions when prompted

                2. CREATE HERO PROFILE:
                • Enter basic information
                • Set emergency preferences
                • Configure safety features
                • Complete initial training

                3. ADD TRUSTED CONTACTS:
                • Select primary emergency contacts
                • Add secondary contacts
                • Set custom alert messages
                • Test notification system

                4. COMPLETE TRAINING:
                • Take initial safety assessment
                • Complete basic training modules
                • Practice emergency scenarios
                • Build confidence and skills

                5. DAILY USE:
                • App runs in background
                • Minimal battery usage
                • Always listening for emergencies
                • Privacy-focused operation

                The app is designed to be always ready while respecting your privacy and battery life.
            """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("📱 App Permissions & Setup")
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
                SOS HERO GAMIFICATION SYSTEM:

                HERO LEVELS:

                🆕 NOVICE HERO (0 XP):
                • Start your emergency response journey
                • Complete basic training modules
                • Learn fundamental emergency skills
                • Earn 100 BONK + 25 SKR

                📚 TRAINEE HERO (100 XP):
                • Build emergency response confidence
                • Complete intermediate training
                • Practice emergency scenarios
                • Earn 200 BONK + 50 SKR

                🚨 EMERGENCY RESPONDER (500 XP):
                • Ready for real emergency response
                • Advanced emergency protocols
                • Community safety participation
                • Earn 500 BONK + 125 SKR

                🛡️ SAFETY GUARDIAN (1,000 XP):
                • Expert emergency response skills
                • Community safety leadership
                • Training and mentoring others
                • Earn 1,000 BONK + 250 SKR

                💪 LIFE PROTECTOR (2,500 XP):
                • Master emergency responder
                • Advanced life-saving techniques
                • Community safety coordination
                • Earn 2,500 BONK + 625 SKR

                🏘️ COMMUNITY DEFENDER (5,000 XP):
                • Community safety champion
                • Emergency response coordination
                • Training program leadership
                • Earn 5,000 BONK + 1,250 SKR

                👁️ SAFETY SENTINEL (10,000 XP):
                • Regional safety leader
                • Emergency response expert
                • Community safety network builder
                • Earn 10,000 BONK + 2,500 SKR

                🏆 EMERGENCY CHAMPION (25,000 XP):
                • National emergency response leader
                • Advanced emergency protocols expert
                • Community safety network coordinator
                • Earn 25,000 BONK + 6,250 SKR

                👼 GUARDIAN ANGEL (50,000 XP):
                • Global emergency response leader
                • Emergency protocol innovator
                • Community safety network architect
                • Earn 50,000 BONK + 12,500 SKR

                ⭐ LIFE-SAVING LEGEND (100,000 XP):
                • Ultimate emergency response master
                • Emergency protocol pioneer
                • Community safety network founder
                • Earn 100,000 BONK + 25,000 SKR

                TRAINING MODULES:

                • CPR Training (50 BONK + 25 SKR)
                • First Aid Basics (30 BONK + 15 SKR)
                • Voice Recognition (25 BONK + 10 SKR)
                • App Features (20 BONK + 10 SKR)
                • Emergency Protocols (40 BONK + 20 SKR)

                REWARD SYSTEM:

                BONK TOKENS:
                • Emergency response actions
                • Training completion
                • Community engagement
                • Daily challenges
                • Achievement bonuses

                SKR TOKENS:
                • Seeker ecosystem integration
                • Advanced skill development
                • Community leadership
                • Emergency preparedness
                • Long-term engagement

                The gamification system motivates continuous learning and emergency preparedness while building a community of life-saving heroes.
            """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("🎮 SOS Hero Gamification")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }
    
    private fun showTechnicalDetails() {
        val messageView = TextView(this).apply {
            text = """
                TECHNICAL ARCHITECTURE:

                OFFLINE DATABASE (15 TYPES):

                CURRENT PROTOCOLS:
                1. Drowning
                2. Heart Attack
                3. Choking
                4. Bleeding
                5. Stroke
                6. Seizure
                7. Poisoning
                8. Severe Burns
                9. Diabetic Emergency
                10. Allergic Reaction
                11. Trauma
                12. Unconscious
                13. Suicide Prevention
                14. Overdose Reversal
                15. Hypothermia

                DATABASE EXPANSION:
                • Additional protocols added regularly
                • App updates include new emergency types
                • Offline users get new protocols
                • No internet required for updates

                VOICE RECOGNITION:

                VOSK OFFLINE PROCESSING:
                • Local voice recognition
                • No internet required
                • Privacy-focused processing
                • Real-time emergency activation

                CONTEXT-AWARE GUIDANCE:
                • Intelligent emergency assessment
                • Step-by-step instructions
                • Direct action phrase recognition
                • Emergency type identification

                DIRECT ACTIONS:
                • Skip basic assessment steps
                • Immediate specific guidance
                • Emergency protocol access
                • Time-saving emergency response

                TECHNOLOGY STACK:

                RUST BACKEND:
                • High-performance emergency processing
                • Reliable voice recognition
                • Efficient memory usage
                • Cross-platform compatibility

                ANDROID JNI:
                • Native performance integration
                • Hardware acceleration
                • Battery optimization
                • Seamless user experience

                SQLITE DATABASE:
                • Local emergency protocols
                • Offline data storage
                • Fast emergency access
                • Reliable data management

                SOLANA BLOCKCHAIN:
                • Emergency record verification
                • Tamper-proof documentation
                • Community safety coordination
                • Token reward distribution

                The technical architecture ensures reliable, fast, and secure emergency response capabilities.
            """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("🔧 Technical Deep Dive")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }
    
    private fun showMedicalStandardsDetails() {
        val messageView = TextView(this).apply {
            text = """
                MEDICAL STANDARDS & COMPLIANCE:

                EMERGENCY PROTOCOLS BASED ON:

                AMERICAN HEART ASSOCIATION (AHA):
                • CPR guidelines and protocols
                • Emergency cardiovascular care
                • Life support standards
                • Training program standards

                AMERICAN RED CROSS:
                • Emergency response protocols
                • First aid standards
                • Disaster response guidelines
                • Community safety programs

                CENTERS FOR DISEASE CONTROL (CDC):
                • Medical guidelines and protocols
                • Public health standards
                • Emergency response recommendations
                • Medical research integration

                WORLD HEALTH ORGANIZATION (WHO):
                • International medical standards
                • Global emergency protocols
                • Public health guidelines
                • Emergency response coordination

                NATIONAL INSTITUTES OF HEALTH (NIH):
                • Medical research integration
                • Evidence-based protocols
                • Clinical trial results
                • Medical innovation standards

                EMERGENCY MEDICAL SERVICES (EMS):
                • Field protocol standards
                • Prehospital care guidelines
                • Emergency response coordination
                • Professional emergency standards

                NATIONAL SAFETY COUNCIL (NSC):
                • Safety standards and protocols
                • Emergency prevention guidelines
                • Community safety programs
                • Safety education standards

                AMERICAN COLLEGE OF EMERGENCY PHYSICIANS (ACEP):
                • Emergency medicine standards
                • Clinical practice guidelines
                • Emergency care protocols
                • Professional emergency standards

                INTERNATIONAL LIAISON COMMITTEE ON RESUSCITATION (ILCOR):
                • Global resuscitation standards
                • International emergency protocols
                • Evidence-based guidelines
                • Global emergency coordination

                NATIONAL ASSOCIATION OF EMERGENCY MEDICAL TECHNICIANS (NAEMT):
                • Prehospital care standards
                • Emergency medical protocols
                • Field emergency guidelines
                • Professional emergency standards

                PRIVACY & SECURITY COMPLIANCE:

                HIPAA COMPLIANCE:
                • Medical information protection
                • Patient privacy standards
                • Healthcare data security
                • Medical record protection

                GDPR COMPLIANCE:
                • European data protection standards
                • User privacy rights
                • Data processing transparency
                • International privacy standards

                COPPA COMPLIANCE:
                • Children's privacy protection
                • Youth data security
                • Age-appropriate content
                • Family privacy standards

                EMERGENCY SERVICES STANDARDS:
                • 911 integration protocols
                • Emergency service coordination
                • Public safety standards
                • Emergency response compliance

                All emergency protocols are regularly updated to reflect the latest medical standards and best practices from these authoritative sources.
            """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("🏥 Medical Standards & Compliance")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }
    
    private fun showDevelopmentDetails() {
        val messageView = TextView(this).apply {
            text = """
                DEVELOPMENT STATUS:

                WORKING PROTOTYPE:
                • Core emergency response functionality
                • Voice recognition integration
                • Offline emergency protocols
                • Safety features implementation

                SOLANA MOBILE READY:
                • Optimized for Solana Mobile Seeker
                • Hardware acceleration support
                • Native device integration
                • dApp store deployment ready

                CORE TECHNOLOGIES:
                • Voice recognition via Vosk
                • 15 emergency types offline coverage
                • Safety features (silent SOS, crash detection)
                • Gamification system with rewards

                SOLANA INTEGRATION:
                • Blockchain verification for emergency records
                • BONK and SKR token rewards
                • Mobile wallet adapter integration
                • Community safety network

                MULTI-DEVICE SYNC:
                • Bluetooth connectivity
                • Cross-device emergency coordination
                • Community safety network
                • Real-time emergency response

                DEPLOYMENT READY:
                • Solana Mobile dApp store compatible
                • Production-ready emergency protocols
                • Community safety features
                • Token reward system

                The app is ready for immediate deployment on Solana Mobile with full emergency response capabilities.
            """.trimIndent()
            textSize = 12f
            setPadding(32, 32, 32, 32)
            setTextColor(ContextCompat.getColor(this@InfoActivity, android.R.color.black))
        }

        AlertDialog.Builder(this)
            .setTitle("🚀 Development Status")
            .setView(messageView)
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .setCancelable(true)
            .show()
    }
} 