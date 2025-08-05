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
        
        // Set status bar color to match the deep navy background
        window.statusBarColor = ContextCompat.getColor(this, R.color.deep_navy)
        
        binding = ActivityInfoBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        setupUI()
        
        // Test if binding works
        try {
            binding.btnBack.setOnClickListener {
                android.widget.Toast.makeText(this, "Back button test!", android.widget.Toast.LENGTH_SHORT).show()
            }
        } catch (e: Exception) {
            android.util.Log.e("InfoActivity", "Binding test failed: ${e.message}")
        }
    }
    
    private fun setupUI() {
        // Test if binding works
        try {
            android.widget.Toast.makeText(this, "InfoActivity loaded!", android.widget.Toast.LENGTH_SHORT).show()
            android.util.Log.d("InfoActivity", "InfoActivity loaded successfully")
        } catch (e: Exception) {
            android.util.Log.e("InfoActivity", "Error in setupUI: ${e.message}")
        }

        // Back button
        binding.btnBack.setOnClickListener {
            android.util.Log.d("InfoActivity", "Back button clicked!")
            android.widget.Toast.makeText(this, "Back button clicked!", android.widget.Toast.LENGTH_SHORT).show()
            finish()
        }

        // Mission Statement section click handler
        binding.missionStatementSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Mission Statement clicked")
            android.widget.Toast.makeText(this, "Mission Statement clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // What is Solana SOS section click handler
        binding.whatIsSolanaSosSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "What is Solana SOS clicked")
            android.widget.Toast.makeText(this, "What is Solana SOS clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // How It Works section click handler
        binding.howItWorksSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "How It Works clicked")
            android.widget.Toast.makeText(this, "How It Works clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Authoritative Sources section click handler
        binding.authoritativeSourcesSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Authoritative Sources clicked")
            android.widget.Toast.makeText(this, "Authoritative Sources clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Privacy & Security section click handler
        binding.privacySecuritySection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Privacy & Security clicked")
            android.widget.Toast.makeText(this, "Privacy & Security clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Developer Info section click handler
        binding.developerInfoSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Developer Info clicked")
            android.widget.Toast.makeText(this, "Developer Info clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // What is Solana SOS section click handler
        binding.whatIsSolanaSosSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "What is Solana SOS clicked")
            showWhatIsSolanaSosDetails()
        }

        // How It Works section click handler
        binding.howItWorksSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "How It Works clicked")
            showHowItWorksDetails()
        }

        // Authoritative Sources section click handler
        binding.authoritativeSourcesSection.setOnClickListener {
            showAuthoritativeSourcesDetails()
        }

        // Privacy & Security section click handler
        binding.privacySecuritySection.setOnClickListener {
            showPrivacySecurityDetails()
        }

        // Training section click handler
        binding.trainingSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Training clicked!", android.widget.Toast.LENGTH_SHORT).show()
            showTrainingDetails()
        }

        // Challenges section click handler
        binding.challengesSection.setOnClickListener {
            showChallengesDetails()
        }

        // Pitch Deck Sections
        binding.motivationSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Motivation clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("💔 MOTIVATION")
                .setMessage("A few years ago, my 4-year-old son almost drowned. I performed CPR without knowing what I was doing. That's why I built Solana SOS.\n\nThis personal experience drives every feature and decision in the app.")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.problemSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Problem clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("🚨 THE PROBLEM")
                .setMessage("3.8 million people die from preventable emergencies every year\n\n• Traditional apps fail in crisis\n• Manual input required / no voice activated steps\n• Internet dependent\n• 7-14 minute EMS delay\n• 10% survival drop per minute")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.solutionSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Solution clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("💡 THE SOLUTION")
                .setMessage("Solana SOS responds to your voice in under 100 milliseconds, even without internet\n\n• Voice-activated - 'Hey SOS, drowning help!'\n• Enterprise-grade noise filtering\n• Enterprise-grade code architecture and reliability\n• Expandable offline emergency database")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.demoSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Demo clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("🎬 DEMO")
                .setMessage("Watch how 'drowning help!' triggers instant CPR guidance\n\nContext-aware: Skips rescue steps if 'out of water'\n\nOther Features:\n• Silent SOS: discreet location sharing and 911 calling\n• Crash detection: auto 911 with GPS\n• Trusted Network: Emergency contacts that can beat EMS\n• SOS Hero: BONK/SKR rewards")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.marketSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Market clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("📈 MARKET")
                .setMessage("This is for everyone\n\n• Safety apps: 1.5B → $5.2B by 2033\n• 76% parents prioritize safety when buying phones\n• 7.3B+ smartphone users in 2025")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.businessModelSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Business Model clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("💰 BUSINESS MODEL")
                .setMessage("Default app on every mobile Seeker device\n\n• 40% Seeker sales uplift\n• $50M revenue by Q4 2026\n• Family subscriptions\n• Enterprise\n• Government")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.competitiveAdvantageSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Competitive Advantage clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("🏆 COMPETITIVE ADVANTAGE")
                .setMessage("The only mobile device that will save your life by default\n\n• Hybrid online-offline\n• Blockchain secured\n• Context-aware guidance\n• Gamification for viral growth")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.tractionSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Traction clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("📊 TRACTION")
                .setMessage("Working prototype ready for Solana Mobile deployment\n\n• Core tech: voice & response\n• 12 emergency types: offline coverage\n• Features: silent SOS, crash, gamification\n• Solana: verification & rewards\n• Multi-device sync: bluetooth")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.roadmapSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Roadmap clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("🗓️ ROADMAP")
                .setMessage("FROM HACKATHON PROTOTYPE TO SAFETY REVOLUTION\n\n• AUG 2025: HACKATHON SUBMISSION\n• Q1 2026: APP LAUNCH ON SEEKER\n• Q2 2026: EMERGENCY PARTNERSHIPS\n• Q3 2026: GLOBAL EXPANSION & REWARDS\n• Q4 2026: $50M+ REVENUE & CONTRACTS")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.callToActionSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Call to Action clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("🎯 CALL TO ACTION")
                .setMessage("ARE YOU READY TO SAVE LIVES WITH SOLANA SOS?\n\nThis is more than an app - it's a revolution in emergency response that will make every Solana Mobile device a life-saving tool.")
                .setPositiveButton("YES!") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Mission Statement section click handler
        binding.missionStatementSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Mission Statement clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("Mission Statement")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // What is Solana SOS section click handler
        binding.whatIsSolanaSosSection.setOnClickListener {
            android.widget.Toast.makeText(this, "What is Solana SOS clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("What is Solana SOS")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // How It Works section click handler
        binding.howItWorksSection.setOnClickListener {
            android.widget.Toast.makeText(this, "How It Works clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("How It Works")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Pitch Deck Sections
        binding.motivationSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Motivation clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("💔 MOTIVATION")
                .setMessage("A few years ago, my 4-year-old son almost drowned. I performed CPR without knowing what I was doing. That's why I built Solana SOS.\n\nThis personal experience drives every feature and decision in the app.")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.problemSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Problem clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("🚨 THE PROBLEM")
                .setMessage("3.8 million people die from preventable emergencies every year\n\n• Traditional apps fail in crisis\n• Manual input required / no voice activated steps\n• Internet dependent\n• 7-14 minute EMS delay\n• 10% survival drop per minute")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.solutionSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Solution clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("💡 THE SOLUTION")
                .setMessage("Solana SOS responds to your voice in under 100 milliseconds, even without internet\n\n• Voice-activated - 'Hey SOS, drowning help!'\n• Enterprise-grade noise filtering\n• Enterprise-grade code architecture and reliability\n• Expandable offline emergency database")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.demoSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Demo clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("🎬 DEMO")
                .setMessage("Watch how 'drowning help!' triggers instant CPR guidance\n\nContext-aware: Skips rescue steps if 'out of water'\n\nOther Features:\n• Silent SOS: discreet location sharing and 911 calling\n• Crash detection: auto 911 with GPS\n• Trusted Network: Emergency contacts that can beat EMS\n• SOS Hero: BONK/SKR rewards")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.marketSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Market clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("📈 MARKET")
                .setMessage("This is for everyone\n\n• Safety apps: 1.5B → $5.2B by 2033\n• 76% parents prioritize safety when buying phones\n• 7.3B+ smartphone users in 2025")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.businessModelSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Business Model clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("💰 BUSINESS MODEL")
                .setMessage("Default app on every mobile Seeker device\n\n• 40% Seeker sales uplift\n• $50M revenue by Q4 2026\n• Family subscriptions\n• Enterprise\n• Government")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.competitiveAdvantageSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Competitive Advantage clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("🏆 COMPETITIVE ADVANTAGE")
                .setMessage("The only mobile device that will save your life by default\n\n• Hybrid online-offline\n• Blockchain secured\n• Context-aware guidance\n• Gamification for viral growth")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.tractionSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Traction clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("📊 TRACTION")
                .setMessage("Working prototype ready for Solana Mobile deployment\n\n• Core tech: voice & response\n• 12 emergency types: offline coverage\n• Features: silent SOS, crash, gamification\n• Solana: verification & rewards\n• Multi-device sync: bluetooth")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.roadmapSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Roadmap clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("🗓️ ROADMAP")
                .setMessage("FROM HACKATHON PROTOTYPE TO SAFETY REVOLUTION\n\n• AUG 2025: HACKATHON SUBMISSION\n• Q1 2026: APP LAUNCH ON SEEKER\n• Q2 2026: EMERGENCY PARTNERSHIPS\n• Q3 2026: GLOBAL EXPANSION & REWARDS\n• Q4 2026: $50M+ REVENUE & CONTRACTS")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        binding.callToActionSection.setOnClickListener {
            android.widget.Toast.makeText(this, "Call to Action clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("🎯 CALL TO ACTION")
                .setMessage("ARE YOU READY TO SAVE LIVES WITH SOLANA SOS?\n\nThis is more than an app - it's a revolution in emergency response that will make every Solana Mobile device a life-saving tool.")
                .setPositiveButton("YES!") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Mission Statement section click handler
        binding.missionStatementSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Mission Statement clicked!")
            android.widget.Toast.makeText(this, "Mission Statement clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("Mission Statement")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // What is Solana SOS section click handler
        binding.whatIsSolanaSosSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "What is Solana SOS clicked!")
            android.widget.Toast.makeText(this, "What is Solana SOS clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("What is Solana SOS")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // How It Works section click handler
        binding.howItWorksSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "How It Works clicked!")
            android.widget.Toast.makeText(this, "How It Works clicked!", android.widget.Toast.LENGTH_SHORT).show()
            AlertDialog.Builder(this)
                .setTitle("How It Works")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Mission Statement section click handler
        binding.missionStatementSection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("Mission Statement")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // What is Solana SOS section click handler
        binding.whatIsSolanaSosSection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("What is Solana SOS")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // How It Works section click handler
        binding.howItWorksSection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("How It Works")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Authoritative Sources section click handler
        binding.authoritativeSourcesSection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("Authoritative Sources")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Privacy & Security section click handler
        binding.privacySecuritySection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("Privacy & Security")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Developer Info section click handler
        binding.developerInfoSection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("Developer Info")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Mission Statement section click handler
        binding.missionStatementSection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("Mission Statement")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // What is Solana SOS section click handler
        binding.whatIsSolanaSosSection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("What is Solana SOS")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // How It Works section click handler
        binding.howItWorksSection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("How It Works")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Authoritative Sources section click handler
        binding.authoritativeSourcesSection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("Authoritative Sources")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Privacy & Security section click handler
        binding.privacySecuritySection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("Privacy & Security")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Developer Info section click handler
        binding.developerInfoSection.setOnClickListener {
            AlertDialog.Builder(this)
                .setTitle("Developer Info")
                .setMessage("This is a test dialog to see if clicks work!")
                .setPositiveButton("OK") { dialog, _ -> dialog.dismiss() }
                .show()
        }

        // Mission Statement section click handler
        binding.missionStatementSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Mission Statement clicked")
            android.widget.Toast.makeText(this, "Mission Statement clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // What is Solana SOS section click handler
        binding.whatIsSolanaSosSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "What is Solana SOS clicked")
            android.widget.Toast.makeText(this, "What is Solana SOS clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // How It Works section click handler
        binding.howItWorksSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "How It Works clicked")
            android.widget.Toast.makeText(this, "How It Works clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Authoritative Sources section click handler
        binding.authoritativeSourcesSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Authoritative Sources clicked")
            android.widget.Toast.makeText(this, "Authoritative Sources clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Privacy & Security section click handler
        binding.privacySecuritySection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Privacy & Security clicked")
            android.widget.Toast.makeText(this, "Privacy & Security clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Developer Info section click handler
        binding.developerInfoSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Developer Info clicked")
            android.widget.Toast.makeText(this, "Developer Info clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Mission Statement section click handler
        binding.missionStatementSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Mission Statement clicked")
            android.widget.Toast.makeText(this, "Mission Statement clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // What is Solana SOS section click handler
        binding.whatIsSolanaSosSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "What is Solana SOS clicked")
            android.widget.Toast.makeText(this, "What is Solana SOS clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // How It Works section click handler
        binding.howItWorksSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "How It Works clicked")
            android.widget.Toast.makeText(this, "How It Works clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Authoritative Sources section click handler
        binding.authoritativeSourcesSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Authoritative Sources clicked")
            android.widget.Toast.makeText(this, "Authoritative Sources clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Privacy & Security section click handler
        binding.privacySecuritySection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Privacy & Security clicked")
            android.widget.Toast.makeText(this, "Privacy & Security clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Developer Info section click handler
        binding.developerInfoSection.setOnClickListener {
            android.util.Log.d("InfoActivity", "Developer Info clicked")
            android.widget.Toast.makeText(this, "Developer Info clicked!", android.widget.Toast.LENGTH_SHORT).show()
        }

        // Developer Info section click handler
        binding.developerInfoSection.setOnClickListener {
            showDeveloperInfoDetails()
        }
    }

    private fun showWhatIsSolanaSosDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🚨 What is Solana SOS?")
            .setMessage("""
                **VOICE-ACTIVATED EMERGENCY RESPONSE**
                
                Imagine you're at the beach and someone starts drowning. You panic - what do you do? Most people freeze or forget basic lifesaving steps. **Solana SOS** is like having a trained emergency responder in your pocket who guides you through any emergency, step by step.
                
                **🏆 How It Works (In Simple Terms)**
                
                **1. You Say "Hey SOS"**
                • Just like saying "Hey Siri" or "Hey Google"
                • The app wakes up and listens for what emergency you're dealing with
                • Works even in loud places like beaches, pools, or crowded areas
                
                **2. The App Understands Your Situation**
                • You say: "Hey SOS, someone's drowning and I pulled them out of the water"
                • The app thinks: "They're past the rescue phase, need medical care now"
                • Instead of telling you to call for help (you already did that), it jumps straight to checking if the person is breathing
                
                **3. It Guides You Step by Step**
                • Clear, simple instructions: "Check if they're breathing"
                • If they're not breathing: "Start rescue breathing now"
                • If they have a pulse: "Keep them warm and monitor their breathing"
                • No confusing medical jargon - just what you need to do right now
                
                **🚨 Real-World Examples**
                
                **Example 1: Drowning at the Beach**
                You: "Hey SOS, drowning emergency"
                App: "Emergency detected. Is the person still in the water?"
                You: "No, I pulled them out"
                App: "Check if they're breathing and have a pulse"
                App: "If not breathing, start rescue breathing immediately"
                App: "I'm calling 911 and sharing your location"
                
                **What the app saved you from:**
                • Wasting time on "stay calm" instructions when you're already acting
                • Confusing medical terms
                • Not knowing what to do next
                
                **🎮 The Fun Part: Becoming a Hero**
                
                **Level Up Like a Video Game**
                • Start as a **Novice Hero** (just like you!)
                • Complete training to become a **Trainee Hero**
                • Respond to emergencies to become an **Emergency Responder**
                • Keep going until you're a **Life-Saving Legend**
                
                **Earn Rewards**
                • **Experience Points** for learning and helping
                • **BONK Tokens** for emergency responses
                • **SKR Tokens** for building your safety network
                • **Badges** for achievements like "CPR Master" or "Quick Responder"
                
                **🚨 Emergency Types It Handles**
                
                **Medical Emergencies:** Heart Attack, Stroke, Choking, Allergic Reaction, Diabetic Emergency
                **Injury Emergencies:** Bleeding, Severe Burns, Unconscious, Trauma
                **Environmental Emergencies:** Drowning, Poisoning, Seizure
                
                **🎯 Why It's Different**
                
                **Traditional Apps vs. Solana SOS**
                
                **Traditional Emergency Apps:**
                • Require you to open the app manually
                • Need internet connection
                • Give generic instructions
                • Don't understand your specific situation
                
                **Solana SOS:**
                • Voice-activated (hands-free)
                • Works offline
                • Understands context (saves 45 seconds)
                • Calls 911 automatically
                • Alerts your personal network
                
                **Real Impact:**
                • **45 seconds saved** per emergency
                • **5-10 minute advantage** over emergency services
                • **Community safety** - friends arrive before EMS
                • **Confidence building** - you know what to do
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showHowItWorksDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("⚡ How It Works")
            .setMessage("""
                **STEP-BY-STEP EMERGENCY RESPONSE PROCESS**
                
                **1. Voice Activation**
                Say "Hey SOS" followed by the emergency type (e.g., "drowning help", "heart attack", "choking")
                • Uses advanced voice recognition technology
                • Works in noisy environments
                • Processes locally on your device for privacy
                
                **2. Instant Response**
                App recognizes the emergency and provides immediate guidance
                • Response time: Under 100 milliseconds
                • Context-aware instructions
                • Clear, step-by-step guidance
                
                **3. Automatic Actions**
                App calls 911 and shares location automatically
                • No manual dialing required
                • GPS coordinates sent to emergency services
                • Location shared with trusted contacts
                
                **4. Life-Saving Instructions**
                Step-by-step guidance for the specific emergency
                • CPR techniques for heart attacks
                • Heimlich maneuver for choking
                • First aid for bleeding, burns, etc.
                • Instructions spoken at maximum volume
                
                **5. Smart Integration**
                911 operator stays on standby for when you're ready
                • Operator listens for context without interrupting
                • You choose when to speak with operator
                • No interruption of critical life-saving actions
                
                **6. Blockchain Logging**
                Emergency response is securely recorded on Solana
                • Immutable record of emergency response
                • Transparent for emergency services
                • Helps improve emergency protocols
                
                **Offline Mode:**
                When no internet is available, the app still provides life-saving instructions using pre-downloaded emergency protocols and sends SMS messages with your location to trusted contacts.
                
                **Online Mode:**
                With internet connection, the app provides unlimited emergency types with enhanced AI-powered guidance and real-time context analysis.
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showAuthoritativeSourcesDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("📚 Authoritative Sources & Credible Authorities")
            .setMessage("""
                **LIFE-SAVING GUIDANCE FROM TRUSTED AUTHORITIES**
                
                All emergency response instructions in Solana SOS are verified against current medical standards from these authoritative sources:
                
                **🏥 American Heart Association (AHA)**
                • CPR guidelines and techniques
                • AED usage protocols
                • Emergency cardiovascular care standards
                • Latest resuscitation science
                
                **🩸 American Red Cross**
                • Emergency response protocols
                • First aid techniques
                • Disaster preparedness guidelines
                • International emergency standards
                
                **🏛️ Centers for Disease Control (CDC)**
                • Medical guidelines and protocols
                • Public health emergency response
                • Injury prevention strategies
                • Emergency preparedness standards
                
                **🌍 World Health Organization (WHO)**
                • International emergency standards
                • Global health protocols
                • Emergency response guidelines
                • Public health emergency frameworks
                
                **🔬 National Institutes of Health (NIH)**
                • Medical research and protocols
                • Evidence-based emergency care
                • Clinical practice guidelines
                • Emergency medicine standards
                
                **🚑 Emergency Medical Services (EMS)**
                • Professional emergency protocols
                • Pre-hospital care standards
                • Emergency response procedures
                • Medical emergency guidelines
                
                **Protocol Verification:**
                All emergency response instructions are:
                • Verified against current medical standards
                • Regularly updated to reflect latest best practices
                • Reviewed by medical professionals
                • Tested in real-world emergency scenarios
                
                **Continuous Improvement:**
                Emergency protocols are updated as new medical research and best practices become available, ensuring users always have access to the most current and effective emergency response guidance.
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showPrivacySecurityDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🔒 Privacy & Security")
            .setMessage("""
                **YOUR PRIVACY AND SECURITY ARE OUR TOP PRIORITY**
                
                **Voice Data Protection:**
                • Voice data processed locally on your device
                • No voice recordings stored in the cloud
                • Real-time processing for immediate response
                • No voice data transmitted to external servers
                
                **Location Privacy:**
                • Location shared only during emergencies
                • GPS coordinates sent only to emergency services
                • No location tracking or profiling
                • Location data not stored for non-emergency purposes
                
                **Data Storage:**
                • No personal data stored in the cloud
                • Emergency metadata only on blockchain
                • Anonymous emergency reporting
                • No personal information in emergency logs
                
                **Blockchain Security:**
                • Encrypted emergency logs on Solana blockchain
                • Immutable records for transparency
                • Emergency metadata only (no personal info)
                • Secure, decentralized storage
                
                **Communication Security:**
                • End-to-end encryption for all communications
                • Secure SMS and Bluetooth protocols
                • Encrypted emergency service communications
                • No interception of emergency communications
                
                **Security Features:**
                • Secure voice recognition technology
                • Anonymous emergency reporting system
                • No tracking or profiling of users
                • Privacy-first design philosophy
                
                **Emergency Data:**
                • Only emergency response metadata is logged
                • No personal identification information
                • Emergency type, response time, and outcome only
                • Data used solely for improving emergency protocols
                
                **Your Control:**
                • You control when the app activates
                • You choose when to share location
                • You decide when to contact emergency services
                • You maintain control throughout the emergency
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
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
                **COMMUNITY INTEGRATION FOR LIFE-SAVING NETWORK**
                
                **Available Challenges:**
                
                **🫀💎 CPR ↔ DeFi Challenge (100 BONK + 50 SKR)**
                • Duration: 7 days
                • CPR users learn DeFi basics
                • DeFi users learn emergency response
                • Cross-community skill sharing
                • Expected growth: +500% engagement
                
                **🚨🎮 Emergency ↔ Gaming Challenge (150 BONK + 75 SKR)**
                • Duration: 14 days
                • Gamers learn emergency response through games
                • Emergency responders create gaming content
                • Gaming mechanics for emergency training
                • Expected growth: +300% engagement
                
                **🛡️📈 Safety ↔ Trading Challenge (120 BONK + 60 SKR)**
                • Duration: 10 days
                • Traders learn emergency safety protocols
                • Emergency responders learn risk management
                • Safety-first trading strategies
                • Expected growth: +400% engagement
                
                **💎🏆 Hero ↔ DeFi Challenge (200 BONK + 100 SKR)**
                • Duration: 21 days
                • Heroes earn DeFi rewards for emergency actions
                • DeFi protocols fund emergency response initiatives
                • Hero status unlocks DeFi privileges
                • Expected growth: +600% engagement
                
                **🔄🌐 Community Swap Challenge (300 BONK + 150 SKR)**
                • Duration: 30 days
                • Communities swap users and tokens
                • Cross-app feature integration
                • Shared governance across apps
                • Expected growth: +1000% engagement
                
                **Total Possible Rewards:** 870 BONK + 435 SKR
                
                **Why Cross-App Challenges Matter:**
                These challenges expand the network of trained emergency responders across the Solana ecosystem. More trained responders = more lives saved. Every challenge completed means more people prepared to save lives when seconds count.
                
                **Purpose:** Expand emergency response network
                **Goal:** More trained responders = more lives saved
                **Impact:** Building a world where no one dies from preventable emergencies
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

    private fun showMissionStatementDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🎯 Mission Statement & Disclaimer")
            .setMessage("""
                **MISSION STATEMENT**
                To save lives by making emergency response accessible to everyone through voice-activated, blockchain-powered technology that works anywhere, anytime. Anyone can be a Hero.
                
                **⚠️ IMPORTANT DISCLAIMER**
                
                **Solana SOS is a serious life-saving application designed for emergency situations.**
                
                **Medical Disclaimer:**
                • This app provides guidance for emergency situations
                • It is not a substitute for professional medical care
                • Always call 911 for medical emergencies
                • The app's guidance is based on current emergency protocols
                • Users should seek professional medical training when possible
                
                **Emergency Response Disclaimer:**
                • The app is designed to assist in emergency situations
                • It cannot guarantee outcomes in medical emergencies
                • Users should follow local emergency protocols
                • The app works best when used in conjunction with professional emergency services
                
                **Technology Disclaimer:**
                • Voice recognition may not work in all environments
                • Internet connectivity may be required for some features
                • The app requires proper permissions to function
                • Users should test the app in safe environments first
                
                **Gamification Disclaimer:**
                • The gamification features are designed to encourage learning
                • They should not distract from the serious nature of emergency response
                • The primary goal is life-saving, not earning rewards
                • Training should be completed before emergency situations
                
                **Community Safety:**
                • The app encourages building trusted networks
                • Users should only add trusted contacts
                • Location sharing is only used during emergencies
                • Privacy is maintained except during emergency situations
                
                **Remember:** This app is designed to help save lives, but it is not a replacement for professional emergency services or medical care.
            """.trimIndent())
            .setPositiveButton("I Understand") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showDeveloperInfoDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("👨‍💻 Developer Information")
            .setMessage("""
                **OPEN SOURCE PROJECT**
                
                **GitHub Repository:**
                github.com/paragoner1/solana-sos
                
                **License:**
                Apache 2.0 - Open source and free to use
                
                **Contributions Welcome:**
                • Fork the repository
                • Create feature branches
                • Submit pull requests
                • Report issues and bugs
                
                **Contact Information:**
                • Email: paragoner1@icloud.com
                • Twitter: @paragoner1
                • GitHub: @paragoner1
                
                **Development Philosophy:**
                • Built for the Solana ecosystem
                • Expandable to other blockchain platforms
                • Community-driven development
                • Open source for transparency
                
                **Technical Stack:**
                • Android (Kotlin)
                • Solana Blockchain Integration
                • Voice Recognition Technology
                • Emergency Response Protocols
                
                **Mission:**
                To save lives by making emergency response accessible to everyone through voice-activated, blockchain-powered technology that works anywhere, anytime.
                
                **Future Plans:**
                • Expand to iOS platform
                • Add more emergency types
                • Integrate with more blockchain networks
                • Community governance implementation
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showMissionStatementDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🎯 Mission Statement")
            .setMessage("""
                **DISCLAIMER: This is NOT a game**
                
                Solana SOS is a serious emergency response application designed to prevent unnecessary deaths when lives could otherwise be saved. This app is intended for real emergency situations only.
                
                **Our Mission:**
                To save lives by making emergency response accessible to everyone through voice-activated, blockchain-powered technology that works anywhere, anytime, even offline. Anyone can be a Hero.
                
                **Important Notes:**
                • This app is NOT a replacement for professional medical care
                • Always call 911 in emergency situations
                • The app provides guidance but you are responsible for your actions
                • Use only in genuine emergency situations
                • Training is recommended before using in real emergencies
                
                **Emergency Use Only:**
                This application is designed for life-threatening emergency situations. Do not use for non-emergency situations or testing purposes.
            """.trimIndent())
            .setPositiveButton("I Understand") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showAuthoritativeSourcesDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("📚 Authoritative Sources & Credible Authorities")
            .setMessage("""
                **LIFE-SAVING GUIDANCE SOURCES**
                
                All emergency response instructions in Solana SOS are verified against current medical standards from these authoritative sources:
                
                **American Heart Association (AHA)**
                • CPR guidelines and protocols
                • Emergency cardiovascular care standards
                • Life support training programs
                
                **American Red Cross**
                • Emergency response protocols
                • First aid training standards
                • Disaster response guidelines
                
                **Centers for Disease Control (CDC)**
                • Medical guidelines and protocols
                • Public health emergency standards
                • Injury prevention guidelines
                
                **World Health Organization (WHO)**
                • International medical standards
                • Emergency response protocols
                • Global health guidelines
                
                **National Institutes of Health (NIH)**
                • Medical research and standards
                • Clinical practice guidelines
                • Evidence-based protocols
                
                **Emergency Medical Services (EMS)**
                • Professional emergency protocols
                • Pre-hospital care standards
                • Emergency response procedures
                
                **Protocol Verification:**
                All emergency response instructions are regularly updated to reflect the latest best practices from these authoritative sources. The app maintains current medical standards for all emergency protocols.
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showPrivacySecurityDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🔒 Privacy & Security")
            .setMessage("""
                **YOUR PRIVACY MATTERS**
                
                **Voice Data Processing:**
                • All voice recognition processed locally on your device
                • No voice data sent to external servers
                • Privacy-first approach to emergency response
                
                **Location Sharing:**
                • Location shared only during actual emergencies
                • Precise coordinates sent to emergency services
                • Location shared with trusted contacts during emergencies
                • No location tracking during normal use
                
                **Data Storage:**
                • No personal data stored in the cloud
                • Emergency logs contain only metadata (no personal info)
                • Blockchain logs are anonymous and encrypted
                
                **Security Features:**
                • End-to-end encryption for all communications
                • Secure voice recognition technology
                • Encrypted emergency logs on Solana blockchain
                • Anonymous emergency reporting
                • No tracking or profiling of users
                
                **Emergency Data:**
                • Only emergency response metadata is logged
                • No personal information in blockchain records
                • Anonymous emergency reporting system
                • Data used only for emergency response verification
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showDeveloperInfoDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("👨‍💻 Developer Information")
            .setMessage("""
                **OPEN SOURCE PROJECT**
                
                **GitHub Repository:**
                https://github.com/paragoner1/crisis-companion
                
                **License:** Apache 2.0
                **Contributions:** Welcome and encouraged
                **Community:** Open to all developers
                
                **CONTACT INFORMATION**
                
                **Email:** paragoner.dev@gmail.com
                **X (Twitter):** @paragoner1
                **GitHub:** @paragoner1
                
                **DEVELOPMENT PHILOSOPHY**
                
                **Built for Solana Ecosystem:**
                • Leverages Solana blockchain for transparency
                • Integrates with Solana DeFi and gaming platforms
                • Community-driven development approach
                
                **Expandable Platform:**
                • Designed to work with other blockchain platforms
                • Modular architecture for easy expansion
                • Open source for community contributions
                
                **Community-Driven:**
                • Open source development
                • Community feedback and contributions
                • Transparent development process
                
                **Get Involved:**
                • Report bugs on GitHub
                • Suggest features via GitHub issues
                • Contribute code and improvements
                • Join the community discussion
            """.trimIndent())
            .setPositiveButton("Visit GitHub") { dialog, _ ->
                dialog.dismiss()
                // Could add intent to open GitHub URL
            }
            .setNegativeButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showMissionStatementDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🎯 Mission Statement")
            .setMessage("""
                **DISCLAIMER: This is NOT a game**
                
                Solana SOS is a serious emergency response application designed to prevent unnecessary deaths when lives could otherwise be saved. This app is intended for real emergency situations only.
                
                **Our Mission:**
                To save lives by making emergency response accessible to everyone through voice-activated, blockchain-powered technology that works anywhere, anytime, even offline. Anyone can be a Hero.
                
                **Important Notes:**
                • This app is NOT a replacement for professional medical care
                • Always call 911 in emergency situations
                • The app provides guidance but you are responsible for your actions
                • Use only in genuine emergency situations
                • Training is recommended before using in real emergencies
                
                **Emergency Use Only:**
                This application is designed for life-threatening emergency situations. Do not use for non-emergency situations or testing purposes.
            """.trimIndent())
            .setPositiveButton("I Understand") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showWhatIsSolanaSosDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🚨 What is Solana SOS?")
            .setMessage("""
                **VOICE-ACTIVATED EMERGENCY RESPONSE**
                
                Imagine you're at the beach and someone starts drowning. You panic - what do you do? Most people freeze or forget basic lifesaving steps. **Solana SOS** is like having a trained emergency responder in your pocket who guides you through any emergency, step by step.
                
                **🏆 How It Works (In Simple Terms)**
                
                **1. You Say "Hey SOS"**
                • Just like saying "Hey Siri" or "Hey Google"
                • The app wakes up and listens for what emergency you're dealing with
                • Works even in loud places like beaches, pools, or crowded areas
                
                **2. The App Understands Your Situation**
                • You say: "Hey SOS, someone's drowning and I pulled them out of the water"
                • The app thinks: "They're past the rescue phase, need medical care now"
                • Instead of telling you to call for help (you already did that), it jumps straight to checking if the person is breathing
                
                **3. It Guides You Step by Step**
                • Clear, simple instructions: "Check if they're breathing"
                • If they're not breathing: "Start rescue breathing now"
                • If they have a pulse: "Keep them warm and monitor their breathing"
                • No confusing medical jargon - just what you need to do right now
                
                **🚨 Real-World Examples**
                
                **Example 1: Drowning at the Beach**
                You: "Hey SOS, drowning emergency"
                App: "Emergency detected. Is the person still in the water?"
                You: "No, I pulled them out"
                App: "Check if they're breathing and have a pulse"
                App: "If not breathing, start rescue breathing immediately"
                App: "I'm calling 911 and sharing your location"
                
                **What the app saved you from:**
                • Wasting time on "stay calm" instructions when you're already acting
                • Confusing medical terms
                • Not knowing what to do next
                
                **🎮 The Fun Part: Becoming a Hero**
                
                **Level Up Like a Video Game**
                • Start as a **Novice Hero** (just like you!)
                • Complete training to become a **Trainee Hero**
                • Respond to emergencies to become an **Emergency Responder**
                • Keep going until you're a **Life-Saving Legend**
                
                **Earn Rewards**
                • **Experience Points** for learning and helping
                • **BONK Tokens** for emergency responses
                • **SKR Tokens** for building your safety network
                • **Badges** for achievements like "CPR Master" or "Quick Responder"
                
                **🚨 Emergency Types It Handles**
                
                **Medical Emergencies:** Heart Attack, Stroke, Choking, Allergic Reaction, Diabetic Emergency
                **Injury Emergencies:** Bleeding, Severe Burns, Unconscious, Trauma
                **Environmental Emergencies:** Drowning, Poisoning, Seizure
                
                **🎯 Why It's Different**
                
                **Traditional Apps vs. Solana SOS**
                
                **Traditional Emergency Apps:**
                • Require you to open the app manually
                • Need internet connection
                • Give generic instructions
                • Don't understand your specific situation
                
                **Solana SOS:**
                • Voice-activated (hands-free)
                • Works offline
                • Understands context (saves 45 seconds)
                • Calls 911 automatically
                • Alerts your personal network
                
                **Real Impact:**
                • **45 seconds saved** per emergency
                • **5-10 minute advantage** over emergency services
                • **Community safety** - friends arrive before EMS
                • **Confidence building** - you know what to do
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showHowItWorksDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("⚡ How It Works")
            .setMessage("""
                **STEP-BY-STEP EMERGENCY RESPONSE PROCESS**
                
                **1. Voice Activation**
                Say "Hey SOS" followed by the emergency type (e.g., "drowning help", "heart attack", "choking")
                • Uses advanced voice recognition technology
                • Works in noisy environments
                • Processes locally on your device for privacy
                
                **2. Instant Response**
                App recognizes the emergency and provides immediate guidance
                • Response time: Under 100 milliseconds
                • Context-aware instructions
                • Clear, step-by-step guidance
                
                **3. Automatic Actions**
                App calls 911 and shares location automatically
                • No manual dialing required
                • GPS coordinates sent to emergency services
                • Location shared with trusted contacts
                
                **4. Life-Saving Instructions**
                Step-by-step guidance for the specific emergency
                • CPR techniques for heart attacks
                • Heimlich maneuver for choking
                • First aid for bleeding, burns, etc.
                
                **5. Smart Integration**
                911 operator stays on standby for when you're ready to connect
                • Operator has context of the emergency
                • No need to repeat information
                • Seamless handoff to emergency services
                
                **6. Blockchain Logging**
                Emergency response recorded on Solana blockchain
                • Transparent emergency response tracking
                • Anonymous emergency metadata
                • Community safety verification
                
                **🎯 Key Benefits:**
                • **45 seconds saved** per emergency
                • **Context-aware** responses
                • **Offline functionality**
                • **Privacy-first** approach
                • **Community integration**
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showAuthoritativeSourcesDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("📚 Authoritative Sources & Credible Authorities")
            .setMessage("""
                **LIFE-SAVING GUIDANCE SOURCES**
                
                All emergency response instructions in Solana SOS are verified against current medical standards from these authoritative sources:
                
                **American Heart Association (AHA)**
                • CPR guidelines and protocols
                • Emergency cardiovascular care standards
                • Life support training programs
                
                **American Red Cross**
                • Emergency response protocols
                • First aid training standards
                • Disaster response guidelines
                
                **Centers for Disease Control (CDC)**
                • Medical guidelines and protocols
                • Public health emergency standards
                • Injury prevention guidelines
                
                **World Health Organization (WHO)**
                • International medical standards
                • Emergency response protocols
                • Global health guidelines
                
                **National Institutes of Health (NIH)**
                • Medical research and standards
                • Clinical practice guidelines
                • Evidence-based protocols
                
                **Emergency Medical Services (EMS)**
                • Professional emergency protocols
                • Pre-hospital care standards
                • Emergency response procedures
                
                **Protocol Verification:**
                All emergency response instructions are regularly updated to reflect the latest best practices from these authoritative sources. The app maintains current medical standards for all emergency protocols.
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showPrivacySecurityDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🔒 Privacy & Security")
            .setMessage("""
                **YOUR PRIVACY MATTERS**
                
                **Voice Data Processing:**
                • All voice recognition processed locally on your device
                • No voice data sent to external servers
                • Privacy-first approach to emergency response
                
                **Location Sharing:**
                • Location shared only during actual emergencies
                • Precise coordinates sent to emergency services
                • Location shared with trusted contacts during emergencies
                • No location tracking during normal use
                
                **Data Storage:**
                • No personal data stored in the cloud
                • Emergency logs contain only metadata (no personal info)
                • Blockchain logs are anonymous and encrypted
                
                **Security Features:**
                • End-to-end encryption for all communications
                • Secure voice recognition technology
                • Encrypted emergency logs on Solana blockchain
                • Anonymous emergency reporting
                • No tracking or profiling of users
                
                **Emergency Data:**
                • Only emergency response metadata is logged
                • No personal information in blockchain records
                • Anonymous emergency reporting system
                • Data used only for emergency response verification
            """.trimIndent())
            .setPositiveButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun showDeveloperInfoDetails() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("👨‍💻 Developer Information")
            .setMessage("""
                **OPEN SOURCE PROJECT**
                
                **GitHub Repository:**
                https://github.com/paragoner1/crisis-companion
                
                **License:** Apache 2.0
                **Contributions:** Welcome and encouraged
                **Community:** Open to all developers
                
                **CONTACT INFORMATION**
                
                **Email:** paragoner.dev@gmail.com
                **X (Twitter):** @paragoner1
                **GitHub:** @paragoner1
                
                **DEVELOPMENT PHILOSOPHY**
                
                **Built for Solana Ecosystem:**
                • Leverages Solana blockchain for transparency
                • Integrates with Solana DeFi and gaming platforms
                • Community-driven development approach
                
                **Expandable Platform:**
                • Designed to work with other blockchain platforms
                • Modular architecture for easy expansion
                • Open source for community contributions
                
                **Community-Driven:**
                • Open source development
                • Community feedback and contributions
                • Transparent development process
                
                **Get Involved:**
                • Report bugs on GitHub
                • Suggest features via GitHub issues
                • Contribute code and improvements
                • Join the community discussion
            """.trimIndent())
            .setPositiveButton("Visit GitHub") { dialog, _ ->
                dialog.dismiss()
                // Could add intent to open GitHub URL
            }
            .setNegativeButton("Got It") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }
} 