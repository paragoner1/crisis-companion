package com.solanasos.emergency

import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import com.solanasos.emergency.databinding.ActivitySettingsBinding
import kotlinx.coroutines.launch

class SettingsActivity : AppCompatActivity() {
    
    private lateinit var binding: ActivitySettingsBinding
    private lateinit var solanaMobile: SolanaMobileIntegration
    
    companion object {
        private const val TAG = "SettingsActivity"
    }
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivitySettingsBinding.inflate(layoutInflater)
        setContentView(binding.root)
        
        solanaMobile = SolanaMobileIntegration(this)
        
        setupUI()
        loadSettings()
    }
    
    private fun setupUI() {
        // Emergency contacts
        binding.btnAddContact.setOnClickListener {
            addEmergencyContact()
        }
        
        // Voice recognition settings
        binding.switchVoiceRecognition.setOnCheckedChangeListener { _, isChecked ->
            updateVoiceRecognitionSetting(isChecked)
        }
        
        // Location sharing settings
        binding.switchLocationSharing.setOnCheckedChangeListener { _, isChecked ->
            updateLocationSharingSetting(isChecked)
        }
        
        // Solana wallet settings
        binding.btnConnectWallet.setOnClickListener {
            connectSolanaWallet()
        }
        
        // Emergency preferences
        binding.btnEmergencyPreferences.setOnClickListener {
            openEmergencyPreferences()
        }
        
        // Back button
        binding.btnBack.setOnClickListener {
            finish()
        }
    }
    
    private fun loadSettings() {
        // Load saved settings
        val voiceEnabled = getSharedPreferences("settings", MODE_PRIVATE)
            .getBoolean("voice_recognition_enabled", true)
        binding.switchVoiceRecognition.isChecked = voiceEnabled
        
        val locationEnabled = getSharedPreferences("settings", MODE_PRIVATE)
            .getBoolean("location_sharing_enabled", true)
        binding.switchLocationSharing.isChecked = locationEnabled
        
        // Load emergency contacts
        loadEmergencyContacts()
        
        // Check Solana wallet connection
        updateWalletStatus()
    }
    
    private fun addEmergencyContact() {
        val contactName = binding.etContactName.text.toString()
        val contactPhone = binding.etContactPhone.text.toString()
        
        if (contactName.isNotEmpty() && contactPhone.isNotEmpty()) {
            // Save emergency contact
            saveEmergencyContact(contactName, contactPhone)
            
            // Clear input fields
            binding.etContactName.text.clear()
            binding.etContactPhone.text.clear()
            
            Toast.makeText(this, "Emergency contact added", Toast.LENGTH_SHORT).show()
            
            // Refresh contacts list
            loadEmergencyContacts()
        } else {
            Toast.makeText(this, "Please enter name and phone number", Toast.LENGTH_SHORT).show()
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
        
        val contactsList = mutableListOf<String>()
        for (i in 0 until contactCount) {
            val name = contacts.getString("name_$i", "")
            val phone = contacts.getString("phone_$i", "")
            if (name.isNotEmpty() && phone.isNotEmpty()) {
                contactsList.add("$name: $phone")
            }
        }
        
        binding.tvEmergencyContacts.text = contactsList.joinToString("\n")
    }
    
    private fun updateVoiceRecognitionSetting(enabled: Boolean) {
        getSharedPreferences("settings", MODE_PRIVATE)
            .edit()
            .putBoolean("voice_recognition_enabled", enabled)
            .apply()
        
        val message = if (enabled) "Voice recognition enabled" else "Voice recognition disabled"
        Toast.makeText(this, message, Toast.LENGTH_SHORT).show()
    }
    
    private fun updateLocationSharingSetting(enabled: Boolean) {
        getSharedPreferences("settings", MODE_PRIVATE)
            .edit()
            .putBoolean("location_sharing_enabled", enabled)
            .apply()
        
        val message = if (enabled) "Location sharing enabled" else "Location sharing disabled"
        Toast.makeText(this, message, Toast.LENGTH_SHORT).show()
    }
    
    private fun connectSolanaWallet() {
        binding.btnConnectWallet.text = "Connecting..."
        
        lifecycleScope.launch {
            val connected = solanaMobile.connectWallet()
            
            if (connected) {
                val walletAddress = solanaMobile.getWalletAddress()
                binding.btnConnectWallet.text = "Connected: ${walletAddress?.take(8)}..."
                Toast.makeText(this@SettingsActivity, "Solana wallet connected!", Toast.LENGTH_SHORT).show()
            } else {
                binding.btnConnectWallet.text = "Connect Solana Wallet"
                Toast.makeText(this@SettingsActivity, "Wallet connection failed", Toast.LENGTH_SHORT).show()
            }
        }
    }
    
    private fun updateWalletStatus() {
        if (solanaMobile.isWalletConnected()) {
            val walletAddress = solanaMobile.getWalletAddress()
            binding.btnConnectWallet.text = "Connected: ${walletAddress?.take(8)}..."
        } else {
            binding.btnConnectWallet.text = "Connect Solana Wallet"
        }
    }
    
    private fun openEmergencyPreferences() {
        // Open emergency preferences dialog
        Toast.makeText(this, "Emergency preferences", Toast.LENGTH_SHORT).show()
    }
} 