-- Crisis Companion Database Schema
-- SQLite database for storing emergency instructions and response data

-- Emergency types table
CREATE TABLE IF NOT EXISTS emergency_types (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Emergency instructions table with official protocol tracking
CREATE TABLE IF NOT EXISTS emergency_instructions (
    id TEXT PRIMARY KEY,
    emergency_type_id INTEGER NOT NULL,
    step_number INTEGER NOT NULL,
    language_code TEXT NOT NULL DEFAULT 'en',  -- Add language code, default English
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    audio_file TEXT,
    estimated_duration_seconds INTEGER NOT NULL,
    -- Official protocol tracking fields
    official_source TEXT,           -- "American Red Cross", "SAMHSA", etc.
    protocol_version TEXT,          -- "2020", "2024", etc.
    last_updated TEXT,              -- Date of last protocol update
    medical_disclaimer TEXT,        -- Standard medical disclaimer
    source_url TEXT,                -- Link to official protocol
    validation_status TEXT,         -- "Verified", "Pending", "Draft", "Expired"
    authority_type TEXT,            -- "MedicalAssociation", "GovernmentAgency", "NonProfit", "Academic", "International"
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (emergency_type_id) REFERENCES emergency_types(id),
    UNIQUE(emergency_type_id, step_number, language_code)  -- Unique per type, step, language
);

-- Emergency responses table
CREATE TABLE IF NOT EXISTS emergency_responses (
    id TEXT PRIMARY KEY,
    emergency_type_id INTEGER NOT NULL,
    trigger_timestamp DATETIME NOT NULL,
    response_start DATETIME NOT NULL,
    response_end DATETIME,
    status TEXT NOT NULL,
    instructions_provided TEXT,
    audio_recorded BOOLEAN DEFAULT FALSE,
    location_shared BOOLEAN DEFAULT FALSE,
    emergency_called BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (emergency_type_id) REFERENCES emergency_types(id)
);

-- Audio recordings table
CREATE TABLE IF NOT EXISTS audio_recordings (
    id TEXT PRIMARY KEY,
    emergency_response_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    duration_seconds INTEGER,
    file_size_bytes INTEGER,
    encryption_key TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (emergency_response_id) REFERENCES emergency_responses(id)
);

-- Device coordination table
CREATE TABLE IF NOT EXISTS device_coordination (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    emergency_response_id TEXT NOT NULL,
    action TEXT NOT NULL,
    battery_level REAL,
    location_lat REAL,
    location_lng REAL,
    timestamp DATETIME NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (emergency_response_id) REFERENCES emergency_responses(id)
);

-- Blockchain transactions table
CREATE TABLE IF NOT EXISTS blockchain_transactions (
    id TEXT PRIMARY KEY,
    emergency_response_id TEXT NOT NULL,
    transaction_signature TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    amount REAL,
    status TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (emergency_response_id) REFERENCES emergency_responses(id)
);

-- User settings table
CREATE TABLE IF NOT EXISTS user_settings (
    id INTEGER PRIMARY KEY,
    setting_key TEXT UNIQUE NOT NULL,
    setting_value TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Emergency contacts table
CREATE TABLE IF NOT EXISTS emergency_contacts (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    email TEXT,
    relationship TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- User profiles table
CREATE TABLE IF NOT EXISTS user_profiles (
    id INTEGER PRIMARY KEY,
    user_id TEXT UNIQUE NOT NULL,
    profile_data TEXT, -- JSON or serialized
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_emergency_instructions_type_step ON emergency_instructions(emergency_type_id, step_number);
CREATE INDEX IF NOT EXISTS idx_emergency_responses_type ON emergency_responses(emergency_type_id);
CREATE INDEX IF NOT EXISTS idx_emergency_responses_timestamp ON emergency_responses(trigger_timestamp);
CREATE INDEX IF NOT EXISTS idx_emergency_responses_status ON emergency_responses(status);
CREATE INDEX IF NOT EXISTS idx_audio_recordings_response ON audio_recordings(emergency_response_id);
CREATE INDEX IF NOT EXISTS idx_device_coordination_response ON device_coordination(emergency_response_id);
CREATE INDEX IF NOT EXISTS idx_device_coordination_device ON device_coordination(device_id);
CREATE INDEX IF NOT EXISTS idx_blockchain_transactions_response ON blockchain_transactions(emergency_response_id);
CREATE INDEX IF NOT EXISTS idx_blockchain_transactions_signature ON blockchain_transactions(transaction_signature);
CREATE INDEX IF NOT EXISTS idx_user_profiles_user_id ON user_profiles(user_id);

-- Add symptoms to emergency_instructions
ALTER TABLE emergency_instructions ADD COLUMN symptoms TEXT;

CREATE INDEX IF NOT EXISTS idx_emergency_instructions_symptoms ON emergency_instructions(symptoms);
CREATE INDEX IF NOT EXISTS idx_emergency_instructions_type_symptoms ON emergency_instructions(emergency_type_id, symptoms);

-- Insert all 15 emergency types
INSERT OR IGNORE INTO emergency_types (id, name, description) VALUES
(1, 'Drowning', 'Water-related emergencies requiring immediate rescue and CPR'),
(2, 'HeartAttack', 'Cardiac emergencies requiring immediate medical attention'),
(3, 'Stroke', 'Brain emergency requiring immediate medical attention'),
(4, 'Choking', 'Airway obstruction emergencies requiring immediate intervention'),
(5, 'Bleeding', 'Blood loss emergencies requiring immediate pressure and medical attention'),
(6, 'Unconscious', 'Loss of consciousness requiring immediate assessment and medical attention'),
(7, 'Seizure', 'Seizure and convulsion emergencies requiring immediate safety measures'),
(8, 'Poisoning', 'Poison and toxin exposure requiring immediate medical attention'),
(9, 'Burn', 'Burn injury requiring immediate cooling and medical assessment'),
(10, 'Diabetic', 'Diabetic emergency requiring immediate sugar or insulin'),
(11, 'AllergicReaction', 'Severe allergic reaction requiring immediate epinephrine'),
(12, 'Trauma', 'Serious injury requiring immediate medical attention'),
(13, 'SuicidePrevention', 'Crisis intervention for suicidal thoughts and self-harm prevention'),
(14, 'OverdoseReversal', 'Opioid overdose reversal and emergency response'),
(15, 'HypothermiaSelfRescue', 'Self-rescue from cold exposure and hypothermia');

-- Insert official emergency protocols from authoritative sources

-- 1. DROWNING (American Heart Association)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('drowning-1', 1, 1, 'en', 'Check Breathing', 'Check if the person is breathing. Look, listen, and feel for breathing for 5-10 seconds.', NULL, 10, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('drowning-2', 1, 2, 'en', 'Call 911', 'If not breathing, call 911 immediately and get help.', NULL, 5, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('drowning-3', 1, 3, 'en', 'Start CPR', 'Begin chest compressions at rate of 100-120 per minute, depth 2-2.4 inches for adults.', NULL, 30, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('drowning-4', 1, 4, 'en', 'Give Rescue Breaths', 'After 30 compressions, give 2 rescue breaths. Allow full chest recoil between compressions.', NULL, 15, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('drowning-5', 1, 5, 'en', 'Continue Until Help Arrives', 'Continue cycles of 30 compressions and 2 breaths until emergency services arrive.', NULL, 60, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation');

-- Add Spanish version for Drowning step 1
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('drowning-1-es', 1, 1, 'es', 'Verificar Respiración', 'Verifique si la persona está respirando. Mire, escuche y sienta la respiración durante 5-10 segundos.', NULL, 10, 'American Heart Association', '2020', '2024-01-15', 'Esta información es solo para fines educativos y no sustituye la atención médica profesional.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation');

-- 2. HEART ATTACK (American Heart Association)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('heart-1', 2, 1, 'en', 'Call 911 Immediately', 'Call 911 immediately. Time is critical for heart attacks - every minute counts.', NULL, 5, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('heart-2', 2, 2, 'en', 'Have Person Sit Down', 'Have the person sit down and rest comfortably. Loosen any tight clothing.', NULL, 10, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('heart-3', 2, 3, 'en', 'Give Aspirin if Available', 'If available and not allergic, give one adult aspirin (325mg) to chew and swallow.', NULL, 20, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('heart-4', 2, 4, 'en', 'Monitor Symptoms', 'Stay with the person and monitor their condition until help arrives.', NULL, 30, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation');

-- 3. STROKE (American Heart Association)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('stroke-1', 3, 1, 'en', 'Call 911 Immediately', 'Call 911 immediately. Time is brain - every minute counts for stroke treatment.', NULL, 5, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('stroke-2', 3, 2, 'en', 'FAST Test', 'Use FAST test: Face (drooping), Arms (weakness), Speech (slurred), Time (call 911).', NULL, 15, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('stroke-3', 3, 3, 'en', 'Keep Person Calm', 'Keep the person calm and comfortable. Do not give food or drink.', NULL, 10, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('stroke-4', 3, 4, 'en', 'Monitor Until Help Arrives', 'Monitor the person and note any changes in condition until EMS arrives.', NULL, 30, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation');

-- 4. CHOKING (American Red Cross)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('choking-1', 4, 1, 'en', 'Assess Severity', 'Ask "Are you choking?" - if they can speak, encourage coughing.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('choking-2', 4, 2, 'en', 'Back Blows', 'Give 5 back blows between shoulder blades using heel of hand.', NULL, 10, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('choking-3', 4, 3, 'en', 'Abdominal Thrusts', 'Give 5 abdominal thrusts (Heimlich maneuver) - place fist above navel, grasp with other hand, thrust inward and upward.', NULL, 15, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('choking-4', 4, 4, 'en', 'Alternate and Continue', 'Continue alternating 5 back blows and 5 abdominal thrusts until object is expelled or person becomes unconscious.', NULL, 20, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('choking-5', 4, 5, 'en', 'Call 911 if Unconscious', 'Call 911 if person becomes unconscious and begin CPR.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit');

-- 5. BLEEDING (American Red Cross)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('bleeding-1', 5, 1, 'en', 'Apply Direct Pressure', 'Apply direct pressure to wound with clean cloth or bandage - apply pressure for at least 10-15 minutes.', NULL, 15, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('bleeding-2', 5, 2, 'en', 'Elevate if Possible', 'Elevate injured area above heart if possible to reduce blood flow.', NULL, 10, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('bleeding-3', 5, 3, 'en', 'Add More Bandages', 'Do not remove blood-soaked bandages. Add more bandages on top if needed.', NULL, 10, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('bleeding-4', 5, 4, 'en', 'Call 911 for Severe Bleeding', 'Call 911 for severe bleeding that cannot be controlled with direct pressure.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit');

-- 6. UNCONSCIOUS (American Heart Association)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('unconscious-1', 6, 1, 'en', 'Check Responsiveness', 'Check if person is responsive - tap and shout "Are you OK?"', NULL, 5, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('unconscious-2', 6, 2, 'en', 'Call 911', 'Call 911 immediately if person is unresponsive.', NULL, 5, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('unconscious-3', 6, 3, 'en', 'Check Breathing', 'Look, listen, and feel for breathing for 5-10 seconds.', NULL, 10, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation'),
('unconscious-4', 6, 4, 'en', 'Begin CPR if Not Breathing', 'If not breathing, begin chest compressions at rate of 100-120 per minute.', NULL, 30, 'American Heart Association', '2020', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://cpr.heart.org/en/resuscitation-science/cpr-and-ecc-guidelines', 'Verified', 'MedicalAssociation');

-- 7. SEIZURE (American Red Cross)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('seizure-1', 7, 1, 'en', 'Clear Area', 'Clear area of dangerous objects and protect person from injury.', NULL, 10, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('seizure-2', 7, 2, 'en', 'Do Not Restrain', 'Do not restrain the person or put anything in their mouth.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('seizure-3', 7, 3, 'en', 'Time the Seizure', 'Time the seizure and call 911 if it lasts more than 5 minutes.', NULL, 10, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('seizure-4', 7, 4, 'en', 'Recovery Position', 'After seizure ends, place person in recovery position on their side.', NULL, 15, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit');

-- 8. POISONING (American Red Cross)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('poisoning-1', 8, 1, 'en', 'Call Poison Control', 'Call Poison Control: 1-800-222-1222 immediately.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('poisoning-2', 8, 2, 'en', 'Do Not Induce Vomiting', 'Do not induce vomiting unless directed by Poison Control or medical professional.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('poisoning-3', 8, 3, 'en', 'Call 911 if Severe', 'Call 911 if person is unconscious, having trouble breathing, or having seizures.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('poisoning-4', 8, 4, 'en', 'Follow Instructions', 'Follow Poison Control instructions exactly and monitor person until help arrives.', NULL, 30, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit');

-- 9. BURN (American Red Cross)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('burn-1', 9, 1, 'en', 'Cool Burn', 'Cool burn with cool (not cold) water for 10-20 minutes.', NULL, 20, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('burn-2', 9, 2, 'en', 'Remove Jewelry', 'Remove jewelry and tight items from burned area before swelling occurs.', NULL, 10, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('burn-3', 9, 3, 'en', 'Cover Loosely', 'Cover burn loosely with sterile gauze or clean cloth.', NULL, 10, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('burn-4', 9, 4, 'en', 'Call 911 if Severe', 'Call 911 for severe burns, burns on face/hands/genitals, or burns larger than palm.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit');

-- 10. DIABETIC (American Red Cross)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('diabetic-1', 10, 1, 'en', 'Check Consciousness', 'Check if person is conscious and responsive.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('diabetic-2', 10, 2, 'en', 'Give Sugar if Conscious', 'If conscious, give sugar (glucose tablets, juice, candy) to raise blood sugar.', NULL, 10, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('diabetic-3', 10, 3, 'en', 'Call 911 if Unconscious', 'Call 911 if person is unconscious or not improving after 15 minutes.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('diabetic-4', 10, 4, 'en', 'Monitor Until Help Arrives', 'Monitor person and keep them comfortable until emergency services arrive.', NULL, 30, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit');

-- 11. ALLERGIC REACTION (American Red Cross)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('allergic-1', 11, 1, 'en', 'Call 911 Immediately', 'Call 911 immediately for severe allergic reaction.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('allergic-2', 11, 2, 'en', 'Use Epinephrine', 'Use epinephrine auto-injector if available and person has been prescribed one.', NULL, 10, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('allergic-3', 11, 3, 'en', 'Monitor Breathing', 'Monitor breathing and be prepared to perform CPR if person stops breathing.', NULL, 15, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('allergic-4', 11, 4, 'en', 'Stay with Person', 'Stay with person until emergency services arrive - reaction can worsen quickly.', NULL, 30, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit');

-- 12. TRAUMA (American Red Cross)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('trauma-1', 12, 1, 'en', 'Call 911 Immediately', 'Call 911 immediately for serious injury.', NULL, 5, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('trauma-2', 12, 2, 'en', 'Stop Bleeding', 'Stop any bleeding by applying direct pressure with clean cloth.', NULL, 15, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('trauma-3', 12, 3, 'en', 'Stabilize Injury', 'Stabilize injury and prevent further movement if possible.', NULL, 20, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit'),
('trauma-4', 12, 4, 'en', 'Monitor Until Help Arrives', 'Monitor person and keep them comfortable until emergency services arrive.', NULL, 30, 'American Red Cross', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.redcross.org/take-a-class/first-aid', 'Verified', 'NonProfit');

-- 13. SUICIDE PREVENTION (National Suicide Prevention Lifeline)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('suicide-1', 13, 1, 'en', 'Call Crisis Hotline', 'Call 988 or 1-800-273-8255 immediately - National Suicide Prevention Lifeline.', NULL, 5, 'National Suicide Prevention Lifeline', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://988lifeline.org/', 'Verified', 'GovernmentAgency'),
('suicide-2', 13, 2, 'en', 'Remove Lethal Means', 'Remove access to lethal means immediately.', NULL, 10, 'National Suicide Prevention Lifeline', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://988lifeline.org/', 'Verified', 'GovernmentAgency'),
('suicide-3', 13, 3, 'en', 'Contact Trusted Person', 'Contact trusted person from your emergency contacts.', NULL, 10, 'National Suicide Prevention Lifeline', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://988lifeline.org/', 'Verified', 'GovernmentAgency'),
('suicide-4', 13, 4, 'en', 'Use Grounding Technique', 'Use 5-4-3-2-1 grounding technique: Name 5 things you see, 4 you can touch, 3 you hear, 2 you smell, 1 you taste.', NULL, 15, 'National Suicide Prevention Lifeline', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://988lifeline.org/', 'Verified', 'GovernmentAgency'),
('suicide-5', 13, 5, 'en', 'Create Safety Plan', 'Create a safety plan with crisis hotline counselor.', NULL, 20, 'National Suicide Prevention Lifeline', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://988lifeline.org/', 'Verified', 'GovernmentAgency');

-- 14. OVERDOSE REVERSAL (SAMHSA)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('overdose-1', 14, 1, 'en', 'Call 911 Immediately', 'Call 911 immediately - opioid overdose is life-threatening.', NULL, 5, 'SAMHSA', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.samhsa.gov/', 'Verified', 'GovernmentAgency'),
('overdose-2', 14, 2, 'en', 'Administer Naloxone', 'Administer naloxone (Narcan): Insert tip in nostril and press plunger, or inject into muscle (thigh, upper arm, or buttock).', NULL, 15, 'SAMHSA', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.samhsa.gov/', 'Verified', 'GovernmentAgency'),
('overdose-3', 14, 3, 'en', 'Recovery Position', 'Place person in recovery position on their side.', NULL, 10, 'SAMHSA', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.samhsa.gov/', 'Verified', 'GovernmentAgency'),
('overdose-4', 14, 4, 'en', 'Monitor Breathing', 'Monitor breathing - if not breathing, begin rescue breathing.', NULL, 15, 'SAMHSA', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.samhsa.gov/', 'Verified', 'GovernmentAgency'),
('overdose-5', 14, 5, 'en', 'Stay Until Help Arrives', 'Stay with person until EMS arrives - overdose can recur.', NULL, 30, 'SAMHSA', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://www.samhsa.gov/', 'Verified', 'GovernmentAgency');

-- 15. HYPOTHERMIA SELF-RESCUE (Wilderness Medical Society)
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, language_code, title, description, audio_file, estimated_duration_seconds, official_source, protocol_version, last_updated, medical_disclaimer, source_url, validation_status, authority_type) VALUES
('hypothermia-1', 15, 1, 'en', 'Get to Shelter', 'Get out of cold environment immediately - find shelter.', NULL, 10, 'Wilderness Medical Society', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://wms.org/', 'Verified', 'MedicalAssociation'),
('hypothermia-2', 15, 2, 'en', 'Remove Wet Clothing', 'Remove wet clothing and replace with dry layers.', NULL, 15, 'Wilderness Medical Society', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://wms.org/', 'Verified', 'MedicalAssociation'),
('hypothermia-3', 15, 3, 'en', 'Begin Rewarming', 'Begin gradual rewarming: Use body-to-body contact, warm sweet drinks (not alcohol), avoid rapid temperature changes.', NULL, 20, 'Wilderness Medical Society', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://wms.org/', 'Verified', 'MedicalAssociation'),
('hypothermia-4', 15, 4, 'en', 'Call 911 if Severe', 'Call 911 if severe hypothermia (confusion, loss of consciousness).', NULL, 5, 'Wilderness Medical Society', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://wms.org/', 'Verified', 'MedicalAssociation'),
('hypothermia-5', 15, 5, 'en', 'Monitor for Frostbite', 'Monitor for frostbite - do not rub affected areas.', NULL, 10, 'Wilderness Medical Society', '2024', '2024-01-15', 'This information is for educational purposes only and is not a substitute for professional medical care.', 'https://wms.org/', 'Verified', 'MedicalAssociation');

-- Insert default user settings
INSERT OR IGNORE INTO user_settings (setting_key, setting_value) VALUES
('emergency_volume', '1.0'),
('voice_confidence_threshold', '0.8'),
('auto_dial_911', 'true'),
('enable_location_sharing', 'true'),
('enable_audio_recording', 'true'),
('enable_family_alerts', 'false'),
('ble_coordination_timeout', '10'),
('max_nearby_devices', '5'),
('battery_threshold', '0.2'),
('ui_theme', 'auto'),
('ui_language', 'en');

-- Insert sample emergency contacts
INSERT OR IGNORE INTO emergency_contacts (name, phone_number, email, relationship) VALUES
('Emergency Services', '911', NULL, 'Emergency'),
('Primary Contact', '+1234567890', 'primary@example.com', 'Spouse'),
('Secondary Contact', '+1234567891', 'secondary@example.com', 'Parent');

-- Create views for easier querying
CREATE VIEW IF NOT EXISTS v_emergency_statistics AS
SELECT 
    et.name as emergency_type,
    COUNT(er.id) as response_count,
    AVG(CASE WHEN er.audio_recorded THEN 1 ELSE 0 END) as audio_recorded_rate,
    AVG(CASE WHEN er.location_shared THEN 1 ELSE 0 END) as location_shared_rate,
    AVG(CASE WHEN er.emergency_called THEN 1 ELSE 0 END) as emergency_called_rate,
    AVG((julianday(er.response_end) - julianday(er.response_start)) * 24 * 60) as avg_response_duration_minutes
FROM emergency_types et
LEFT JOIN emergency_responses er ON et.id = er.emergency_type_id
GROUP BY et.id, et.name
ORDER BY response_count DESC;

CREATE VIEW IF NOT EXISTS v_recent_emergencies AS
SELECT 
    er.id,
    et.name as emergency_type,
    er.trigger_timestamp,
    er.status,
    er.audio_recorded,
    er.location_shared,
    er.emergency_called
FROM emergency_responses er
JOIN emergency_types et ON er.emergency_type_id = et.id
ORDER BY er.trigger_timestamp DESC
LIMIT 50;

-- Create triggers for data integrity
CREATE TRIGGER IF NOT EXISTS update_emergency_responses_timestamp
AFTER UPDATE ON emergency_responses
BEGIN
    UPDATE emergency_responses SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER IF NOT EXISTS update_user_settings_timestamp
AFTER UPDATE ON user_settings
BEGIN
    UPDATE user_settings SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END; 