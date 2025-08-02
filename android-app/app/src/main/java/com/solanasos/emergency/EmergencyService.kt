package com.solanasos.emergency

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Intent
import android.location.Location
import android.location.LocationManager
import android.os.Build
import android.os.IBinder
import android.util.Log
import androidx.core.app.NotificationCompat
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class EmergencyService : Service() {
    
    companion object {
        private const val TAG = "EmergencyService"
        private const val NOTIFICATION_ID = 1
        private const val CHANNEL_ID = "emergency_channel"
    }
    
    private lateinit var solanaIntegration: SolanaIntegration
    private lateinit var locationManager: LocationManager
    private var currentEmergency: String? = null
    
    override fun onCreate() {
        super.onCreate()
        solanaIntegration = SolanaIntegration(this)
        locationManager = getSystemService(LOCATION_SERVICE) as LocationManager
        
        createNotificationChannel()
    }
    
    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.d(TAG, "Emergency service started")
        
        // Get emergency type from intent
        currentEmergency = intent?.getStringExtra("emergency_type")
        
        if (currentEmergency != null) {
            Log.d(TAG, "Processing emergency: $currentEmergency")
            
            // Start foreground service with notification
            startForeground(NOTIFICATION_ID, createEmergencyNotification())
            
            // Process emergency
            processEmergency()
            
            return START_STICKY
        }
        
        return START_NOT_STICKY
    }
    
    private fun processEmergency() {
        CoroutineScope(Dispatchers.IO).launch {
            try {
                // 1. Get current location
                val location = getCurrentLocation()
                
                // 2. Call 911 (simulated)
                call911(location)
                
                // 3. Share location with emergency contacts
                shareLocationWithContacts(location)
                
                // 4. Record on Solana blockchain
                recordEmergencyOnBlockchain(location)
                
                // 5. Award emergency response tokens
                awardEmergencyResponseTokens()
                
                Log.d(TAG, "Emergency processing completed")
                
            } catch (e: Exception) {
                Log.e(TAG, "Error processing emergency", e)
            }
        }
    }
    
    private suspend fun getCurrentLocation(): Location? = withContext(Dispatchers.IO) {
        try {
            Log.d(TAG, "Getting current location...")
            
            // This would use actual GPS location
            // For demo purposes, return a sample location
            val location = Location("gps")
            location.latitude = 37.7749 // San Francisco
            location.longitude = -122.4194
            location.accuracy = 10.0f
            
            Log.d(TAG, "Location: ${location.latitude}, ${location.longitude}")
            return@withContext location
            
        } catch (e: Exception) {
            Log.e(TAG, "Error getting location", e)
            return@withContext null
        }
    }
    
    private suspend fun call911(location: Location?) = withContext(Dispatchers.IO) {
        Log.d(TAG, "Calling 911...")
        
        // This would make an actual emergency call
        // For demo purposes, just log the action
        
        val locationString = if (location != null) {
            "Location: ${location.latitude}, ${location.longitude}"
        } else {
            "Location unavailable"
        }
        
        Log.d(TAG, "911 call initiated for $currentEmergency at $locationString")
        
        // Simulate call duration
        Thread.sleep(2000)
        Log.d(TAG, "911 call completed")
    }
    
    private suspend fun shareLocationWithContacts(location: Location?) = withContext(Dispatchers.IO) {
        Log.d(TAG, "Sharing location with emergency contacts...")
        
        // This would send SMS to emergency contacts
        // For demo purposes, just log the action
        
        if (location != null) {
            val locationUrl = "https://maps.google.com/?q=${location.latitude},${location.longitude}"
            Log.d(TAG, "Location shared: $locationUrl")
        }
        
        // Simulate SMS sending
        Thread.sleep(1000)
        Log.d(TAG, "Location shared with emergency contacts")
    }
    
    private suspend fun recordEmergencyOnBlockchain(location: Location?) = withContext(Dispatchers.IO) {
        Log.d(TAG, "Recording emergency on Solana blockchain...")
        
        val emergencyData = EmergencyData(
            emergencyType = currentEmergency ?: "Unknown",
            timestamp = System.currentTimeMillis(),
            location = if (location != null) "${location.latitude},${location.longitude}" else null,
            actions = listOf("911_called", "location_shared", "emergency_service_activated"),
            outcome = "In Progress"
        )
        
        val recordId = solanaIntegration.recordEmergencyOnBlockchain(emergencyData)
        
        if (recordId != null) {
            Log.d(TAG, "Emergency recorded on blockchain: $recordId")
        } else {
            Log.w(TAG, "Failed to record emergency on blockchain")
        }
    }
    
    private suspend fun awardEmergencyResponseTokens() = withContext(Dispatchers.IO) {
        Log.d(TAG, "Awarding emergency response tokens...")
        
        val emergencyType = currentEmergency ?: "unknown"
        
        // Award tokens for different emergency response actions
        val actions = listOf(
            "emergency_service_activated",
            "location_shared",
            "911_called",
            "blockchain_recorded"
        )
        
        for (action in actions) {
            val rewarded = solanaIntegration.awardEmergencyRewards(emergencyType, action)
            if (rewarded) {
                Log.d(TAG, "Awarded tokens for $action")
            }
        }
        
        val totalRewards = solanaIntegration.getTotalRewards()
        Log.d(TAG, "Total emergency rewards: $totalRewards")
    }
    
    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel(
                CHANNEL_ID,
                "Emergency Service",
                NotificationManager.IMPORTANCE_HIGH
            ).apply {
                description = "Emergency response service"
                enableLights(true)
                enableVibration(true)
            }
            
            val notificationManager = getSystemService(NotificationManager::class.java)
            notificationManager.createNotificationChannel(channel)
        }
    }
    
    private fun createEmergencyNotification(): Notification {
        val emergencyType = currentEmergency ?: "Emergency"
        
        return NotificationCompat.Builder(this, CHANNEL_ID)
            .setContentTitle("🚨 Solana SOS Emergency")
            .setContentText("Processing $emergencyType emergency")
            .setSmallIcon(android.R.drawable.ic_dialog_alert)
            .setPriority(NotificationCompat.PRIORITY_HIGH)
            .setCategory(NotificationCompat.CATEGORY_ALARM)
            .setOngoing(true)
            .build()
    }
    
    override fun onBind(intent: Intent?): IBinder? = null
    
    override fun onDestroy() {
        super.onDestroy()
        Log.d(TAG, "Emergency service destroyed")
        
        // Disconnect from Solana wallet
        solanaIntegration.disconnect()
    }
} 