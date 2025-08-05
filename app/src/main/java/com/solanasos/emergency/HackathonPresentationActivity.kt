package com.solanasos.emergency

import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import com.solanasos.emergency.databinding.ActivityHackathonPresentationBinding

class HackathonPresentationActivity : AppCompatActivity() {
    private lateinit var binding: ActivityHackathonPresentationBinding
    private val TAG = "HackathonPresentation"

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityHackathonPresentationBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setupPresentationFlow()
    }

    private fun setupPresentationFlow() {
        binding.btnFullPresentation.setOnClickListener {
            startFullPresentation()
        }

        binding.btnSkipToDemo.setOnClickListener {
            startDemo()
        }

        binding.btnBack.setOnClickListener {
            finish()
        }
    }

    private fun startFullPresentation() {
        showIntroduction()
    }

    private fun showIntroduction() {
        val message = """
            **SOLANA SOS - BE A HERO**
            
            Hello everyone! I'm Paragoner, founder and developer of Solana SOS. Let me show you how we're revolutionizing emergency response with voice-activated, blockchain-powered life-saving technology.
            
            **The Problem:** Every year, 3.8 million people die from preventable emergencies. Traditional apps fail in crisis because they require manual input, depend on internet, and can't respond fast enough when seconds matter.
            
            **Our Solution:** Solana SOS responds to your voice in under 100 milliseconds, even without internet.
        """.trimIndent()

        AlertDialog.Builder(this)
            .setTitle("🎯 Introduction")
            .setMessage(message)
            .setPositiveButton("Next: Problem & Solution") { _, _ ->
                showProblemSolution()
            }
            .setNegativeButton("Skip to Demo") { _, _ ->
                startDemo()
            }
            .setCancelable(false)
            .show()
    }

    private fun showProblemSolution() {
        val message = """
            **THE PROBLEM:**
            • 3.8 million people die from preventable emergencies every year
            • Traditional apps fail in crisis
            • Manual input required / no voice activated steps
            • Internet dependent
            • 7-14 minute EMS delay
            • 10% survival drop per minute
            
            **THE SOLUTION:**
            • Voice-activated: "Hey SOS, drowning help!"
            • Enterprise-grade noise filtering
            • Enterprise-grade code architecture and reliability
            • 12 context-aware emergency types in offline mode
            • Unlimited emergency types with AI guidance in online mode
            • Smart Audio Management: AI guidance + 911 operator integration
        """.trimIndent()

        AlertDialog.Builder(this)
            .setTitle("🚨 Problem & Solution")
            .setMessage(message)
            .setPositiveButton("Next: Demo") { _, _ ->
                startDemo()
            }
            .setNegativeButton("Back") { _, _ ->
                showIntroduction()
            }
            .setCancelable(false)
            .show()
    }

    private fun startDemo() {
        val message = """
            **DEMO: Watch how "drowning help!" triggers instant CPR guidance**
            
            Key Features to Watch:
            • Context-aware: Skips rescue steps if "out of water"
            • App automatically calls 911 and shares location
            • 911 operator stays on standby for when you're ready
            • Smart audio prevents interruption of life-saving actions
            
            **Other Features:**
            • Silent SOS: Discreet location sharing and 911 calling
            • Crash Detection: Auto 911 with GPS
            • Trusted Network: Emergency contacts that can beat EMS
            • SOS Hero: BONK/SKR rewards for training and engagement
            • Training Center: Earn tokens by learning life-saving skills
            • Cross-App Challenges: Viral growth across Solana ecosystem
        """.trimIndent()

        AlertDialog.Builder(this)
            .setTitle("🎬 Demo Overview")
            .setMessage(message)
            .setPositiveButton("Start Demo") { _, _ ->
                showDemoInstructions()
            }
            .setNegativeButton("Back") { _, _ ->
                showProblemSolution()
            }
            .setCancelable(false)
            .show()
    }

    private fun showDemoInstructions() {
        val message = """
            **DEMO INSTRUCTIONS:**
            
            1. Go to the main app screen
            2. Click "Demo Mode" button
            3. Select "Drowning" emergency type
            4. Watch the comprehensive emergency response flow
            5. Observe how the app automatically calls 911 and shares location
            6. See the life-saving instructions provided
            7. Notice the smart audio management system
            
            Ready to proceed with the demo?
        """.trimIndent()

        AlertDialog.Builder(this)
            .setTitle("📋 Demo Instructions")
            .setMessage(message)
            .setPositiveButton("Proceed to App") { _, _ ->
                showMarketBusinessModel()
            }
            .setNegativeButton("Back") { _, _ ->
                startDemo()
            }
            .setCancelable(false)
            .show()
    }

    private fun showMarketBusinessModel() {
        val message = """
            **MARKET:**
            This is for everyone
            • Safety apps: 1.5B → $5.2B by 2033
            • 76% parents prioritize safety when buying phones
            • 7.3B+ smartphone users in 2025
            • Family engagement through gamification
            • Enterprise safety compliance
            • Government emergency response integration
            
            **BUSINESS MODEL:**
            Default app on every Solana Mobile Seeker device
            • 40% Seeker sales uplift
            • $50M revenue by Q4 2026
            • Family subscriptions with training rewards
            • Enterprise safety contracts
            • Government emergency partnerships
            • Cross-app ecosystem revenue sharing
        """.trimIndent()

        AlertDialog.Builder(this)
            .setTitle("💰 Market & Business Model")
            .setMessage(message)
            .setPositiveButton("Next: Competitive Advantage") { _, _ ->
                showCompetitiveAdvantage()
            }
            .setNegativeButton("Back") { _, _ ->
                showDemoInstructions()
            }
            .setCancelable(false)
            .show()
    }

    private fun showCompetitiveAdvantage() {
        val message = """
            **COMPETITIVE ADVANTAGE:**
            The only mobile device that will save your life by default
            
            • Hybrid online-offline with seamless transition
            • Blockchain-secured emergency logs
            • Context-aware guidance with AI enhancement
            • Gamification for viral growth and user retention
            • Training system for skill development
            • Cross-app challenges for ecosystem growth
            
            **TRACTION:**
            Working prototype ready for Solana Mobile deployment
            • Core tech: Voice recognition & emergency response
            • 12 emergency types: Complete offline coverage
            • Advanced features: Silent SOS, crash detection, gamification
            • Solana integration: Verification & token rewards
            • Multi-device sync: Bluetooth for offline communication
            • Training system: Skill development with rewards
            • Cross-app challenges: Ecosystem viral growth
        """.trimIndent()

        AlertDialog.Builder(this)
            .setTitle("🏆 Competitive Advantage & Traction")
            .setMessage(message)
            .setPositiveButton("Next: Roadmap") { _, _ ->
                showRoadmap()
            }
            .setNegativeButton("Back") { _, _ ->
                showMarketBusinessModel()
            }
            .setCancelable(false)
            .show()
    }

    private fun showRoadmap() {
        val message = """
            **ROADMAP: FROM HACKATHON PROTOTYPE TO SAFETY REVOLUTION**
            
            • AUG 2025: HACKATHON SUBMISSION ✅
            • Q1 2026: APP LAUNCH ON SEEKER
            • Q2 2026: EMERGENCY PARTNERSHIPS
            • Q3 2026: GLOBAL EXPANSION & REWARDS
            • Q4 2026: $50M+ REVENUE & CONTRACTS
            
            **CALL TO ACTION:**
            "ARE YOU READY TO SAVE LIVES WITH SOLANA SOS?"
            
            **THANK YOU!**
            Questions?
        """.trimIndent()

        AlertDialog.Builder(this)
            .setTitle("🗺️ Roadmap & Call to Action")
            .setMessage(message)
            .setPositiveButton("Complete") { _, _ ->
                finish()
            }
            .setNegativeButton("Back") { _, _ ->
                showCompetitiveAdvantage()
            }
            .setCancelable(false)
            .show()
    }
} 