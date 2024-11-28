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
    pub name: Option<String>,
    pub qth: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub dxcc: Option<u32>,
    pub band: Option<String>,

    
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

#[derive(Clone)]
pub struct DeletedEntry {
    pub entry: LogEntry,
    pub index: usize,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
