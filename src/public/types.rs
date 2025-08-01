//! Common Types and Enums
//! 
//! This module provides common types and enums used throughout the application.

/// Emergency types supported by Solana SOS
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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