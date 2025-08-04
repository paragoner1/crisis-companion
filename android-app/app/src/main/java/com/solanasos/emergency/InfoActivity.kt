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
} 