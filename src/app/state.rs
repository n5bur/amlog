// src/app/state.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum AppMode {
    Normal,
    NewEntry,
    Edit,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub callsign: String,
    pub frequency: f64,
    pub mode: String,
    pub rst_sent: Option<String>,
    pub rst_received: Option<String>,
    pub notes: Option<String>,
    
    // Optional fields
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub grid: Option<String>,
    #[serde(default)]
    pub power: Option<f32>,

    // Extensible fields for plugins
    #[serde(default)]
    pub custom_fields: HashMap<String, String>,
}