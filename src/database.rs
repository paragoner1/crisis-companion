#![allow(unused_imports, unused_variables, dead_code)]
use crate::error::AppResult;
use crate::{AppError, types::*};
use crate::config::DatabaseConfig;
use rusqlite::{Connection, params, Result as SqliteResult};
use tracing::{info, debug};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Debug)]
/// Emergency database manager for SQLite operations
pub struct EmergencyDatabase {
    config: DatabaseConfig,
    connection: Arc<Mutex<Connection>>,
}

impl EmergencyDatabase {
    /// Create a new emergency database
    pub fn new(config: &DatabaseConfig) -> AppResult<Self> {
        info!("Initializing emergency database: {}", config.path);
        
        // Create database directory if it doesn't exist
        if let Some(parent) = Path::new(&config.path).parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        
        // Open database connection
        let connection = Connection::open(&config.path)
            .map_err(|e| AppError::DatabaseError(format!("Failed to open database: {}", e)))?;
        
        // Initialize database schema
        connection.execute(
            "CREATE TABLE IF NOT EXISTS emergency_instructions (
                id TEXT PRIMARY KEY,
                emergency_type TEXT NOT NULL,
                step_number INTEGER NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL,
                audio_file TEXT,
                estimated_duration_seconds INTEGER NOT NULL
            )",
            [],
        )?;
        
        connection.execute(
            "CREATE TABLE IF NOT EXISTS user_profiles (
                id TEXT PRIMARY KEY,
                default_role TEXT NOT NULL,
                is_caregiver BOOLEAN NOT NULL,
                emergency_contacts TEXT,
                medical_info TEXT
            )",
            [],
        )?;
        
        connection.execute(
            "CREATE TABLE IF NOT EXISTS emergency_history (
                id TEXT PRIMARY KEY,
                emergency_type TEXT NOT NULL,
                trigger_timestamp TEXT NOT NULL,
                response_start TEXT NOT NULL,
                response_end TEXT,
                status TEXT NOT NULL,
                instructions_provided TEXT,
                audio_recorded BOOLEAN NOT NULL,
                location_shared BOOLEAN NOT NULL,
                emergency_called BOOLEAN NOT NULL
            )",
            [],
        )?;
        
        info!("✅ Emergency database initialized successfully");
        Ok(Self { 
            connection: Arc::new(Mutex::new(connection)),
            config: config.clone(),
        })
    }
    
    /// Initialize database schema
    fn initialize_schema(&self) -> AppResult<()> {
        info!("Initializing database schema");
        
        let conn = self.connection.lock().unwrap();
        
        // Create emergency_types table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS emergency_types (
                id INTEGER PRIMARY KEY,
                name TEXT UNIQUE NOT NULL,
                description TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        // Create emergency_instructions table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS emergency_instructions (
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
            )",
            [],
        )?;
        
        // Create emergency_responses table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS emergency_responses (
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
            )",
            [],
        )?;
        
        // Create audio_recordings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS audio_recordings (
                id TEXT PRIMARY KEY,
                emergency_response_id TEXT NOT NULL,
                file_path TEXT NOT NULL,
                duration_seconds INTEGER,
                file_size_bytes INTEGER,
                encryption_key TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (emergency_response_id) REFERENCES emergency_responses(id)
            )",
            [],
        )?;
        
        // Create indexes for better performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_emergency_instructions_type_step ON emergency_instructions(emergency_type_id, step_number)",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_emergency_responses_type ON emergency_responses(emergency_type_id)",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_emergency_responses_timestamp ON emergency_responses(trigger_timestamp)",
            [],
        )?;
        
        info!("Database schema initialized successfully");
        Ok(())
    }
    
    /// Load initial emergency data
    fn load_initial_data(&self) -> AppResult<()> {
        info!("Loading initial emergency data");
        
        let conn = self.connection.lock().unwrap();
        
        // Check if emergency types already exist
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM emergency_types",
            [],
            |row| row.get(0),
        )?;
        
        if count == 0 {
            // Insert emergency types
            let emergency_types = vec![
                ("Drowning", "Water-related emergencies"),
                ("HeartAttack", "Cardiac emergencies"),
                ("Stroke", "Stroke and brain emergencies"),
                ("Choking", "Airway obstruction emergencies"),
                ("Bleeding", "Blood loss and hemorrhage"),
                ("Unconscious", "Unconsciousness and cardiac arrest"),
                ("Seizure", "Seizure and convulsion emergencies"),
                ("Poisoning", "Poison and overdose emergencies"),
                ("SevereBurns", "Severe burn emergencies"),
                ("DiabeticEmergency", "Diabetic emergency crises"),
                ("AllergicReaction", "Allergic reactions and anaphylaxis"),
                ("Trauma", "Injury and trauma emergencies"),
            ];
            
            for (name, description) in emergency_types {
                conn.execute(
                    "INSERT INTO emergency_types (name, description) VALUES (?, ?)",
                    params![name, description],
                )?;
            }
            
            // Insert sample emergency instructions
            Self::insert_sample_instructions(&conn)?;
            
            info!("Initial emergency data loaded successfully");
        } else {
            info!("Emergency data already exists, skipping initial load");
        }
        
        Ok(())
    }
    
    /// Insert sample emergency instructions
    fn insert_sample_instructions(conn: &Connection) -> AppResult<()> {
        info!("Inserting sample emergency instructions");
        
        // Drowning instructions
        let drowning_instructions = vec![
            (1, "Check Breathing", "Check if the person is breathing. Look, listen, and feel for breathing.", None::<String>, 10),
            (2, "Call for Help", "If not breathing, call 911 immediately and get help.", None::<String>, 5),
            (3, "Start CPR", "Begin chest compressions at a rate of 100-120 per minute.", None::<String>, 30),
            (4, "Give Rescue Breaths", "After 30 compressions, give 2 rescue breaths.", None::<String>, 15),
            (5, "Continue Until Help Arrives", "Continue cycles of 30 compressions and 2 breaths until emergency services arrive.", None::<String>, 60),
        ];
        
        for (step, title, description, audio_file, duration) in drowning_instructions {
            conn.execute(
                "INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, title, description, audio_file, estimated_duration_seconds) VALUES (?, ?, ?, ?, ?, ?, ?)",
                params![
                    Uuid::new_v4().to_string(),
                    1, // Drowning emergency type ID
                    step,
                    title,
                    description,
                    audio_file,
                    duration,
                ],
            )?;
        }
        
        // Fire instructions
        let fire_instructions = vec![
            (1, "Get Out Immediately", "Leave the building immediately. Do not stop to collect belongings.", None::<String>, 10),
            (2, "Call 911", "Call 911 from outside the building.", None::<String>, 5),
            (3, "Stay Low", "If smoke is present, stay low to the ground where air is cleaner.", None::<String>, 10),
            (4, "Use Stairs", "Use stairs, not elevators, to exit the building.", None::<String>, 15),
            (5, "Meet at Designated Location", "Go to your designated meeting place outside the building.", None::<String>, 10),
        ];
        
        for (step, title, description, audio_file, duration) in fire_instructions {
            conn.execute(
                "INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, title, description, audio_file, estimated_duration_seconds) VALUES (?, ?, ?, ?, ?, ?, ?)",
                params![
                    Uuid::new_v4().to_string(),
                    2, // Fire emergency type ID
                    step,
                    title,
                    description,
                    audio_file,
                    duration,
                ],
            )?;
        }
        
        // Heart Attack instructions
        let heart_attack_instructions = vec![
            (1, "Call 911 Immediately", "Call 911 immediately. Time is critical for heart attacks.", None::<String>, 5),
            (2, "Have Person Sit Down", "Have the person sit down and rest comfortably.", None::<String>, 10),
            (3, "Loosen Tight Clothing", "Loosen any tight clothing around the neck and chest.", None::<String>, 15),
            (4, "Give Aspirin if Available", "If available and not allergic, give one adult aspirin (325mg).", None::<String>, 20),
            (5, "Monitor Symptoms", "Stay with the person and monitor their condition until help arrives.", None::<String>, 30),
        ];
        
        for (step, title, description, audio_file, duration) in heart_attack_instructions {
            conn.execute(
                "INSERT OR IGNORE INTO emergency_instructions (id, emergency_type_id, step_number, title, description, audio_file, estimated_duration_seconds) VALUES (?, ?, ?, ?, ?, ?, ?)",
                params![
                    Uuid::new_v4().to_string(),
                    3, // Heart Attack emergency type ID
                    step,
                    title,
                    description,
                    audio_file,
                    duration,
                ],
            )?;
        }
        
        info!("Sample emergency instructions inserted successfully");
        Ok(())
    }
    
    /// Get emergency instructions for a specific emergency type
    pub fn get_emergency_instructions(&self, emergency_type: &EmergencyType) -> AppResult<Vec<EmergencyInstruction>> {
        debug!("Getting instructions for emergency type: {:?}", emergency_type);
        
        let conn = self.connection.lock().unwrap();
        
        // Get emergency type ID
        let emergency_type_id: i64 = conn.query_row(
            "SELECT id FROM emergency_types WHERE name = ?",
            params![format!("{:?}", emergency_type)],
            |row| row.get(0),
        )?;
        
        // Get instructions
        let mut stmt = conn.prepare(
            "SELECT id, emergency_type_id, step_number, title, description, audio_file, estimated_duration_seconds 
             FROM emergency_instructions 
             WHERE emergency_type_id = ? 
             ORDER BY step_number"
        )?;
        
        let instructions = stmt.query_map(params![emergency_type_id], |row| {
            Ok(EmergencyInstruction {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                emergency_type: emergency_type.clone(),
                step_number: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                audio_file: row.get(5)?,
                estimated_duration_seconds: row.get(6)?,
            })
        })?.collect::<SqliteResult<Vec<_>>>()?;
        
        info!("Retrieved {} instructions for {:?}", instructions.len(), emergency_type);
        Ok(instructions)
    }
    
    /// Get all emergency types
    pub fn get_emergency_types(&self) -> AppResult<Vec<EmergencyType>> {
        debug!("Getting all emergency types");
        
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT name FROM emergency_types ORDER BY id")?;
        
        let emergency_types = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            Ok(match name.as_str() {
                "Drowning" => EmergencyType::Drowning,
                "HeartAttack" => EmergencyType::HeartAttack,
                "Stroke" => EmergencyType::Stroke,
                "Choking" => EmergencyType::Choking,
                "Bleeding" => EmergencyType::Bleeding,
                "Unconscious" => EmergencyType::Unconscious,
                "Seizure" => EmergencyType::Seizure,
                "Poisoning" => EmergencyType::Poisoning,
                "SevereBurns" => EmergencyType::SevereBurns,
                "DiabeticEmergency" => EmergencyType::DiabeticEmergency,
                "AllergicReaction" => EmergencyType::AllergicReaction,
                "Trauma" => EmergencyType::Trauma,
                _ => EmergencyType::Drowning, // Default fallback
            })
        })?.collect::<SqliteResult<Vec<_>>>()?;
        
        info!("Retrieved {} emergency types", emergency_types.len());
        Ok(emergency_types)
    }
    
    /// Record an emergency response
    pub fn record_emergency_response(&self, response: &EmergencyResponse) -> AppResult<()> {
        info!("Recording emergency response: {:?}", response.emergency_type);
        
        let conn = self.connection.lock().unwrap();
        
        // Get emergency type ID
        let emergency_type_id: i64 = conn.query_row(
            "SELECT id FROM emergency_types WHERE name = ?",
            params![format!("{:?}", response.emergency_type)],
            |row| row.get(0),
        )?;
        
        // Insert emergency response
        conn.execute(
            "INSERT INTO emergency_responses (
                id, emergency_type_id, trigger_timestamp, response_start, response_end, 
                status, instructions_provided, audio_recorded, location_shared, emergency_called
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                response.id.to_string(),
                emergency_type_id,
                response.trigger_timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
                response.response_start.format("%Y-%m-%d %H:%M:%S").to_string(),
                response.response_end.map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string()),
                format!("{:?}", response.status),
                serde_json::to_string(&response.instructions_provided)?,
                response.audio_recorded,
                response.location_shared,
                response.emergency_called,
            ],
        )?;
        
        info!("Emergency response recorded successfully");
        Ok(())
    }
    
    /// Get emergency response statistics
    pub fn get_response_statistics(&self) -> AppResult<Vec<(EmergencyType, i64)>> {
        debug!("Getting emergency response statistics");
        
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT et.name, COUNT(er.id) 
             FROM emergency_types et 
             LEFT JOIN emergency_responses er ON et.id = er.emergency_type_id 
             GROUP BY et.id, et.name 
             ORDER BY COUNT(er.id) DESC"
        )?;
        
        let stats = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            let count: i64 = row.get(1)?;
            
            let emergency_type = match name.as_str() {
                "Drowning" => EmergencyType::Drowning,
                "HeartAttack" => EmergencyType::HeartAttack,
                "Stroke" => EmergencyType::Stroke,
                "Choking" => EmergencyType::Choking,
                "Bleeding" => EmergencyType::Bleeding,
                "Unconscious" => EmergencyType::Unconscious,
                "Seizure" => EmergencyType::Seizure,
                "Poisoning" => EmergencyType::Poisoning,
                "SevereBurns" => EmergencyType::SevereBurns,
                "DiabeticEmergency" => EmergencyType::DiabeticEmergency,
                "AllergicReaction" => EmergencyType::AllergicReaction,
                "Trauma" => EmergencyType::Trauma,
                _ => EmergencyType::Drowning,
            };
            
            Ok((emergency_type, count))
        })?.collect::<SqliteResult<Vec<_>>>()?;
        
        info!("Retrieved statistics for {} emergency types", stats.len());
        Ok(stats)
    }
    
    /// Test database functionality
    pub fn test_database(&self) -> AppResult<()> {
        info!("Testing database functionality");
        
        // Test getting emergency types
        let emergency_types = self.get_emergency_types()?;
        assert!(!emergency_types.is_empty());
        assert!(emergency_types.contains(&EmergencyType::Drowning));
        assert!(emergency_types.contains(&EmergencyType::Choking));
        assert!(emergency_types.contains(&EmergencyType::Bleeding));
        
        // Test getting instructions for drowning
        let drowning_instructions = self.get_emergency_instructions(&EmergencyType::Drowning)?;
        assert!(!drowning_instructions.is_empty());
        assert_eq!(drowning_instructions[0].emergency_type, EmergencyType::Drowning);
        assert_eq!(drowning_instructions[0].step_number, 1);
        
        // Test getting instructions for bleeding
        let bleeding_instructions = self.get_emergency_instructions(&EmergencyType::Bleeding)?;
        assert!(!bleeding_instructions.is_empty());
        
        // Test getting instructions for unconscious
        let unconscious_instructions = self.get_emergency_instructions(&EmergencyType::Unconscious)?;
        assert!(!unconscious_instructions.is_empty());
        
        // Test statistics
        let stats = self.get_response_statistics()?;
        assert!(!stats.is_empty());
        
        info!("Database functionality test completed successfully");
        Ok(())
    }
}

impl Drop for EmergencyDatabase {
    fn drop(&mut self) {
        info!("Emergency database shutting down");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DatabaseConfig;
    use tempfile::tempdir;
    
    #[test]
    fn test_database_creation() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        let config = DatabaseConfig {
            path: db_path.to_str().unwrap().to_string(),
            ..Default::default()
        };
        
        let database = EmergencyDatabase::new(&config);
        assert!(database.is_ok());
    }
    
    #[test]
    fn test_emergency_types() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        let config = DatabaseConfig {
            path: db_path.to_str().unwrap().to_string(),
            ..Default::default()
        };
        
        let database = EmergencyDatabase::new(&config).unwrap();
        let emergency_types = database.get_emergency_types().unwrap();
        
        assert!(!emergency_types.is_empty());
        assert!(emergency_types.contains(&EmergencyType::Drowning));
        assert!(emergency_types.contains(&EmergencyType::Choking));
        assert!(emergency_types.contains(&EmergencyType::Bleeding));
    }
    
    #[test]
    fn test_emergency_instructions() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        let config = DatabaseConfig {
            path: db_path.to_str().unwrap().to_string(),
            ..Default::default()
        };
        
        let database = EmergencyDatabase::new(&config).unwrap();
        
        // Test drowning instructions
        let drowning_instructions = database.get_emergency_instructions(&EmergencyType::Drowning).unwrap();
        assert!(!drowning_instructions.is_empty());
        assert_eq!(drowning_instructions[0].emergency_type, EmergencyType::Drowning);
        assert_eq!(drowning_instructions[0].step_number, 1);
        
        // Test choking instructions
        let choking_instructions = database.get_emergency_instructions(&EmergencyType::Choking).unwrap();
        assert!(!choking_instructions.is_empty());
        assert_eq!(choking_instructions[0].emergency_type, EmergencyType::Choking);
        assert_eq!(choking_instructions[0].step_number, 1);
    }
    
    #[test]
    fn test_response_statistics() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        let config = DatabaseConfig {
            path: db_path.to_str().unwrap().to_string(),
            ..Default::default()
        };
        
        let database = EmergencyDatabase::new(&config).unwrap();
        let stats = database.get_response_statistics().unwrap();
        
        assert!(!stats.is_empty());
        // All counts should be 0 for a fresh database
        for (_, count) in &stats {
            assert_eq!(*count, 0);
        }
    }
} 