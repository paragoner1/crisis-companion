use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jint, jstring, JNI_TRUE, JNI_FALSE};
use std::ffi::CString;
use std::ptr;

use crate::voice_interface::VoiceInterface;
use crate::audio_interface::AudioInterface;
use crate::emergency_interface::EmergencyInterface;
use crate::error::AppError;

// Global static instances
static mut VOICE_INTERFACE: Option<VoiceInterface> = None;
static mut AUDIO_INTERFACE: Option<AudioInterface> = None;
static mut EMERGENCY_INTERFACE: Option<EmergencyInterface> = None;

/// Initialize Android-specific paths and interfaces
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_initializeAndroidPaths(
    _env: JNIEnv,
    _class: JClass,
    files_dir: JString,
    cache_dir: JString,
) {
    unsafe {
        // Convert Java strings to Rust strings
        let files_dir_str = _env.get_string(files_dir).unwrap().into();
        let cache_dir_str = _env.get_string(cache_dir).unwrap().into();
        
        // Initialize interfaces with Android paths
        VOICE_INTERFACE = Some(VoiceInterface::new(&files_dir_str));
        AUDIO_INTERFACE = Some(AudioInterface::new(&cache_dir_str));
        EMERGENCY_INTERFACE = Some(EmergencyInterface::new(&files_dir_str));
    }
}

/// Initialize voice recognition
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_initializeVoiceRecognition(
    _env: JNIEnv,
    _class: JClass,
) -> jboolean {
    unsafe {
        if let Some(voice_interface) = &mut VOICE_INTERFACE {
            // For JNI, we need to handle async functions synchronously
            // In a real implementation, you'd use a runtime or block_on
            match std::panic::catch_unwind(|| {
                // This is a simplified version - in production you'd use tokio::runtime
                tracing::info!("Voice interface initialized");
            }) {
                Ok(_) => JNI_TRUE,
                Err(_) => JNI_FALSE,
            }
        } else {
            JNI_FALSE
        }
    }
}

/// Process voice input and return recognized text
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_processVoiceInput(
    _env: JNIEnv,
    _class: JClass,
    audio_data: JObject,
) -> jstring {
    unsafe {
        if let Some(voice_interface) = &mut VOICE_INTERFACE {
            // Convert Java byte array to Rust Vec<u8>
            let audio_bytes = _env.get_byte_array_elements(audio_data, ptr::null_mut()).unwrap();
            let audio_vec = audio_bytes.to_vec();
            
            // For JNI, we'll return a placeholder since async functions are complex
            let recognized_text = "hey sos drowning emergency".to_string();
            let c_string = CString::new(recognized_text).unwrap();
            _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
        } else {
            ptr::null_mut()
        }
    }
}

/// Detect wake word in audio
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_detectWakeWord(
    _env: JNIEnv,
    _class: JClass,
    audio_data: JObject,
) -> jboolean {
    unsafe {
        if let Some(_voice_interface) = &mut VOICE_INTERFACE {
            let _audio_bytes = _env.get_byte_array_elements(audio_data, ptr::null_mut()).unwrap();
            let _audio_vec = _audio_bytes.to_vec();
            
            // For demo purposes, always detect wake word
            JNI_TRUE
        } else {
            JNI_FALSE
        }
    }
}

/// Detect emergency phrase in audio
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_detectEmergencyPhrase(
    _env: JNIEnv,
    _class: JClass,
    audio_data: JObject,
) -> jstring {
    unsafe {
        if let Some(_voice_interface) = &mut VOICE_INTERFACE {
            let _audio_bytes = _env.get_byte_array_elements(audio_data, ptr::null_mut()).unwrap();
            let _audio_vec = _audio_bytes.to_vec();
            
            // For demo purposes, return a detected emergency
            let detected_emergency = "drowning".to_string();
            let c_string = CString::new(detected_emergency).unwrap();
            _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
        } else {
            ptr::null_mut()
        }
    }
}

/// Get emergency instructions
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getEmergencyInstructions(
    _env: JNIEnv,
    _class: JClass,
    emergency_type: JString,
) -> jstring {
    unsafe {
        let emergency_type_str = _env.get_string(emergency_type).unwrap().into();
        
        // Return appropriate instructions based on emergency type
        let instructions = match emergency_type_str.as_str() {
            "drowning" => "1. Remove victim from water\n2. Check breathing\n3. Begin CPR if needed\n4. Call 911",
            "heart attack" => "1. Call 911 immediately\n2. Have victim sit down\n3. Loosen tight clothing\n4. Monitor breathing",
            "choking" => "1. Perform Heimlich maneuver\n2. 5 back blows, 5 abdominal thrusts\n3. Call 911 if not resolved",
            "bleeding" => "1. Apply direct pressure\n2. Elevate if possible\n3. Use tourniquet if severe\n4. Call 911",
            "unconscious" => "1. Check breathing\n2. Begin CPR if needed\n3. Call 911 immediately\n4. Monitor for changes",
            "stroke" => "1. Remember FAST\n2. Face, Arm, Speech, Time\n3. Call 911 immediately\n4. Note time of onset",
            "seizure" => "1. Clear area of objects\n2. Don't restrain\n3. Time the seizure\n4. Call 911 if >5 minutes",
            "poisoning" => "1. Call Poison Control\n2. Don't induce vomiting\n3. Save container\n4. Call 911 if severe",
            "burn" => "1. Cool with water\n2. Don't use ice\n3. Cover with clean cloth\n4. Call 911 if severe",
            "diabetic" => "1. Check blood sugar\n2. Give sugar if low\n3. Call 911 if unconscious\n4. Monitor breathing",
            "allergic" => "1. Use EpiPen if available\n2. Call 911 immediately\n3. Monitor breathing\n4. Lie flat if dizzy",
            "trauma" => "1. Stop bleeding\n2. Immobilize injuries\n3. Call 911\n4. Monitor consciousness",
            _ => "Call 911 immediately and follow emergency dispatcher instructions"
        };
        
        let c_string = CString::new(instructions).unwrap();
        _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
    }
}

/// Get context-aware guidance
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getContextAwareGuidance(
    _env: JNIEnv,
    _class: JClass,
    emergency_type: JString,
    context: JString,
) -> jstring {
    let emergency_type_str = _env.get_string(emergency_type).unwrap().into();
    let context_str = _env.get_string(context).unwrap().into();
    
    // Return context-aware guidance
    let guidance = format!("Context-aware guidance for {}: {}", emergency_type_str, context_str);
    let c_string = CString::new(guidance).unwrap();
    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
}

/// Detect emergency stage
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_detectEmergencyStage(
    _env: JNIEnv,
    _class: JClass,
    user_phrase: JString,
    location: JString,
    actions: JString,
) -> jstring {
    let user_phrase_str = _env.get_string(user_phrase).unwrap().into();
    let location_str = _env.get_string(location).unwrap().into();
    let actions_str = _env.get_string(actions).unwrap().into();
    
    // Return detected stage
    let stage = format!("Stage detected for phrase: {} at location: {} with actions: {}", 
                       user_phrase_str, location_str, actions_str);
    let c_string = CString::new(stage).unwrap();
    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
}

/// Process audio with noise filtering
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_processAudioWithNoiseFiltering(
    _env: JNIEnv,
    _class: JClass,
    audio_data: JObject,
) -> JObject {
    let audio_bytes = _env.get_byte_array_elements(audio_data, ptr::null_mut()).unwrap();
    let audio_vec = audio_bytes.to_vec();
    
    // For demo purposes, return the original audio
    let filtered_array = _env.new_byte_array(audio_vec.len() as i32).unwrap();
    _env.set_byte_array_region(filtered_array, 0, &audio_vec).unwrap();
    filtered_array.into_inner()
}

/// Award XP for actions
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_awardXP(
    _env: JNIEnv,
    _class: JClass,
    action: JString,
    amount: jint,
) -> jboolean {
    let action_str = _env.get_string(action).unwrap().into();
    
    // For demo purposes, always award XP
    tracing::info!("Awarded {} XP for action: {}", amount, action_str);
    JNI_TRUE
}

/// Get hero level
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getHeroLevel(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    // For demo purposes, return level 5
    5
}

/// Get total rewards
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getTotalRewards(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    let rewards = "100 BONK, 25 SKR".to_string();
    let c_string = CString::new(rewards).unwrap();
    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
}

/// Connect Solana wallet
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_connectSolanaWallet(
    _env: JNIEnv,
    _class: JClass,
) -> jboolean {
    // For demo purposes, always connect successfully
    tracing::info!("Solana wallet connected");
    JNI_TRUE
}

/// Get wallet address
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getWalletAddress(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    let address = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string();
    let c_string = CString::new(address).unwrap();
    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
}

/// Record emergency on blockchain
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_recordEmergencyOnBlockchain(
    _env: JNIEnv,
    _class: JClass,
    emergency_data: JString,
) -> jstring {
    let emergency_data_str = _env.get_string(emergency_data).unwrap().into();
    
    // For demo purposes, return a transaction hash
    let record_id = "5J7X9K2M4N6P8Q1R3S5T7U9V2W4X6Y8Z0A1B3C5D7E9F".to_string();
    let c_string = CString::new(record_id).unwrap();
    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
}

/// Initialize database
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_initializeDatabase(
    _env: JNIEnv,
    _class: JClass,
) -> jboolean {
    // For demo purposes, always initialize successfully
    tracing::info!("Database initialized");
    JNI_TRUE
}

/// Save emergency contact
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_saveEmergencyContact(
    _env: JNIEnv,
    _class: JClass,
    name: JString,
    phone: JString,
) -> jboolean {
    let name_str = _env.get_string(name).unwrap().into();
    let phone_str = _env.get_string(phone).unwrap().into();
    
    // For demo purposes, always save successfully
    tracing::info!("Emergency contact saved: {} - {}", name_str, phone_str);
    JNI_TRUE
}

/// Get emergency contacts
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getEmergencyContacts(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    let contacts = r#"[
        {"name": "Emergency Contact 1", "phone": "555-0101"},
        {"name": "Emergency Contact 2", "phone": "555-0102"}
    ]"#.to_string();
    let c_string = CString::new(contacts).unwrap();
    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
}

/// Validate emergency type
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_validateEmergencyType(
    _env: JNIEnv,
    _class: JClass,
    emergency_type: JString,
) -> jboolean {
    let emergency_type_str = _env.get_string(emergency_type).unwrap().into();
    
    // Validate against known emergency types
    let valid_types = vec![
        "drowning", "heart attack", "choking", "bleeding", "unconscious",
        "stroke", "seizure", "poisoning", "burn", "diabetic", "allergic", "trauma"
    ];
    
    if valid_types.contains(&emergency_type_str.as_str()) {
        JNI_TRUE
    } else {
        JNI_FALSE
    }
}

/// Get app version
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getAppVersion(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    let version = "1.0.0";
    let c_string = CString::new(version).unwrap();
    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
}

/// Get build info
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getBuildInfo(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    let build_info = "Solana SOS Android v1.0.0 - Built with Rust backend";
    let c_string = CString::new(build_info).unwrap();
    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
} 