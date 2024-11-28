// src/app/mod.rs
mod form;
mod state;

pub use form::{Form, FormField};
pub use state::{AppMode, LogEntry, DeletedEntry};
use chrono::{Utc, DateTime};
use std::path::PathBuf;
use tokio::runtime::Runtime;
use uuid::Uuid;
use std::env;
use std::fs;
use dirs;

use crate::storage::{StorageManager, StorageFormat, StorageError};

use std::collections::HashMap;

/// Main application state container
pub struct App {
    pub mode: AppMode,
    pub form: Form,
    pub status_message: Option<(String, bool)>, // (message, is_error)
    entries: Vec<LogEntry>,
    storage_manager: StorageManager,
    runtime: Runtime,
    selected_index: Option<usize>,
    pub deleted_entries: Vec<DeletedEntry>,
    editing_index: Option<usize>, // Track the index of the entry being edited
}

impl App {
    pub fn new() -> Result<Self, StorageError> {
        let runtime = Runtime::new()
            .map_err(|e| StorageError::Backend(format!("Failed to create runtime: {}", e)))?;

        // Get the XDG data directory
        let xdg_data_home = dirs::data_dir()
            .ok_or_else(|| StorageError::Backend("Failed to get XDG_DATA_HOME directory".to_string()))?;

        // Create the application-specific data directory under XDG_DATA_HOME
        let app_data_dir = xdg_data_home.join("amlog");

        // Ensure the directory exists, creating it if necessary
        fs::create_dir_all(&app_data_dir)
            .map_err(|e| StorageError::Backend(format!("Failed to create data directory: {}", e)))?;

        // Create the database path
        let db_path = app_data_dir.join("logbook.db");

        println!("Database path: {:?}", db_path);

        let storage_manager = runtime.block_on(async {
            StorageManager::new(
                StorageFormat::Sqlite,
                db_path.clone(),
            ).await
        })?;

        let entries = runtime.block_on(async {
            storage_manager.list_entries().await
        }).unwrap_or_else(|e| {
            eprintln!("Failed to load entries: {}. Starting with empty log.", e);
            Vec::new()
        });

        Ok(App {
            mode: AppMode::Normal,
            form: Form::new(),
            status_message: Some(("amlog".to_string(), false)),
            storage_manager,
            runtime,
            entries,
            selected_index: None,
            deleted_entries: Vec::new(),
            editing_index: None,
        })
    }

    pub fn save_entry(&mut self) {
        let frequency = match self.form.fields[1].value.parse::<f64>() {
            Ok(freq) => freq,
            Err(_) => {
                self.set_error("Invalid frequency format");
                return;
            }
        };

        let mut entry = LogEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            callsign: self.form.fields[0].value.clone(),
            frequency,
            mode: self.form.fields[2].value.clone(),
            rst_sent: Some(self.form.fields[3].value.clone()),
            rst_received: Some(self.form.fields[4].value.clone()),
            notes: Some(self.form.fields[5].value.clone()),
            // Initialize other fields as needed
            ..Default::default()
        };

        let result = if self.mode == AppMode::Edit {
            // Editing an existing entry
            if let Some(idx) = self.editing_index {
                if let Some(existing_entry) = self.entries.get(idx) {
                    // Keep the same ID and timestamp
                    entry.id = existing_entry.id.clone();
                    entry.timestamp = existing_entry.timestamp;
                }

                self.runtime.block_on(async {
                    self.storage_manager.save_entry(entry.clone()).await
                })
            } else {
                self.set_error("No entry selected for editing");
                return;
            }
        } else {
            // Adding a new entry
            self.runtime.block_on(async {
                self.storage_manager.add_entry(entry.clone()).await
            })
        };

        match result {
            Ok(_) => {
                if self.mode == AppMode::Edit {
                    // Update the existing entry in the entries vector
                    if let Some(idx) = self.editing_index {
                        self.entries[idx] = entry;
                    }
                    self.set_status("Entry updated successfully");
                    self.editing_index = None;
                } else {
                    // Add the new entry to the entries vector
                    self.entries.push(entry);
                    self.set_status("Entry saved successfully");
                }
                self.form.reset();
                self.mode = AppMode::Normal;
            }
            Err(e) => {
                self.set_error(&format!("Failed to save entry: {:?}", e));
            }
        }
    }

    pub fn change_storage_format(&mut self, format: StorageFormat, path: PathBuf) -> Result<(), StorageError> {
        let new_storage = self.runtime.block_on(async {
            StorageManager::new(format, path.clone()).await
        })?;

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

    pub fn delete_current_entry(&mut self) -> Result<(), StorageError> {
        if let Some(selected_idx) = self.selected_index {
            if selected_idx < self.entries.len() {
                // Store the deleted entry for potential undo
                let deleted = DeletedEntry {
                    entry: self.entries[selected_idx].clone(),
                    index: selected_idx,
                    timestamp: Utc::now(),
                };

                // Remove from storage
                let entry_id = &self.entries[selected_idx].id;
                self.runtime.block_on(async {
                    self.storage_manager.delete_entry(entry_id).await
                })?;

                // Remove from UI list and store in undo buffer
                self.entries.remove(selected_idx);
                self.deleted_entries.push(deleted);

                // Adjust selection if needed
                if self.entries.is_empty() {
                    self.selected_index = None;
                } else if selected_idx >= self.entries.len() {
                    self.selected_index = Some(self.entries.len() - 1);
                }

                self.set_status("Entry deleted. Press 'u' to undo.");
            }
        }
        Ok(())
    }

    pub fn undo_delete(&mut self) -> Result<(), StorageError> {
        if let Some(deleted) = self.deleted_entries.pop() {
            // Add back to storage
            self.runtime.block_on(async {
                self.storage_manager.save_entry(deleted.entry.clone()).await
            })?;

            // Insert back into UI list at original position
            let insert_pos = deleted.index.min(self.entries.len());
            self.entries.insert(insert_pos, deleted.entry);
            self.selected_index = Some(insert_pos);

            self.set_status("Delete undone");
        } else {
            self.set_status("No deletions to undo");
        }
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
        self.editing_index = None;
    }

    pub fn enter_normal_mode(&mut self) {
        self.mode = AppMode::Normal;
        self.form.reset();
        self.clear_status();
        self.editing_index = None;
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

    // Selection handling
    pub fn select_next(&mut self) {
        let len = self.entries.len();
        if len == 0 {
            return;
        }

        self.selected_index = Some(match self.selected_index {
            Some(i) if i + 1 < len => i + 1,
            Some(_) => 0,
            None => 0,
        });
    }

    pub fn select_previous(&mut self) {
        let len = self.entries.len();
        if len == 0 {
            return;
        }

        self.selected_index = Some(match self.selected_index {
            Some(0) | None => len - 1,
            Some(i) => i - 1,
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
                self.editing_index = Some(idx);
            }
        }
    }

    pub fn get_selected_entry(&self) -> Option<&LogEntry> {
        self.selected_index.and_then(|idx| self.entries.get(idx))
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }
}
