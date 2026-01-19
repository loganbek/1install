//! Anonymized telemetry for performance monitoring

use crate::config::load_config;
use crate::config::save_config;
use serde::Serialize;
use uuid::Uuid;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

#[derive(Debug, Serialize)]
pub enum TelemetryEvent {
    UserPing, // Heartbeat to track active users
    SearchStarted { query_length: usize, backends_count: usize },
    SearchFinished { total_results: usize, duration_ms: u128 },
    InstallStarted { backend: String },
    InstallFinished { backend: String, success: bool, duration_ms: u128 },
    ErrorOccurred { error_type: String },
}

pub struct TelemetryClient;

impl TelemetryClient {
    /// Send an anonymized telemetry event
    pub fn track_event(event: TelemetryEvent) {
        let mut config = match load_config() {
            Ok(c) => c,
            Err(_) => return,
        };

        if !config.telemetry.enabled {
            return;
        }

        // Ensure we have a persistent client_id
        let client_id = match config.telemetry.client_id {
            Some(ref id) => id.clone(),
            None => {
                let new_id = Uuid::new_v4().to_string();
                config.telemetry.client_id = Some(new_id.clone());
                let _ = save_config(&config);
                new_id
            }
        };

        // Log to local file for persistence before remote submission implementation
        let log_path = Self::get_telemetry_log_path();
        if let Some(path) = log_path {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path) 
            {
                let timestamp = Utc::now();
                let log_entry = format!("[{}] [{}] {:?}\n", timestamp, client_id, event);
                let _ = file.write_all(log_entry.as_bytes());
            }
        }

        if config.behavior.verbose {
            eprintln!("[TELEMETRY] [{}] Event: {:?}", client_id, event);
        }
        
        // TODO: Implement async HTTPS submission to a central endpoint
    }

    fn get_telemetry_log_path() -> Option<std::path::PathBuf> {
        let home = dirs::home_dir()?;
        Some(home.join(".config").join("1install").join("telemetry.log"))
    }
}
