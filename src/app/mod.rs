// src/app/mod.rs
mod form;
mod state;

pub use form::{Form, FormField};
pub use state::{AppMode, LogEntry};

use chrono::Utc;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use uuid::Uuid;

use crate::storage::{StorageManager, StorageFormat, StorageError};

/// Main application state container
pub struct App {
    pub mode: AppMode,
    pub form: Form,
    pub status_message: Option<(String, bool)>, // (message, is_error)
    entries: Vec<LogEntry>,
    storage_manager: StorageManager,
    runtime: Runtime,
    selected_index: Option<usize>,  // Add this field
    
}

impl App {
    pub fn new() -> Result<Self, StorageError> {
        let runtime = Runtime::new()
            .map_err(|e| StorageError::Backend(format!("Failed to create runtime: {}", e)))?;
        
        let storage_manager = match StorageManager::new(
            StorageFormat::Json,
            PathBuf::from("logbook.json"),
        ) {
            Ok(sm) => sm,
            Err(e) => {
                eprintln!("Failed to initialize storage: {}. Starting with empty log.", e);
                // Try to create with a different filename
                StorageManager::new(
                    StorageFormat::Json,
                    PathBuf::from("logbook_new.json"),
                )?
            }
        };

        let entries = runtime.block_on(async {
            storage_manager.list_entries().await
        }).unwrap_or_else(|e| {
            eprintln!("Failed to load entries: {}. Starting with empty log.", e);
            Vec::new()
        });

        Ok(App {
            mode: AppMode::Normal,
            form: Form::new(),
            status_message: Some(("New logbook created".to_string(), false)),
            storage_manager,
            runtime,
            entries,
            selected_index: None,
        })
    }

    pub fn save_entry(&mut self) {
        let frequency = self.form.fields[1].value
            .parse::<f64>()
            .unwrap_or_else(|_| {
                self.set_error("Invalid frequency format");
                0.0
            });

        let entry = LogEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            callsign: self.form.fields[0].value.clone(),
            frequency,
            mode: self.form.fields[2].value.clone(),
            rst_sent: Some(self.form.fields[3].value.clone()),
            rst_received: Some(self.form.fields[4].value.clone()),
            notes: Some(self.form.fields[5].value.clone()),
            ..Default::default()
        };

        let result = self.runtime.block_on(async {
            self.storage_manager.save_entry(entry).await
        });

        match result {
            Ok(_) => {
                self.set_status("Entry saved successfully");
                self.form.reset();
            }
            Err(e) => {
                self.set_error(&format!("Failed to save entry: {}", e));
            }
        }
    }

    pub fn change_storage_format(&mut self, format: StorageFormat, path: PathBuf) -> Result<(), StorageError> {
        let new_storage = StorageManager::new(format, path.clone())?;
        
        let entries = self.runtime.block_on(async {
            self.storage_manager.list_entries().await
        })?;

        self.storage_manager = new_storage;

        self.runtime.block_on(async {
            for entry in entries {
                self.storage_manager.save_entry(entry).await?;
            }
            Ok::<_, StorageError>(())
        })?;

        self.set_status("Storage format changed successfully");
        Ok(())
    }

    // Status handling methods
    pub fn set_status(&mut self, message: &str) {
        self.status_message = Some((message.to_string(), false));
    }

    pub fn set_error(&mut self, message: &str) {
        self.status_message = Some((message.to_string(), true));
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    // Mode handling methods
    pub fn enter_edit_mode(&mut self) {
        self.mode = AppMode::Edit;
    }

    pub fn enter_new_mode(&mut self) {
        self.mode = AppMode::NewEntry;
        self.form.reset();
    }

    pub fn enter_normal_mode(&mut self) {
        self.mode = AppMode::Normal;
        self.form.reset();
        self.clear_status();
    }

    // Form handling methods
    pub fn handle_input(&mut self, c: char) {
        self.form.input(c);
    }

    pub fn handle_backspace(&mut self) {
        self.form.backspace();
    }

    pub fn next_field(&mut self) {
        self.form.next_field();
    }

    pub fn previous_field(&mut self) {
        self.form.previous_field();
    }
    
    pub fn get_entries(&self) -> &Vec<LogEntry> {
        &self.entries
    }

    // Add these methods for selection handling
    pub fn select_next(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        self.selected_index = Some(match self.selected_index {
            Some(i) if i + 1 < self.entries.len() => i + 1,
            Some(_) => 0, // Wrap to start
            None => 0,    // Select first item
        });
    }

    pub fn select_previous(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        self.selected_index = Some(match self.selected_index {
            Some(0) => self.entries.len() - 1, // Wrap to end
            Some(i) => i - 1,
            None => 0, // Select first item
        });
    }

    pub fn edit_selected_entry(&mut self) {
        if let Some(idx) = self.selected_index {
            if let Some(entry) = self.entries.get(idx) {
                // Fill form with selected entry's data
                self.form.fields[0].value = entry.callsign.clone();
                self.form.fields[1].value = entry.frequency.to_string();
                self.form.fields[2].value = entry.mode.clone();
                self.form.fields[3].value = entry.rst_sent.clone().unwrap_or_default();
                self.form.fields[4].value = entry.rst_received.clone().unwrap_or_default();
                self.form.fields[5].value = entry.notes.clone().unwrap_or_default();
                self.mode = AppMode::Edit;
            }
        }
    }

    pub fn get_selected_entry(&self) -> Option<&LogEntry> {
        self.selected_index.and_then(|idx| self.entries.get(idx))
    }

}