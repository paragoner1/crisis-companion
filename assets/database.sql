-- Crisis Companion Database Schema
-- SQLite database for storing emergency instructions and response data

-- Emergency types table
CREATE TABLE IF NOT EXISTS emergency_types (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Emergency instructions table
CREATE TABLE IF NOT EXISTS emergency_instructions (
    id TEXT PRIMARY KEY,
    emergency_type_id INTEGER NOT NULL,
    step_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    audio_file TEXT,
    estimated_duration_seconds INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (emergency_type_id) REFERENCES emergency_types(id),
    UNIQUE(emergency_type_id, step_number)
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

-- Insert default emergency types
INSERT OR IGNORE INTO emergency_types (id, name, description) VALUES
(1, 'Drowning', 'Water-related emergencies'),
(2, 'Fire', 'Fire and smoke emergencies'),
(3, 'HeartAttack', 'Cardiac emergencies'),
(4, 'Choking', 'Airway obstruction emergencies'),
(5, 'Bleeding', 'Blood loss emergencies'),
(6, 'Unconscious', 'Loss of consciousness'),
(7, 'Seizure', 'Seizure and convulsion emergencies'),
(8, 'AllergicReaction', 'Severe allergic reactions'),
(9, 'Poisoning', 'Poison and toxin exposure'),
(10, 'Trauma', 'Physical injury emergencies');

-- Insert sample emergency instructions for drowning
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, title, description, audio_file, estimated_duration_seconds) VALUES
('drowning-1', 1, 1, 'Check Breathing', 'Check if the person is breathing. Look, listen, and feel for breathing.', NULL, 10),
('drowning-2', 1, 2, 'Call for Help', 'If not breathing, call 911 immediately and get help.', NULL, 5),
('drowning-3', 1, 3, 'Start CPR', 'Begin chest compressions at a rate of 100-120 per minute.', NULL, 30),
('drowning-4', 1, 4, 'Give Rescue Breaths', 'After 30 compressions, give 2 rescue breaths.', NULL, 15),
('drowning-5', 1, 5, 'Continue Until Help Arrives', 'Continue cycles of 30 compressions and 2 breaths until emergency services arrive.', NULL, 60);

-- Insert sample emergency instructions for fire
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, title, description, audio_file, estimated_duration_seconds) VALUES
('fire-1', 2, 1, 'Get Out Immediately', 'Leave the building immediately. Do not stop to collect belongings.', NULL, 10),
('fire-2', 2, 2, 'Call 911', 'Call 911 from outside the building.', NULL, 5),
('fire-3', 2, 3, 'Stay Low', 'If smoke is present, stay low to the ground where air is cleaner.', NULL, 10),
('fire-4', 2, 4, 'Use Stairs', 'Use stairs, not elevators, to exit the building.', NULL, 15),
('fire-5', 2, 5, 'Meet at Designated Location', 'Go to your designated meeting place outside the building.', NULL, 10);

-- Insert sample emergency instructions for heart attack
INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, title, description, audio_file, estimated_duration_seconds) VALUES
('heart-1', 3, 1, 'Call 911 Immediately', 'Call 911 immediately. Time is critical for heart attacks.', NULL, 5),
('heart-2', 3, 2, 'Have Person Sit Down', 'Have the person sit down and rest comfortably.', NULL, 10),
('heart-3', 3, 3, 'Loosen Tight Clothing', 'Loosen any tight clothing around the neck and chest.', NULL, 15),
('heart-4', 3, 4, 'Give Aspirin if Available', 'If available and not allergic, give one adult aspirin (325mg).', NULL, 20),
('heart-5', 3, 5, 'Monitor Symptoms', 'Stay with the person and monitor their condition until help arrives.', NULL, 30);

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