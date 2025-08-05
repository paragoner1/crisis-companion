package com.solanasos.emergency

import android.Manifest
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import com.solanasos.emergency.databinding.ActivityMainBinding
import android.util.Log

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
        requestPermissions()
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
            showDemoDialog()
        }
        
        // Emergency button
        binding.btnEmergency.setOnClickListener {
            showEmergencyDialog()
        }
        
        // Training Center button
        binding.btnTraining.setOnClickListener {
            Toast.makeText(this, "Training Center - Coming Soon!", Toast.LENGTH_SHORT).show()
        }
        
        // Cross-App Challenges button
        binding.btnChallenges.setOnClickListener {
            Toast.makeText(this, "Cross-App Challenges - Coming Soon!", Toast.LENGTH_SHORT).show()
        }
        
        // Settings button
        binding.btnSettings.setOnClickListener {
            Toast.makeText(this, "Settings - Coming Soon!", Toast.LENGTH_SHORT).show()
        }
    }
    
    private fun showDemoDialog() {
        val demoMessage = """
            🎬 SOLANA SOS DEMO MODE
            
            Step 1: Voice Recognition
            • Say "Hey SOS" to activate
            • App detects wake word instantly
            
            Step 2: Emergency Response
            • Say emergency type (e.g., "drowning")
            • App provides life-saving instructions
            • Automatic 911 call with location sharing
            
            Step 3: Safety Features
            • Real-time location tracking
            • Emergency contact notifications
            • Blockchain-secured records
            
            Step 4: SOS Hero Rewards
            • BONK/SKR token rewards
            • Achievement system
            • Community engagement
            
            DEMO: This simulation shows the complete emergency response flow
        """.trimIndent()
        
        androidx.appcompat.app.AlertDialog.Builder(this)
            .setTitle("🎬 Demo Mode")
            .setMessage(demoMessage)
            .setPositiveButton("Start Demo") { dialog, _ ->
                Toast.makeText(this, "Demo started! Say 'Hey SOS' to activate", Toast.LENGTH_LONG).show()
                dialog.dismiss()
            }
            .setNegativeButton("Cancel") { dialog, _ ->
                dialog.dismiss()
            }
            .show()
    }
    
    private fun showEmergencyDialog() {
        val emergencyMessage = """
            🚨 EMERGENCY ACTIVATION
            
            Voice Commands:
            • "Hey SOS" - Wake word
            • "drowning" - Drowning emergency
            • "heart attack" - Cardiac emergency
            • "choking" - Choking emergency
            • "bleeding" - Bleeding emergency
            
            Features:
            • Instant voice recognition
            • Life-saving instructions
            • Automatic 911 calling
            • Location sharing
            • Offline functionality
            
            DEMO: Press and say "Hey SOS" followed by emergency type
        """.trimIndent()
        
        androidx.appcompat.app.AlertDialog.Builder(this)
            .setTitle("�� Emergency Mode")
            .setMessage(emergencyMessage)
            .setPositiveButton("Activate") { dialog, _ ->
                Toast.makeText(this, "Emergency mode activated! Say 'Hey SOS'", Toast.LENGTH_LONG).show()
                dialog.dismiss()
            }
            .setNegativeButton("Cancel") { dialog, _ ->
                dialog.dismiss()
            }
            .show()
    }
    
    private fun requestPermissions() {
        val permissions = arrayOf(
            Manifest.permission.RECORD_AUDIO,
            Manifest.permission.ACCESS_FINE_LOCATION,
            Manifest.permission.CALL_PHONE
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
                Toast.makeText(this, "All permissions granted!", Toast.LENGTH_SHORT).show()
            } else {
                Toast.makeText(this, "Some permissions denied", Toast.LENGTH_SHORT).show()
            }
        }
    }
}
