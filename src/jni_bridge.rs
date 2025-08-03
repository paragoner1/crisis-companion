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
            match voice_interface.initialize() {
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
            
            match voice_interface.process_audio(&audio_vec) {
                Ok(text) => {
                    let c_string = CString::new(text).unwrap();
                    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
                }
                Err(_) => ptr::null_mut(),
            }
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
        if let Some(voice_interface) = &mut VOICE_INTERFACE {
            let audio_bytes = _env.get_byte_array_elements(audio_data, ptr::null_mut()).unwrap();
            let audio_vec = audio_bytes.to_vec();
            
            match voice_interface.detect_wake_word(&audio_vec) {
                Ok(detected) => if detected { JNI_TRUE } else { JNI_FALSE },
                Err(_) => JNI_FALSE,
            }
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
        if let Some(voice_interface) = &mut VOICE_INTERFACE {
            let audio_bytes = _env.get_byte_array_elements(audio_data, ptr::null_mut()).unwrap();
            let audio_vec = audio_bytes.to_vec();
            
            match voice_interface.detect_emergency_phrase(&audio_vec) {
                Ok(phrase) => {
                    if let Some(phrase_str) = phrase {
                        let c_string = CString::new(phrase_str).unwrap();
                        _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
                    } else {
                        ptr::null_mut()
                    }
                }
                Err(_) => ptr::null_mut(),
            }
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
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            let emergency_type_str = _env.get_string(emergency_type).unwrap().into();
            
            match emergency_interface.get_instructions(&emergency_type_str) {
                Ok(instructions) => {
                    let c_string = CString::new(instructions).unwrap();
                    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
                }
                Err(_) => ptr::null_mut(),
            }
        } else {
            ptr::null_mut()
        }
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
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            let emergency_type_str = _env.get_string(emergency_type).unwrap().into();
            let context_str = _env.get_string(context).unwrap().into();
            
            match emergency_interface.get_context_aware_guidance(&emergency_type_str, &context_str) {
                Ok(guidance) => {
                    let c_string = CString::new(guidance).unwrap();
                    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
                }
                Err(_) => ptr::null_mut(),
            }
        } else {
            ptr::null_mut()
        }
    }
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
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            let user_phrase_str = _env.get_string(user_phrase).unwrap().into();
            let location_str = _env.get_string(location).unwrap().into();
            let actions_str = _env.get_string(actions).unwrap().into();
            
            match emergency_interface.detect_stage(&user_phrase_str, &location_str, &actions_str) {
                Ok(stage) => {
                    let c_string = CString::new(stage).unwrap();
                    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
                }
                Err(_) => ptr::null_mut(),
            }
        } else {
            ptr::null_mut()
        }
    }
}

/// Process audio with noise filtering
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_processAudioWithNoiseFiltering(
    _env: JNIEnv,
    _class: JClass,
    audio_data: JObject,
) -> JObject {
    unsafe {
        if let Some(audio_interface) = &mut AUDIO_INTERFACE {
            let audio_bytes = _env.get_byte_array_elements(audio_data, ptr::null_mut()).unwrap();
            let audio_vec = audio_bytes.to_vec();
            
            match audio_interface.apply_noise_filtering(&audio_vec) {
                Ok(filtered_audio) => {
                    let filtered_array = _env.new_byte_array(filtered_audio.len() as i32).unwrap();
                    _env.set_byte_array_region(filtered_array, 0, &filtered_audio).unwrap();
                    filtered_array.into_inner()
                }
                Err(_) => ptr::null_mut(),
            }
        } else {
            ptr::null_mut()
        }
    }
}

/// Award XP for actions
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_awardXP(
    _env: JNIEnv,
    _class: JClass,
    action: JString,
    amount: jint,
) -> jboolean {
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            let action_str = _env.get_string(action).unwrap().into();
            
            match emergency_interface.award_xp(&action_str, amount as u32) {
                Ok(_) => JNI_TRUE,
                Err(_) => JNI_FALSE,
            }
        } else {
            JNI_FALSE
        }
    }
}

/// Get hero level
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getHeroLevel(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            match emergency_interface.get_hero_level() {
                Ok(level) => level as jint,
                Err(_) => 0,
            }
        } else {
            0
        }
    }
}

/// Get total rewards
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getTotalRewards(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            match emergency_interface.get_total_rewards() {
                Ok(rewards) => {
                    let c_string = CString::new(rewards).unwrap();
                    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
                }
                Err(_) => ptr::null_mut(),
            }
        } else {
            ptr::null_mut()
        }
    }
}

/// Connect Solana wallet
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_connectSolanaWallet(
    _env: JNIEnv,
    _class: JClass,
) -> jboolean {
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            match emergency_interface.connect_wallet() {
                Ok(_) => JNI_TRUE,
                Err(_) => JNI_FALSE,
            }
        } else {
            JNI_FALSE
        }
    }
}

/// Get wallet address
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getWalletAddress(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            match emergency_interface.get_wallet_address() {
                Ok(address) => {
                    let c_string = CString::new(address).unwrap();
                    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
                }
                Err(_) => ptr::null_mut(),
            }
        } else {
            ptr::null_mut()
        }
    }
}

/// Record emergency on blockchain
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_recordEmergencyOnBlockchain(
    _env: JNIEnv,
    _class: JClass,
    emergency_data: JString,
) -> jstring {
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            let emergency_data_str = _env.get_string(emergency_data).unwrap().into();
            
            match emergency_interface.record_emergency(&emergency_data_str) {
                Ok(record_id) => {
                    let c_string = CString::new(record_id).unwrap();
                    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
                }
                Err(_) => ptr::null_mut(),
            }
        } else {
            ptr::null_mut()
        }
    }
}

/// Initialize database
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_initializeDatabase(
    _env: JNIEnv,
    _class: JClass,
) -> jboolean {
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            match emergency_interface.initialize_database() {
                Ok(_) => JNI_TRUE,
                Err(_) => JNI_FALSE,
            }
        } else {
            JNI_FALSE
        }
    }
}

/// Save emergency contact
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_saveEmergencyContact(
    _env: JNIEnv,
    _class: JClass,
    name: JString,
    phone: JString,
) -> jboolean {
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            let name_str = _env.get_string(name).unwrap().into();
            let phone_str = _env.get_string(phone).unwrap().into();
            
            match emergency_interface.save_contact(&name_str, &phone_str) {
                Ok(_) => JNI_TRUE,
                Err(_) => JNI_FALSE,
            }
        } else {
            JNI_FALSE
        }
    }
}

/// Get emergency contacts
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_getEmergencyContacts(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            match emergency_interface.get_contacts() {
                Ok(contacts) => {
                    let c_string = CString::new(contacts).unwrap();
                    _env.new_string(c_string.to_str().unwrap()).unwrap().into_inner()
                }
                Err(_) => ptr::null_mut(),
            }
        } else {
            ptr::null_mut()
        }
    }
}

/// Validate emergency type
#[no_mangle]
pub extern "C" fn Java_com_solanasos_emergency_RustBridge_validateEmergencyType(
    _env: JNIEnv,
    _class: JClass,
    emergency_type: JString,
) -> jboolean {
    unsafe {
        if let Some(emergency_interface) = &mut EMERGENCY_INTERFACE {
            let emergency_type_str = _env.get_string(emergency_type).unwrap().into();
            
            match emergency_interface.validate_emergency_type(&emergency_type_str) {
                Ok(valid) => if valid { JNI_TRUE } else { JNI_FALSE },
                Err(_) => JNI_FALSE,
            }
        } else {
            JNI_FALSE
        }
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