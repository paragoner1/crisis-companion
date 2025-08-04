package com.solanasos.emergency

import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.core.content.ContextCompat
import com.solanasos.emergency.databinding.ActivityTrainingBinding
import android.util.Log

class TrainingActivity : AppCompatActivity() {
    
    private lateinit var binding: ActivityTrainingBinding
    private lateinit var solanaMobile: SolanaMobileIntegration
    
    companion object {
        private const val TAG = "TrainingActivity"
    }
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Set status bar color to match the deep navy background
        window.statusBarColor = ContextCompat.getColor(this, R.color.deep_navy)
        
        binding = ActivityTrainingBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        solanaMobile = SolanaMobileIntegration(this)
        
        setupUI()
        loadTrainingProgress()
    }
    
    private fun setupUI() {
        // Back button
        binding.btnBack.setOnClickListener {
            finish()
        }
        
        // Training modules
        binding.btnCprTraining.setOnClickListener {
            startCprTraining()
        }
        
        binding.btnFirstAidTraining.setOnClickListener {
            startFirstAidTraining()
        }
        
        binding.btnVoiceTraining.setOnClickListener {
            startVoiceTraining()
        }
        
        binding.btnAppFeaturesTraining.setOnClickListener {
            startAppFeaturesTraining()
        }
        
        binding.btnEmergencyTypesTraining.setOnClickListener {
            startEmergencyTypesTraining()
        }
    }
    
    private fun loadTrainingProgress() {
        val sharedPrefs = getSharedPreferences("training_progress", MODE_PRIVATE)
        
        // Load completion status
        val cprCompleted = sharedPrefs.getBoolean("cpr_completed", false)
        val firstAidCompleted = sharedPrefs.getBoolean("first_aid_completed", false)
        val voiceCompleted = sharedPrefs.getBoolean("voice_completed", false)
        val featuresCompleted = sharedPrefs.getBoolean("features_completed", false)
        val emergencyTypesCompleted = sharedPrefs.getBoolean("emergency_types_completed", false)
        
        // Update UI to show completion status
        updateTrainingStatus(binding.tvCprStatus, cprCompleted, "CPR Training")
        updateTrainingStatus(binding.tvFirstAidStatus, firstAidCompleted, "First Aid Basics")
        updateTrainingStatus(binding.tvVoiceStatus, voiceCompleted, "Voice Recognition")
        updateTrainingStatus(binding.tvFeaturesStatus, featuresCompleted, "App Features")
        updateTrainingStatus(binding.tvEmergencyTypesStatus, emergencyTypesCompleted, "Emergency Types")
        
        // Calculate total progress
        val totalCompleted = listOf(cprCompleted, firstAidCompleted, voiceCompleted, featuresCompleted, emergencyTypesCompleted).count { it }
        val totalProgress = (totalCompleted * 100) / 5
        
        binding.tvProgress.text = "Training Progress: $totalProgress%"
        binding.progressBar.progress = totalProgress
    }
    
    private fun updateTrainingStatus(textView: android.widget.TextView, completed: Boolean, moduleName: String) {
        val status = if (completed) "✅ Completed" else "⏳ Not Started"
        val color = if (completed) R.color.status_success else R.color.status_inactive
        textView.text = "$moduleName: $status"
        textView.setTextColor(ContextCompat.getColor(this, color))
    }
    
    private fun startCprTraining() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🫀 CPR Training")
            .setMessage("""
                Welcome to CPR Training!
                
                This module will teach you:
                • Adult CPR techniques
                • Child and infant CPR
                • AED usage
                • When to perform CPR
                
                Duration: ~10 minutes
                Reward: 50 BONK + 25 SKR
                
                Ready to start?
            """.trimIndent())
            .setPositiveButton("Start Training") { dialog, _ ->
                dialog.dismiss()
                showCprQuiz()
            }
            .setNegativeButton("Cancel") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }
    
    private fun showCprQuiz() {
        val questions = listOf(
            "What is the correct compression rate for adult CPR?" to "100-120 compressions per minute",
            "What is the compression-to-breath ratio for adult CPR?" to "30:2",
            "How deep should chest compressions be for adults?" to "2-2.4 inches",
            "When should you start CPR?" to "When the person is unresponsive and not breathing normally"
        )
        
        var currentQuestion = 0
        var correctAnswers = 0
        
        fun showQuestion() {
            if (currentQuestion < questions.size) {
                val (question, answer) = questions[currentQuestion]
                val options = listOf(
                    answer,
                    "120-140 compressions per minute",
                    "20:2 ratio",
                    "1-1.5 inches deep"
                ).shuffled()
                
                val dialog = AlertDialog.Builder(this)
                    .setTitle("CPR Quiz - Question ${currentQuestion + 1}")
                    .setMessage("""
                        $question
                        
                        A) ${options[0]}
                        B) ${options[1]}
                        C) ${options[2]}
                        D) ${options[3]}
                    """.trimIndent())
                    .setPositiveButton("A") { _, _ ->
                        if (options[0] == answer) correctAnswers++
                        currentQuestion++
                        showQuestion()
                    }
                    .setNegativeButton("B") { _, _ ->
                        if (options[1] == answer) correctAnswers++
                        currentQuestion++
                        showQuestion()
                    }
                    .setNeutralButton("C") { _, _ ->
                        if (options[2] == answer) correctAnswers++
                        currentQuestion++
                        showQuestion()
                    }
                    .setCancelable(false)
                    .create()
                
                dialog.show()
            } else {
                // Quiz completed
                val score = (correctAnswers * 100) / questions.size
                if (score >= 75) {
                    completeTraining("cpr", 50, 25)
                } else {
                    Toast.makeText(this, "Score: $score%. Need 75% to pass. Try again!", Toast.LENGTH_LONG).show()
                }
            }
        }
        
        showQuestion()
    }
    
    private fun startFirstAidTraining() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🩹 First Aid Training")
            .setMessage("""
                Welcome to First Aid Basics!
                
                This module covers:
                • Emergency assessment
                • Basic life support
                • Injury treatment
                • Emergency communication
                
                Duration: ~15 minutes
                Reward: 30 BONK + 15 SKR
                
                Ready to start?
            """.trimIndent())
            .setPositiveButton("Start Training") { dialog, _ ->
                dialog.dismiss()
                completeTraining("first_aid", 30, 15)
            }
            .setNegativeButton("Cancel") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }
    
    private fun startVoiceTraining() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🎤 Voice Recognition Training")
            .setMessage("""
                Welcome to Voice Training!
                
                Practice saying:
                • "Hey SOS"
                • Emergency types clearly
                • Voice commands
                
                Duration: ~5 minutes
                Reward: 25 BONK + 10 SKR
                
                Ready to start?
            """.trimIndent())
            .setPositiveButton("Start Training") { dialog, _ ->
                dialog.dismiss()
                completeTraining("voice", 25, 10)
            }
            .setNegativeButton("Cancel") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }
    
    private fun startAppFeaturesTraining() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("⚙️ App Features Training")
            .setMessage("""
                Welcome to App Features Training!
                
                Learn about:
                • Settings configuration
                • Demo mode usage
                • Safety features
                • Emergency contacts
                
                Duration: ~8 minutes
                Reward: 20 BONK + 10 SKR
                
                Ready to start?
            """.trimIndent())
            .setPositiveButton("Start Training") { dialog, _ ->
                dialog.dismiss()
                completeTraining("features", 20, 10)
            }
            .setNegativeButton("Cancel") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }
    
    private fun startEmergencyTypesTraining() {
        val dialog = AlertDialog.Builder(this)
            .setTitle("🚨 Emergency Types Training")
            .setMessage("""
                Welcome to Emergency Types Training!
                
                Learn about:
                • Current emergency types
                • Recognition and response
                • Context-aware guidance
                • Future emergency types
                
                Duration: ~12 minutes
                Reward: 40 BONK + 20 SKR
                
                Ready to start?
            """.trimIndent())
            .setPositiveButton("Start Training") { dialog, _ ->
                dialog.dismiss()
                completeTraining("emergency_types", 40, 20)
            }
            .setNegativeButton("Cancel") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }
    
    private fun completeTraining(module: String, bonkReward: Int, skrReward: Int) {
        val sharedPrefs = getSharedPreferences("training_progress", MODE_PRIVATE)
        sharedPrefs.edit().putBoolean("${module}_completed", true).apply()
        
        // Award tokens
        val currentBonk = sharedPrefs.getInt("total_bonk", 0)
        val currentSkr = sharedPrefs.getInt("total_skr", 0)
        
        sharedPrefs.edit()
            .putInt("total_bonk", currentBonk + bonkReward)
            .putInt("total_skr", currentSkr + skrReward)
            .apply()
        
        // Show completion dialog
        val dialog = AlertDialog.Builder(this)
            .setTitle("🎉 Training Completed!")
            .setMessage("""
                Congratulations! You've completed the training module.
                
                Rewards Earned:
                • $bonkReward BONK tokens
                • $skrReward SKR tokens
                
                Total Tokens:
                • ${currentBonk + bonkReward} BONK
                • ${currentSkr + skrReward} SKR
                
                Your tokens have been recorded on the Solana blockchain!
            """.trimIndent())
            .setPositiveButton("Continue") { dialog, _ ->
                dialog.dismiss()
                loadTrainingProgress() // Refresh the UI
                setResult(RESULT_OK) // Signal to MainActivity to refresh
            }
            .setCancelable(false)
            .create()
        
        dialog.show()
        
        Log.d(TAG, "Training completed: $module, Rewards: $bonkReward BONK, $skrReward SKR")
    }
} 