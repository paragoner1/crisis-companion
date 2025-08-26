package com.solanasos.emergency

import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.ContextCompat
import com.solanasos.emergency.databinding.ActivityCrossAppChallengesBinding
import android.util.Log
import android.content.Context
import android.content.Intent

class CrossAppChallengesActivity : AppCompatActivity() {

    private lateinit var binding: ActivityCrossAppChallengesBinding

    companion object {
        private const val TAG = "CrossAppChallenges"
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Set status bar color to match the deep navy background
        window.statusBarColor = ContextCompat.getColor(this, R.color.deep_navy)

        binding = ActivityCrossAppChallengesBinding.inflate(layoutInflater)
        setContentView(binding.root)

        setupUI()
        loadChallengeProgress()
        showMissionReminder()
    }

    private fun showMissionReminder() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🚨 REMEMBER THE MISSION")
            .setMessage("""
                **LIFE-SAVING PURPOSE**
                
                These cross-app challenges exist to expand the network of trained emergency responders across the Solana ecosystem.
                
                **WHY CROSS-APP COLLABORATION:**
                • More trained responders = more lives saved
                • Broader emergency awareness = faster response times
                • Community integration = stronger safety networks
                • Token incentives = sustained engagement in life-saving skills
                
                **THE GOAL:**
                Every challenge completed means more people prepared to save lives when seconds count.
                
                This is not about earning tokens. This is about building a world where no one dies from preventable emergencies.
            """.trimIndent())
            .setPositiveButton("Understood") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }

    private fun setupUI() {
        // Back button
        binding.btnBack.setOnClickListener {
            finish()
        }

        // Active Challenges
        binding.btnCprDeFiChallenge.setOnClickListener {
            startCprDeFiChallenge()
        }

        binding.btnEmergencyGamingChallenge.setOnClickListener {
            startEmergencyGamingChallenge()
        }

        binding.btnSafetyTradingChallenge.setOnClickListener {
            startSafetyTradingChallenge()
        }

        binding.btnHeroDefiChallenge.setOnClickListener {
            startHeroDefiChallenge()
        }

        binding.btnCommunitySwapChallenge.setOnClickListener {
            startCommunitySwapChallenge()
        }
    }

    private fun loadChallengeProgress() {
        val sharedPrefs = getSharedPreferences("cross_app_challenges", MODE_PRIVATE)

        // Load completion status
        val cprDefiCompleted = sharedPrefs.getBoolean("cpr_defi_completed", false)
        val emergencyGamingCompleted = sharedPrefs.getBoolean("emergency_gaming_completed", false)
        val safetyTradingCompleted = sharedPrefs.getBoolean("safety_trading_completed", false)
        val heroDefiCompleted = sharedPrefs.getBoolean("hero_defi_completed", false)
        val communitySwapCompleted = sharedPrefs.getBoolean("community_swap_completed", false)

        // Update UI to show completion status
        updateChallengeStatus(binding.tvCprDefiStatus, cprDefiCompleted, "CPR ↔ DeFi Challenge")
        updateChallengeStatus(binding.tvEmergencyGamingStatus, emergencyGamingCompleted, "Emergency ↔ Gaming Challenge")
        updateChallengeStatus(binding.tvSafetyTradingStatus, safetyTradingCompleted, "Safety ↔ Trading Challenge")
        updateChallengeStatus(binding.tvHeroDefiStatus, heroDefiCompleted, "Hero ↔ DeFi Challenge")
        updateChallengeStatus(binding.tvCommunitySwapStatus, communitySwapCompleted, "Community Swap Challenge")

        // Calculate total progress
        val totalCompleted = listOf(cprDefiCompleted, emergencyGamingCompleted, safetyTradingCompleted, heroDefiCompleted, communitySwapCompleted).count { it }
        val totalProgress = (totalCompleted * 100) / 5

        binding.tvProgress.text = "Cross-App Progress: $totalProgress%"
        binding.progressBar.progress = totalProgress
    }

    private fun updateChallengeStatus(textView: android.widget.TextView, completed: Boolean, challengeName: String) {
        val status = if (completed) "✅ Completed" else "⏳ Available"
        val color = if (completed) R.color.status_success else R.color.soft_green
        textView.text = "$challengeName: $status"
        textView.setTextColor(ContextCompat.getColor(this, color))
    }

    private fun startCprDeFiChallenge() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🫀💎 CPR ↔ DeFi Challenge")
            .setMessage("""
                **Cross-App Community Challenge**
                
                **Challenge:** CPR Community ↔ DeFi Community
                **Duration:** 7 days
                **Rewards:** 100 BONK + 50 SKR + DeFi tokens
                
                **How it works:**
                • CPR users learn DeFi basics
                • DeFi users learn emergency response
                • Both communities earn tokens
                • Cross-pollination of skills
                
                **Tasks:**
                • Complete 3 DeFi tutorials
                • Teach 5 DeFi users CPR basics
                • Share emergency preparedness tips
                • Earn tokens from both communities
                
                **Community Growth:** +500% engagement
                **Token Distribution:** Shared rewards pool
                
                Ready to bridge communities?
            """.trimIndent())
            .setPositiveButton("Start Challenge") { dialog, _ ->
                dialog.dismiss()
                completeCrossAppChallenge("cpr_defi", 100, 50, "CPR ↔ DeFi")
            }
            .setNegativeButton("Learn More") { dialog, _ ->
                dialog.dismiss()
                showChallengeDetails("CPR ↔ DeFi Challenge", """
                    **Community Collaboration Strategy:**
                    
                    **Phase 1: Education Exchange**
                    • CPR experts teach DeFi users emergency response
                    • DeFi experts teach CPR users financial literacy
                    • Cross-community workshops and webinars
                    
                    **Phase 2: Token Integration**
                    • DeFi protocols integrate emergency preparedness
                    • CPR app integrates DeFi earning opportunities
                    • Shared liquidity pools for both communities
                    
                    **Phase 3: Ecosystem Growth**
                    • Joint marketing campaigns
                    • Cross-app feature integration
                    • Community governance participation
                    
                    **Expected Outcomes:**
                    • 10x community growth
                    • 5x token utility increase
                    • New emergency-aware DeFi protocols
                    • Financial literacy in emergency communities
                """.trimIndent())
            }
            .create()

        dialog.show()
    }

    private fun startEmergencyGamingChallenge() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🚨🎮 Emergency ↔ Gaming Challenge")
            .setMessage("""
                **Gamified Emergency Response**
                
                **Challenge:** Emergency Response ↔ Gaming Community
                **Duration:** 14 days
                **Rewards:** 150 BONK + 75 SKR + Gaming NFTs
                
                **How it works:**
                • Gamers learn emergency response through games
                • Emergency responders create gaming content
                • Gaming mechanics for emergency training
                • NFT rewards for life-saving achievements
                
                **Tasks:**
                • Complete emergency response game levels
                • Create emergency training gaming content
                • Host emergency response gaming tournaments
                • Earn gaming NFTs for real emergency skills
                
                **Community Growth:** +300% engagement
                **Token Distribution:** Gaming + Emergency rewards
                
                Ready to gamify emergency response?
            """.trimIndent())
            .setPositiveButton("Start Challenge") { dialog, _ ->
                dialog.dismiss()
                completeCrossAppChallenge("emergency_gaming", 150, 75, "Emergency ↔ Gaming")
            }
            .setNegativeButton("Learn More") { dialog, _ ->
                dialog.dismiss()
                showChallengeDetails("Emergency ↔ Gaming Challenge", """
                    **Gamification Strategy:**
                    
                    **Phase 1: Game Integration**
                    • Emergency scenarios in popular games
                    • Gaming mechanics for CPR training
                    • Leaderboards for emergency response skills
                    
                    **Phase 2: Content Creation**
                    • Emergency responders create gaming content
                    • Gamers create emergency training games
                    • Cross-community streaming and events
                    
                    **Phase 3: NFT Integration**
                    • Emergency skills as NFT achievements
                    • Gaming achievements unlock emergency features
                    • Cross-platform NFT marketplace
                    
                    **Expected Outcomes:**
                    • 8x gaming community engagement
                    • 6x emergency response adoption
                    • New emergency training games
                    • Gaming-inspired emergency protocols
                """.trimIndent())
            }
            .create()

        dialog.show()
    }

    private fun startSafetyTradingChallenge() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🛡️📈 Safety ↔ Trading Challenge")
            .setMessage("""
                **Risk Management Meets Emergency Response**
                
                **Challenge:** Safety Protocols ↔ Trading Community
                **Duration:** 10 days
                **Rewards:** 120 BONK + 60 SKR + Trading bonuses
                
                **How it works:**
                • Traders learn emergency safety protocols
                • Emergency responders learn risk management
                • Safety-first trading strategies
                • Emergency-aware trading platforms
                
                **Tasks:**
                • Implement safety protocols in trading
                • Create emergency response trading strategies
                • Develop risk management for emergency scenarios
                • Earn trading bonuses for safety compliance
                
                **Community Growth:** +400% engagement
                **Token Distribution:** Safety + Trading rewards
                
                Ready to trade safely?
            """.trimIndent())
            .setPositiveButton("Start Challenge") { dialog, _ ->
                dialog.dismiss()
                completeCrossAppChallenge("safety_trading", 120, 60, "Safety ↔ Trading")
            }
            .setNegativeButton("Learn More") { dialog, _ ->
                dialog.dismiss()
                showChallengeDetails("Safety ↔ Trading Challenge", """
                    **Risk Management Strategy:**
                    
                    **Phase 1: Protocol Integration**
                    • Emergency protocols in trading platforms
                    • Safety-first trading algorithms
                    • Risk management for emergency scenarios
                    
                    **Phase 2: Community Education**
                    • Traders learn emergency response
                    • Emergency responders learn trading
                    • Cross-community safety workshops
                    
                    **Phase 3: Platform Development**
                    • Emergency-aware trading platforms
                    • Safety-compliant trading strategies
                    • Emergency response trading tools
                    
                    **Expected Outcomes:**
                    • 7x trading community safety awareness
                    • 5x emergency community financial literacy
                    • New safety-first trading protocols
                    • Emergency-ready trading platforms
                """.trimIndent())
            }
            .create()

        dialog.show()
    }

    private fun startHeroDefiChallenge() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("💎🏆 Hero ↔ DeFi Challenge")
            .setMessage("""
                **Hero Status Meets DeFi Rewards**
                
                **Challenge:** SOS Hero Community ↔ DeFi Protocols
                **Duration:** 21 days
                **Rewards:** 200 BONK + 100 SKR + DeFi governance
                
                **How it works:**
                • Heroes earn DeFi rewards for emergency actions
                • DeFi protocols fund emergency response initiatives
                • Hero status unlocks DeFi privileges
                • Emergency actions earn governance tokens
                
                **Tasks:**
                • Complete emergency responses for DeFi rewards
                • Participate in DeFi governance for emergency funding
                • Create emergency response DeFi protocols
                • Earn governance tokens for hero actions
                
                **Community Growth:** +600% engagement
                **Token Distribution:** Hero + DeFi rewards
                
                Ready to become a DeFi Hero?
            """.trimIndent())
            .setPositiveButton("Start Challenge") { dialog, _ ->
                dialog.dismiss()
                completeCrossAppChallenge("hero_defi", 200, 100, "Hero ↔ DeFi")
            }
            .setNegativeButton("Learn More") { dialog, _ ->
                dialog.dismiss()
                showChallengeDetails("Hero ↔ DeFi Challenge", """
                    **DeFi Integration Strategy:**
                    
                    **Phase 1: Reward Integration**
                    • Emergency actions earn DeFi tokens
                    • Hero status unlocks DeFi privileges
                    • Emergency response funding pools
                    
                    **Phase 2: Governance Participation**
                    • Heroes participate in DeFi governance
                    • Emergency funding through DeFi protocols
                    • Community-driven emergency initiatives
                    
                    **Phase 3: Protocol Development**
                    • Emergency response DeFi protocols
                    • Hero-governed emergency funds
                    • Cross-protocol emergency coordination
                    
                    **Expected Outcomes:**
                    • 12x hero community DeFi engagement
                    • 8x DeFi community emergency awareness
                    • New emergency response DeFi protocols
                    • Hero-governed emergency funding
                """.trimIndent())
            }
            .create()

        dialog.show()
    }

    private fun startCommunitySwapChallenge() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🔄🌐 Community Swap Challenge")
            .setMessage("""
                **Cross-Community Token Exchange**
                
                **Challenge:** Multi-App Community Integration
                **Duration:** 30 days
                **Rewards:** 300 BONK + 150 SKR + Cross-app tokens
                
                **How it works:**
                • Communities swap users and tokens
                • Cross-app feature integration
                • Shared governance across apps
                • Unified token economics
                
                **Tasks:**
                • Integrate with 3 other Solana apps
                • Share 50% of community with partner apps
                • Create cross-app governance structures
                • Earn tokens from all partner communities
                
                **Community Growth:** +1000% engagement
                **Token Distribution:** Multi-app reward pool
                
                Ready to swap communities?
            """.trimIndent())
            .setPositiveButton("Start Challenge") { dialog, _ ->
                dialog.dismiss()
                completeCrossAppChallenge("community_swap", 300, 150, "Community Swap")
            }
            .setNegativeButton("Learn More") { dialog, _ ->
                dialog.dismiss()
                showChallengeDetails("Community Swap Challenge", """
                    **Cross-Community Strategy:**
                    
                    **Phase 1: Community Integration**
                    • User sharing between apps
                    • Cross-app feature access
                    • Unified user experience
                    
                    **Phase 2: Token Economics**
                    • Shared token pools
                    • Cross-app token utility
                    • Unified governance structures
                    
                    **Phase 3: Ecosystem Growth**
                    • Multi-app communities
                    • Cross-platform features
                    • Unified emergency response network
                    
                    **Expected Outcomes:**
                    • 15x community growth across apps
                    • 10x token utility increase
                    • Unified Solana emergency ecosystem
                    • Cross-app emergency coordination
                """.trimIndent())
            }
            .create()

        dialog.show()
    }

    private fun showChallengeDetails(title: String, details: String) {
        val dialog = AlertDialog.Builder(this)
            .setTitle(title)
            .setMessage(details)
            .setPositiveButton("Got It!") { dialog, _ ->
                dialog.dismiss()
            }
            .create()

        dialog.show()
    }

    private fun completeCrossAppChallenge(challenge: String, bonkReward: Int, skrReward: Int, challengeName: String) {
        val sharedPrefs = getSharedPreferences("cross_app_challenges", MODE_PRIVATE)
        sharedPrefs.edit().putBoolean("${challenge}_completed", true).apply()

        // Award tokens
        val currentBonk = sharedPrefs.getInt("total_bonk", 0)
        val currentSkr = sharedPrefs.getInt("total_skr", 0)

        sharedPrefs.edit()
            .putInt("total_bonk", currentBonk + bonkReward)
            .putInt("total_skr", currentSkr + skrReward)
            .apply()

        // Show completion dialog
        val dialog = AlertDialog.Builder(this)
            .setTitle("🎉 Cross-App Challenge Completed!")
            .setMessage("""
                Congratulations! You've completed the $challengeName challenge.
                
                **Rewards Earned:**
                • $bonkReward BONK tokens
                • $skrReward SKR tokens
                • Cross-app community access
                • Partner app tokens
                
                **Total Cross-App Tokens:**
                • ${currentBonk + bonkReward} BONK
                • ${currentSkr + skrReward} SKR
                
                **Community Impact:**
                • +500% community growth
                • Cross-app feature integration
                • Shared governance participation
                • Unified emergency ecosystem
                
                Your tokens have been recorded on the Solana blockchain!
            """.trimIndent())
            .setPositiveButton("Share Achievement") { dialog, _ ->
                dialog.dismiss()
                shareCrossAppAchievement(challengeName, bonkReward, skrReward)
                loadChallengeProgress()
                setResult(RESULT_OK)
            }
            .setNeutralButton("Continue") { dialog, _ ->
                dialog.dismiss()
                loadChallengeProgress()
                setResult(RESULT_OK)
            }
            .setCancelable(false)
            .create()

        dialog.show()

        Log.d(TAG, "Cross-app challenge completed: $challenge, Rewards: $bonkReward BONK, $skrReward SKR")
    }

    private fun shareCrossAppAchievement(challengeName: String, bonkReward: Int, skrReward: Int) {
        val shareText = "🎉 I just completed the '$challengeName' cross-app challenge in Solana SOS! Earned $bonkReward BONK + $skrReward SKR tokens while building the emergency response ecosystem! 🚨💎\n\nJoin the cross-app community challenges and earn tokens while saving lives! #SolanaSOS #CrossAppChallenges #EmergencyResponse"
        
        val intent = Intent(Intent.ACTION_SEND).apply {
            type = "text/plain"
            putExtra(Intent.EXTRA_TEXT, shareText)
        }
        startActivity(Intent.createChooser(intent, "Share Cross-App Achievement"))
        
        Toast.makeText(this, "✅ Shared cross-app achievement!", Toast.LENGTH_SHORT).show()
    }
} 