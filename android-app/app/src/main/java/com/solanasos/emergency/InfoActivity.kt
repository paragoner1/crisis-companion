package com.solanasos.emergency

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.solanasos.emergency.databinding.ActivityInfoBinding
import androidx.core.content.ContextCompat
import android.content.Intent
import androidx.appcompat.app.AlertDialog

class InfoActivity : AppCompatActivity() {
    
    private lateinit var binding: ActivityInfoBinding
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        android.widget.Toast.makeText(this, "InfoActivity onCreate!", android.widget.Toast.LENGTH_SHORT).show()
        
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

        // Training section click handler
        binding.trainingSection.setOnClickListener {
            showTrainingDetails()
        }

        // Challenges section click handler
        binding.challengesSection.setOnClickListener {
            showChallengesDetails()
        }

        // Pitch Deck section click handler
        binding.pitchDeckSection.setOnClickListener {
            showPitchDeckDetails()
        }

        // Hackathon Alignment section click handler
        binding.hackathonAlignmentSection.setOnClickListener {
            showHackathonAlignmentDetails()
        }
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