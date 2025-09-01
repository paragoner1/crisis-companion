//! Common Types and Enums
use serde::{Serialize, Deserialize};// This module provides common types and enums used throughout the application.

/// Emergency types supported by Solana SOS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmergencyType {
    /// Drowning emergency
    Drowning,
    /// Heart attack emergency
    HeartAttack,
    /// Stroke emergency
    Stroke,
    /// Choking emergency
    Choking,
    /// Bleeding emergency
    Bleeding,
    /// Unconscious emergency
    Unconscious,
    /// Seizure emergency
    Seizure,
    /// Poisoning emergency
    Poisoning,
    /// Severe burns emergency
    SevereBurns,
    /// Diabetic emergency
    DiabeticEmergency,
    /// Allergic reaction emergency
    AllergicReaction,
    /// Trauma emergency
    Trauma,
    /// Suicide prevention emergency
    Suicide,
    /// Drug overdose emergency
    DrugOverdose,
    /// Hypothermia emergency
    Hypothermia,
    /// Other emergency
    Other,
}

impl EmergencyType {
    /// Gets the display name for the emergency type
    pub fn display_name(&self) -> &'static str {
        match self {
            EmergencyType::Drowning => "Drowning",
            EmergencyType::HeartAttack => "Heart Attack",
            EmergencyType::Stroke => "Stroke",
            EmergencyType::Choking => "Choking",
            EmergencyType::Bleeding => "Bleeding",
            EmergencyType::Unconscious => "Unconscious",
            EmergencyType::Seizure => "Seizure",
            EmergencyType::Poisoning => "Poisoning",
            EmergencyType::SevereBurns => "Severe Burns",
            EmergencyType::DiabeticEmergency => "Diabetic Emergency",
            EmergencyType::AllergicReaction => "Allergic Reaction",
            EmergencyType::Trauma => "Trauma",
            EmergencyType::Suicide => "Suicide Prevention",
            EmergencyType::DrugOverdose => "Drug Overdose",
            EmergencyType::Hypothermia => "Hypothermia",
            EmergencyType::Other => "Other",
        }
    }

    /// Gets the description for the emergency type
    pub fn description(&self) -> &'static str {
        match self {
            EmergencyType::Drowning => "Water-related emergency requiring immediate rescue",
            EmergencyType::HeartAttack => "Cardiac emergency requiring immediate medical attention",
            EmergencyType::Stroke => "Neurological emergency requiring immediate medical attention",
            EmergencyType::Choking => "Airway obstruction requiring immediate intervention",
            EmergencyType::Bleeding => "Blood loss requiring immediate control",
            EmergencyType::Unconscious => "Loss of consciousness requiring immediate assessment",
            EmergencyType::Seizure => "Neurological episode requiring immediate safety measures",
            EmergencyType::Poisoning => "Toxic substance exposure requiring immediate treatment",
            EmergencyType::SevereBurns => "Thermal injury requiring immediate cooling and care",
            EmergencyType::DiabeticEmergency => "Blood sugar emergency requiring immediate intervention",
            EmergencyType::AllergicReaction => "Severe allergic response requiring immediate treatment",
            EmergencyType::Trauma => "Physical injury requiring immediate assessment and care",
            EmergencyType::Suicide => "Mental health crisis requiring immediate human connection and support",
            EmergencyType::DrugOverdose => "Substance overdose requiring harm reduction approach and medical intervention",
            EmergencyType::Hypothermia => "Cold exposure requiring gradual rewarming and specialized care",
            EmergencyType::Other => "Other emergency requiring immediate attention",
        }
    }
}

/// Emergency stages for context-aware guidance
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmergencyStage {
    /// Initial detection of emergency
    InitialDetection,
    /// Victim has been extracted from danger
    VictimExtracted,
    /// Victim is unconscious
    Unconscious,
    /// Victim is conscious but injured
    ConsciousButInjured,
    /// Victim is breathing but unresponsive
    BreathingButUnresponsive,
    /// Emergency services are en route
    ServicesEnRoute,
    /// Post-emergency care
    PostEmergency,
}

impl EmergencyStage {
    /// Gets the display name for the emergency stage
    pub fn display_name(&self) -> &'static str {
        match self {
            EmergencyStage::InitialDetection => "Initial Detection",
            EmergencyStage::VictimExtracted => "Victim Extracted",
            EmergencyStage::Unconscious => "Unconscious",
            EmergencyStage::ConsciousButInjured => "Conscious but Injured",
            EmergencyStage::BreathingButUnresponsive => "Breathing but Unresponsive",
            EmergencyStage::ServicesEnRoute => "Services En Route",
            EmergencyStage::PostEmergency => "Post Emergency",
        }
    }
}

/// Direct action phrases for trained responders
pub enum DirectAction {
    /// CPR action
    CPR,
    /// Heimlich maneuver
    Heimlich,
    /// AED usage
    AED,
    /// Tourniquet application
    Tourniquet,
    /// EpiPen administration
    EpiPen,
    /// Rescue breathing
    RescueBreathing,
    /// First aid
    FirstAid,
    /// FAST test for stroke
    FASTTest,
    /// Poison control
    PoisonControl,
    /// Cool burn treatment
    CoolBurn,
    /// Medical alert
    MedicalAlert,
}

impl DirectAction {
    /// Gets the display name for the direct action
    pub fn display_name(&self) -> &'static str {
        match self {
            DirectAction::CPR => "CPR",
            DirectAction::Heimlich => "Heimlich Maneuver",
            DirectAction::AED => "AED",
            DirectAction::Tourniquet => "Tourniquet",
            DirectAction::EpiPen => "EpiPen",
            DirectAction::RescueBreathing => "Rescue Breathing",
            DirectAction::FirstAid => "First Aid",
            DirectAction::FASTTest => "FAST Test",
            DirectAction::PoisonControl => "Poison Control",
            DirectAction::CoolBurn => "Cool Burn",
            DirectAction::MedicalAlert => "Medical Alert",
        }
    }

    /// Gets the description for the direct action
    pub fn description(&self) -> &'static str {
        match self {
            DirectAction::CPR => "Cardiopulmonary resuscitation for cardiac arrest",
            DirectAction::Heimlich => "Abdominal thrusts for choking",
            DirectAction::AED => "Automated external defibrillator usage",
            DirectAction::Tourniquet => "Tourniquet application for severe bleeding",
            DirectAction::EpiPen => "Epinephrine auto-injector for severe allergic reaction",
            DirectAction::RescueBreathing => "Rescue breathing for respiratory arrest",
            DirectAction::FirstAid => "Basic first aid procedures",
            DirectAction::FASTTest => "Stroke assessment test",
            DirectAction::PoisonControl => "Poison control procedures",
            DirectAction::CoolBurn => "Cool burn treatment",
            DirectAction::MedicalAlert => "Medical alert activation",
        }
    }
}

/// Connectivity modes for hybrid architecture
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectivityMode {
    /// Offline mode only
    Offline,
    /// Online mode only
    Online,
    /// Hybrid mode with both offline and online
    Hybrid,
}

impl ConnectivityMode {
    /// Gets the display name for the connectivity mode
    pub fn display_name(&self) -> &'static str {
        match self {
            ConnectivityMode::Offline => "Offline",
            ConnectivityMode::Online => "Online",
            ConnectivityMode::Hybrid => "Hybrid",
        }
    }
}

/// Guidance modes for context-aware system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuidanceMode {
    /// Offline guidance only
    Offline,
    /// Online guidance only
    Online,
    /// Hybrid guidance with both modes
    Hybrid,
}

impl GuidanceMode {
    /// Gets the display name for the guidance mode
    pub fn display_name(&self) -> &'static str {
        match self {
            GuidanceMode::Offline => "Offline",
            GuidanceMode::Online => "Online",
            GuidanceMode::Hybrid => "Hybrid",
        }
    }
} 
// ===== EMERGENCY TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SilentSOSMethod {
    MotionGesture,
    ScreenTapPattern,
    PowerButtonSequence,
    VolumeButtonSequence,
    HoldButton,
    Voice,
    Gesture,
    Button,
    Automatic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CrashStatus {
    NoCrash,
    PotentialCrash,
    ConfirmedCrash,
    None,
    Detected,
    Confirmed,
    Responding,
    Resolved,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrustedContact {
    pub phone_number: String,
    pub location_sharing_enabled: bool,
    pub emergency_access_enabled: bool,
    pub notification_preferences: NotificationPreferences,
    pub id: String,
    pub name: String,
    pub relationship: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NotificationType {
    Emergency,
    Warning,
    Info,
    Success,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub accuracy: Option<f64>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GPSData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub speed: f64,
    pub heading: f64,
    pub accuracy: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencyContact {
    pub id: String,
    pub name: String,
    pub relationship: String,
    pub last_notified: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencyData {
    pub id: String,
    pub emergency_type: EmergencyType,
    pub location: Location,
    pub timestamp: u64,
    pub severity: u8,
    pub description: String,
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencyRecord {
    pub id: String,
    pub emergency_data: EmergencyData,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencyHistory {
    pub user_id: String,
    pub records: Vec<EmergencyRecord>,
    pub total_emergencies: u32,
    pub last_emergency: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencyInstruction {
    pub title: String,
    pub description: String,
    pub audio_file: Option<String>,
    pub estimated_duration_seconds: u64,
    pub id: String,
    pub emergency_type: EmergencyType,
    pub step_number: u8,
    pub instruction: String,
    pub duration: Option<u64>,
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencyProtocol {
    pub id: String,
    pub emergency_type: EmergencyType,
    pub name: String,
    pub description: String,
    pub steps: Vec<EmergencyStep>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencyStep {
    pub id: String,
    pub step_number: u8,
    pub action: String,
    pub description: String,
    pub duration: Option<u64>,
    pub requires_confirmation: bool,
}

// ===== USER & ROLE TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
    Bystander,
    Victim,
    Unknown,
    Driver,
    Passenger,
    Pedestrian,
    EmergencyResponder,
    Administrator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoleDetectionMethod {
    AIInference,
    UITap,
    UserProfile,
    VoiceConfirmation,
    Voice,
    Movement,
    Location,
    Manual,
    Automatic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleDetectionResult {
    pub role: UserRole,
    pub detected_role: UserRole,
    pub detection_time_ms: u64,
    pub context_data: Option<RoleContext>,
    pub confidence: f64,
    pub method: RoleDetectionMethod,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoleContext {
    pub detection_method: RoleDetectionMethod,
    pub last_updated: u64,
    pub current_detected_role: UserRole,
    pub phrase_detected: Option<String>,
    pub sensor_data: Option<SensorData>,
    pub user_profile: Option<UserProfile>,
    pub emergency_type: Option<EmergencyType>,
    pub user_id: String,
    pub current_role: UserRole,
    pub confidence: f64,
}

// ===== SENSOR & LOCATION TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorData {
    pub audio_environment: Option<AudioEnvironment>,
    pub device_movement: Option<DeviceMovement>,
    pub location_context: Option<LocationContext>,
    pub accelerometer: Option<[f64; 3]>,
    pub gyroscope: Option<[f64; 3]>,
    pub magnetometer: Option<[f64; 3]>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeviceMovement {
    Stationary,
    Walking,
    Running,
    Swimming,
    Falling,
    Driving,
}

pub struct DeviceMovementData {
    pub acceleration: [f64; 3],
    pub velocity: [f64; 3],
    pub orientation: [f64; 3],
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocationType {
    Beach,
    Pool,
    Hospital,
    Home,
    Work,
    Vehicle,
    Public,
    Emergency,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationContext {
    pub location_type: LocationType,
    pub coordinates: Location,
    pub address: Option<String>,
    pub safety_score: f64,
    pub timestamp: u64,
}

// ===== AUDIO & VOICE TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioEnvironment {
    pub noise_level: f64,
    pub clarity: f64,
    pub background_noise: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceModel {
    pub user_id: String,
    pub model_path: String,
    pub accuracy: f64,
    pub last_trained: u64,
    pub is_active: bool,
}

// ===== MEDICAL TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MedicalInfo {
    pub user_id: String,
    pub blood_type: Option<String>,
    pub allergies: Vec<String>,
    pub medications: Vec<String>,
    pub conditions: Vec<String>,
    pub emergency_contacts: Vec<EmergencyContact>,
}

// ===== TOKEN & ACHIEVEMENT TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    Emergency,
    Safety,
    Training,
    Achievement,
    Bonus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementType {
    EmergencyResponse,
    SafetyTraining,
    CommunityHelp,
    QuickResponse,
    PerfectProtocol,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AchievementLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub achievement_type: AchievementType,
    pub level: AchievementLevel,
    pub name: String,
    pub description: String,
    pub points: u32,
    pub unlocked_at: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HeroLevel {
    Novice,
    Apprentice,
    Guardian,
    Hero,
    Legend,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub level: u8,
    pub experience: u32,
    pub max_experience: u32,
}

// ===== TRAINING TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainingModule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub difficulty: u8,
    pub duration: u64,
    pub topics: Vec<String>,
    pub is_completed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainingProgress {
    pub user_id: String,
    pub module_id: String,
    pub progress: f64,
    pub completed_lessons: Vec<String>,
    pub current_lesson: Option<String>,
    pub last_accessed: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AITrainingRecommendation {
    pub user_id: String,
    pub recommended_modules: Vec<TrainingModule>,
    pub reasoning: String,
    pub generated_at: u64,
}

// ===== UI TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIData {
    pub user_id: String,
    pub current_screen: String,
    pub notifications: Vec<String>,
    pub last_interaction: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIInput {
    pub input_type: String,
    pub data: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UIResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<String>,
    pub timestamp: u64,
}

// ===== CRASH TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrashData {
    pub id: String,
    pub severity: u8,
    pub location: Location,
    pub timestamp: u64,
    pub sensor_data: SensorData,
    pub user_id: String,
}

// ===== COORDINATION TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CoordinationAction {
    ProvideInstructions,
    DisplayInstructions,
    Dial911,
    RecordAudio,
    SendLocation,
    ContactEmergencyContacts,
    InitiateEmergencyProtocol,
}

// ===== NOTIFICATION TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub silent_sos: bool,
    pub crash_detection: bool,
    pub emergency_alerts: bool,
    pub safety_warnings: bool,
    pub training_reminders: bool,
    pub achievement_notifications: bool,
    pub sound_enabled: bool,
    pub vibration_enabled: bool,
    pub quiet_hours_start: Option<u8>,
    pub quiet_hours_end: Option<u8>,
}

// ===== USER PROFILE TYPE =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserProfile {
    pub is_caregiver: bool,
    pub default_role: UserRole,
    pub id: String,
    pub name: String,
    pub email: String,
    pub emergency_contacts: Vec<EmergencyContact>,
    pub medical_info: Option<MedicalInfo>,
    pub notification_preferences: NotificationPreferences,
    pub created_at: u64,
    pub updated_at: u64,
}

// ===== VOICE TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceTriggerResult {
    pub audio_hash: String,
    pub detected: bool,
    pub confidence: f64,
    pub phrase: String,
    pub timestamp: u64,
    pub emergency_type: Option<EmergencyType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceAnalysisResult {
    pub stress_level: f64,
    pub emotion: String,
    pub clarity: f64,
    pub background_noise: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceCommand {
    pub command: String,
    pub parameters: Vec<String>,
    pub confidence: f64,
    pub timestamp: u64,
}

// ===== EMERGENCY RESPONSE TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmergencyResponse {
    pub status: ResponseStatus,
    pub trigger_timestamp: u64,
    pub response_start: u64,
    pub response_end: Option<u64>,
    pub instructions_provided: bool,
    pub audio_recorded: bool,
    pub location_shared: bool,
    pub emergency_called: bool,
    pub id: String,
    pub emergency_type: EmergencyType,
    pub location: Location,
    pub timestamp: u64,
    pub instructions: Vec<EmergencyInstruction>,
    pub current_step: u8,
    pub completed_steps: Vec<String>,
    pub estimated_completion_time: Option<u64>,
    pub emergency_contacts_notified: Vec<String>,
    pub emergency_services_contacted: bool,
}

// ===== RESPONSE STATUS TYPES =====
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseStatus {
    Pending,
    Active,
    Completed,
    Cancelled,
    Failed,
}
