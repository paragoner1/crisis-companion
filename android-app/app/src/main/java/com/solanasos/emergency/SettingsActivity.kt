package com.solanasos.emergency

import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import com.solanasos.emergency.databinding.ActivitySettingsBinding
import kotlinx.coroutines.launch
import androidx.core.content.ContextCompat
import android.util.Log
import android.content.Context

class SettingsActivity : AppCompatActivity() {
    
    private lateinit var binding: ActivitySettingsBinding
    private lateinit var solanaMobile: SolanaMobileIntegration
    
    companion object {
        private const val TAG = "SettingsActivity"
    }
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        // Set status bar color to match the deep navy background
        window.statusBarColor = ContextCompat.getColor(this, R.color.deep_navy)
        
        binding = ActivitySettingsBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        solanaMobile = SolanaMobileIntegration(this)
        
        setupUI()
        loadSettings()
    }
    
    private fun setupUI() {
        // Emergency Contacts
        binding.btnAddContact.setOnClickListener {
            addEmergencyContact()
        }
        
        binding.btnRemoveContact.setOnClickListener {
            removeEmergencyContact()
        }
        
        // App Settings
        binding.switchVoiceRecognition.setOnCheckedChangeListener { _, isChecked ->
            updateVoiceRecognitionSetting(isChecked)
        }
        
        binding.switchLocationSharing.setOnCheckedChangeListener { _, isChecked ->
            updateLocationSharingSetting(isChecked)
        }
        
        binding.switchAutoDial911.setOnCheckedChangeListener { _, isChecked ->
            updateAutoDial911Setting(isChecked)
        }
        
        binding.switchSilentSOSPower.setOnCheckedChangeListener { _, isChecked ->
            updateSilentSOSPowerSetting(isChecked)
        }
        
        binding.switchSilentSOSVolume.setOnCheckedChangeListener { _, isChecked ->
            updateSilentSOSVolumeSetting(isChecked)
        }
        
        binding.switchCrashDetection.setOnCheckedChangeListener { _, isChecked ->
            updateCrashDetectionSetting(isChecked)
        }
        
        // Solana Integration
        binding.btnConnectWallet.setOnClickListener {
            connectSolanaWallet()
        }
        
        // Back button
        binding.btnBack.setOnClickListener {
            finish()
        }
    }
    
    private fun loadSettings() {
        val sharedPrefs = getSharedPreferences("settings", Context.MODE_PRIVATE)
        
        // Load toggle states
        binding.switchVoiceRecognition.isChecked = sharedPrefs.getBoolean("voice_recognition_enabled", true)
        binding.switchLocationSharing.isChecked = sharedPrefs.getBoolean("location_sharing_enabled", true)
        binding.switchAutoDial911.isChecked = sharedPrefs.getBoolean("auto_dial_911_enabled", false)
        binding.switchSilentSOSPower.isChecked = sharedPrefs.getBoolean("silent_sos_power_enabled", false)
        binding.switchSilentSOSVolume.isChecked = sharedPrefs.getBoolean("silent_sos_volume_enabled", false)
        binding.switchCrashDetection.isChecked = sharedPrefs.getBoolean("crash_detection_enabled", false)
        
        // Update toggle colors
        updateToggleColors()
        
        // Load emergency contacts
        loadEmergencyContacts()
        
        // Update wallet status
        updateWalletStatus()
    }
    
    private fun addEmergencyContact() {
        val contactName = binding.etContactName.text.toString()
        val contactPhone = binding.etContactPhone.text.toString()
        
        if (contactName.isNotEmpty() && contactPhone.isNotEmpty()) {
            // Format phone number
            val formattedPhone = formatPhoneNumber(contactPhone)
            
            // Save emergency contact
            saveEmergencyContact(contactName, formattedPhone)
            
            // Clear input fields
            binding.etContactName.text.clear()
            binding.etContactPhone.text.clear()
            
            Toast.makeText(this, "✅ Emergency contact added! Earned 25 SKR for safety setup!", Toast.LENGTH_LONG).show()
            
            // Refresh contacts list
            loadEmergencyContacts()
        } else {
            Toast.makeText(this, "Please enter name and phone number", Toast.LENGTH_SHORT).show()
        }
    }
    
    private fun formatPhoneNumber(phone: String): String {
        // Remove all non-digit characters
        val digits = phone.replace(Regex("[^0-9]"), "")
        
        return when {
            digits.length == 10 -> "(${digits.substring(0, 3)})-${digits.substring(3, 6)}-${digits.substring(6)}"
            digits.length == 11 && digits.startsWith("1") -> "(${digits.substring(1, 4)})-${digits.substring(4, 7)}-${digits.substring(7)}"
            else -> phone // Return original if can't format
        }
    }
    
    private fun saveEmergencyContact(name: String, phone: String) {
        val contacts = getSharedPreferences("emergency_contacts", MODE_PRIVATE)
        val contactCount = contacts.getInt("count", 0)
        
        contacts.edit()
            .putString("name_$contactCount", name)
            .putString("phone_$contactCount", phone)
            .putInt("count", contactCount + 1)
            .apply()
    }
    
    private fun loadEmergencyContacts() {
        val contacts = getSharedPreferences("emergency_contacts", MODE_PRIVATE)
        val contactCount = contacts.getInt("count", 0)
        
        if (contactCount > 0) {
            val contactText = StringBuilder()
            for (i in 0 until contactCount) {
                val name = contacts.getString("name_$i", "") ?: ""
                val phone = contacts.getString("phone_$i", "") ?: ""
                if (name.isNotEmpty() && phone.isNotEmpty()) {
                    contactText.append("👤 ${name}\n")
                    contactText.append("📞 ${phone}\n")
                    if (i < contactCount - 1) {
                        contactText.append("───────────────\n")
                    }
                }
            }
            binding.tvEmergencyContacts.text = contactText.toString()
        } else {
            binding.tvEmergencyContacts.text = "No emergency contacts added"
        }
    }
    
    private fun updateToggleColors() {
        val sharedPrefs = getSharedPreferences("settings", Context.MODE_PRIVATE)
        
        // Voice Recognition
        val voiceEnabled = sharedPrefs.getBoolean("voice_recognition_enabled", true)
        binding.switchVoiceRecognition.thumbTintList = ContextCompat.getColorStateList(this, if (voiceEnabled) R.color.soft_green else R.color.text_secondary)
        binding.switchVoiceRecognition.trackTintList = ContextCompat.getColorStateList(this, if (voiceEnabled) R.color.soft_green else R.color.text_secondary)
        
        // Location Sharing
        val locationEnabled = sharedPrefs.getBoolean("location_sharing_enabled", true)
        binding.switchLocationSharing.thumbTintList = ContextCompat.getColorStateList(this, if (locationEnabled) R.color.soft_green else R.color.text_secondary)
        binding.switchLocationSharing.trackTintList = ContextCompat.getColorStateList(this, if (locationEnabled) R.color.soft_green else R.color.text_secondary)
        
        // Auto-dial 911
        val autoDialEnabled = sharedPrefs.getBoolean("auto_dial_911_enabled", false)
        binding.switchAutoDial911.thumbTintList = ContextCompat.getColorStateList(this, if (autoDialEnabled) R.color.soft_green else R.color.text_secondary)
        binding.switchAutoDial911.trackTintList = ContextCompat.getColorStateList(this, if (autoDialEnabled) R.color.soft_green else R.color.text_secondary)
        
        // Silent SOS Power
        val silentSOSPowerEnabled = sharedPrefs.getBoolean("silent_sos_power_enabled", false)
        binding.switchSilentSOSPower.thumbTintList = ContextCompat.getColorStateList(this, if (silentSOSPowerEnabled) R.color.soft_green else R.color.text_secondary)
        binding.switchSilentSOSPower.trackTintList = ContextCompat.getColorStateList(this, if (silentSOSPowerEnabled) R.color.soft_green else R.color.text_secondary)
        
        // Silent SOS Volume
        val silentSOSVolumeEnabled = sharedPrefs.getBoolean("silent_sos_volume_enabled", false)
        binding.switchSilentSOSVolume.thumbTintList = ContextCompat.getColorStateList(this, if (silentSOSVolumeEnabled) R.color.soft_green else R.color.text_secondary)
        binding.switchSilentSOSVolume.trackTintList = ContextCompat.getColorStateList(this, if (silentSOSVolumeEnabled) R.color.soft_green else R.color.text_secondary)
        
        // Crash Detection
        val crashDetectionEnabled = sharedPrefs.getBoolean("crash_detection_enabled", false)
        binding.switchCrashDetection.thumbTintList = ContextCompat.getColorStateList(this, if (crashDetectionEnabled) R.color.soft_green else R.color.text_secondary)
        binding.switchCrashDetection.trackTintList = ContextCompat.getColorStateList(this, if (crashDetectionEnabled) R.color.soft_green else R.color.text_secondary)
    }

    private fun updateVoiceRecognitionSetting(enabled: Boolean) {
        getSharedPreferences("settings", Context.MODE_PRIVATE)
            .edit()
            .putBoolean("voice_recognition_enabled", enabled)
            .apply()
        
        updateToggleColors()
        setResult(RESULT_OK)
        Log.d(TAG, "Voice recognition setting updated: $enabled")
    }
    
    private fun updateLocationSharingSetting(enabled: Boolean) {
        getSharedPreferences("settings", Context.MODE_PRIVATE)
            .edit()
            .putBoolean("location_sharing_enabled", enabled)
            .apply()
        
        updateToggleColors()
        setResult(RESULT_OK)
        Log.d(TAG, "Location sharing setting updated: $enabled")
    }
    
    private fun updateAutoDial911Setting(enabled: Boolean) {
        getSharedPreferences("settings", Context.MODE_PRIVATE)
            .edit()
            .putBoolean("auto_dial_911_enabled", enabled)
            .apply()
        
        updateToggleColors()
        setResult(RESULT_OK)
        Log.d(TAG, "Auto-dial 911 setting updated: $enabled")
    }
    
    private fun updateSilentSOSPowerSetting(enabled: Boolean) {
        getSharedPreferences("settings", Context.MODE_PRIVATE)
            .edit()
            .putBoolean("silent_sos_power_enabled", enabled)
            .apply()
        
        updateToggleColors()
        setResult(RESULT_OK)
        Log.d(TAG, "Silent SOS Power setting updated: $enabled")
    }
    
    private fun updateSilentSOSVolumeSetting(enabled: Boolean) {
        getSharedPreferences("settings", Context.MODE_PRIVATE)
            .edit()
            .putBoolean("silent_sos_volume_enabled", enabled)
            .apply()
        
        updateToggleColors()
        setResult(RESULT_OK)
        Log.d(TAG, "Silent SOS Volume setting updated: $enabled")
    }
    
    private fun updateCrashDetectionSetting(enabled: Boolean) {
        getSharedPreferences("settings", Context.MODE_PRIVATE)
            .edit()
            .putBoolean("crash_detection_enabled", enabled)
            .apply()
        
        updateToggleColors()
        setResult(RESULT_OK)
        Log.d(TAG, "Crash detection setting updated: $enabled")
    }
    
    private fun removeEmergencyContact() {
        val contacts = getSharedPreferences("emergency_contacts", MODE_PRIVATE)
        val contactCount = contacts.getInt("count", 0)
        
        if (contactCount > 0) {
            // Remove the last contact
            contacts.edit()
                .putInt("count", contactCount - 1)
                .remove("name_${contactCount - 1}")
                .remove("phone_${contactCount - 1}")
                .apply()
            
            loadEmergencyContacts()
            Toast.makeText(this, "Emergency contact removed", Toast.LENGTH_SHORT).show()
        } else {
            Toast.makeText(this, "No contacts to remove", Toast.LENGTH_SHORT).show()
        }
    }
    
    private fun connectSolanaWallet() {
        val settings = getSharedPreferences("settings", MODE_PRIVATE)
        val isCurrentlyConnected = settings.getBoolean("wallet_connected", false)
        
        if (isCurrentlyConnected) {
            // Disconnect wallet
            settings.edit()
                .putBoolean("wallet_connected", false)
                .remove("wallet_address")
                .apply()
            
            binding.btnConnectWallet.text = "Connect Solana Wallet"
            binding.btnConnectWallet.setTextColor(resources.getColor(R.color.text_primary, null))
            Toast.makeText(this@SettingsActivity, "Wallet disconnected", Toast.LENGTH_SHORT).show()
            setResult(RESULT_OK)
        } else {
            // Connect wallet
            binding.btnConnectWallet.text = "Connecting..."
            binding.btnConnectWallet.setTextColor(resources.getColor(R.color.text_primary, null))
            
            lifecycleScope.launch {
                val connected = solanaMobile.connectWallet()
                
                if (connected) {
                    val walletAddress = solanaMobile.getWalletAddress()
                    val displayAddress = walletAddress?.take(8)?.let { "${it}..." } ?: "Unknown"
                    binding.btnConnectWallet.text = "Connected (${displayAddress})"
                    binding.btnConnectWallet.setTextColor(resources.getColor(R.color.soft_green, null))
                    Toast.makeText(this@SettingsActivity, "✅ Wallet Connected! Earned 100 BONK for setup!", Toast.LENGTH_LONG).show()
                    
                    // Save wallet connection state
                    settings.edit()
                        .putBoolean("wallet_connected", true)
                        .putString("wallet_address", walletAddress)
                        .apply()
                    
                    // Notify MainActivity of wallet status change
                    setResult(RESULT_OK)
                } else {
                    binding.btnConnectWallet.text = "Connect Solana Wallet"
                    binding.btnConnectWallet.setTextColor(resources.getColor(R.color.text_primary, null))
                    Toast.makeText(this@SettingsActivity, "❌ Wallet connection failed. Please try again.", Toast.LENGTH_SHORT).show()
                }
            }
        }
    }
    
    private fun updateWalletStatus() {
        val settings = getSharedPreferences("settings", MODE_PRIVATE)
        val isConnected = settings.getBoolean("wallet_connected", false)
        
        if (isConnected) {
            val walletAddress = settings.getString("wallet_address", null)
            val displayAddress = walletAddress?.take(8)?.let { "${it}..." } ?: "Unknown"
            binding.btnConnectWallet.text = "Connected (${displayAddress})"
            binding.btnConnectWallet.setTextColor(resources.getColor(R.color.soft_green, null))
        } else {
            binding.btnConnectWallet.text = "Connect Solana Wallet"
            binding.btnConnectWallet.setTextColor(resources.getColor(R.color.text_primary, null))
        }
    }
    
    private fun openEmergencyPreferences() {
        // Create emergency preferences dialog
        val dialog = AlertDialog.Builder(this)
            .setTitle("🚨 Emergency Preferences")
            .setMessage("Configure your emergency response settings:")
            .setPositiveButton("Auto-dial 911") { _, _ ->
                getSharedPreferences("settings", MODE_PRIVATE)
                    .edit()
                    .putBoolean("auto_dial_911", true)
                    .apply()
                Toast.makeText(this, "Auto-dial 911 enabled", Toast.LENGTH_SHORT).show()
            }
            .setNegativeButton("Manual dial") { _, _ ->
                getSharedPreferences("settings", MODE_PRIVATE)
                    .edit()
                    .putBoolean("auto_dial_911", false)
                    .apply()
                Toast.makeText(this, "Manual dial enabled", Toast.LENGTH_SHORT).show()
            }
            .setNeutralButton("Cancel") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }
    
    private fun openSilentSOSSettings() {
        // Create Silent SOS settings dialog
        val dialog = AlertDialog.Builder(this)
            .setTitle("🔇 Silent SOS Settings")
            .setMessage("Configure how to activate Silent SOS without opening the app:")
            .setPositiveButton("Hold Power Button (3s)") { _, _ ->
                getSharedPreferences("settings", MODE_PRIVATE)
                    .edit()
                    .putString("silent_sos_method", "hold_button")
                    .putInt("silent_sos_duration", 3)
                    .apply()
                Toast.makeText(this, "Silent SOS: Hold button for 3 seconds", Toast.LENGTH_SHORT).show()
            }
            .setNegativeButton("Press Power Button (5x)") { _, _ ->
                getSharedPreferences("settings", MODE_PRIVATE)
                    .edit()
                    .putString("silent_sos_method", "power_button")
                    .putInt("silent_sos_presses", 5)
                    .apply()
                Toast.makeText(this, "Silent SOS: Press power button 5 times", Toast.LENGTH_SHORT).show()
            }
            .setNeutralButton("Cancel") { dialog, _ ->
                dialog.dismiss()
            }
            .create()
        
        dialog.show()
    }
} 