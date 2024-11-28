use std::path::PathBuf;
use std::fs;
use async_trait::async_trait;

use crate::LogEntry;
use crate::storage::{Storage, StorageError, StorageFormat};

pub struct JsonStorage {
    path: PathBuf,
    cached_entries: Vec<LogEntry>,
}

impl JsonStorage {
    pub fn new(path: &PathBuf) -> Result<Self, StorageError> {
        let cached_entries = if path.exists() {
            let content = fs::read_to_string(path)?;
            // Handle empty file case
            if content.trim().is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&content).map_err(|e| {
                    // If JSON is invalid, backup the file and start fresh
                    if let Err(backup_err) = fs::rename(path, path.with_extension("json.bak")) {
                        eprintln!("Failed to create backup: {}", backup_err);
                    }
                    StorageError::Json(e)
                })?
            }
        } else {
            // Create directory if it doesn't exist
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            Vec::new()
        };

        Ok(Self {
            path: path.clone(),
            cached_entries,
        })
    }

    fn save_to_file(&self) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(&self.cached_entries)?;
        // Write to temporary file first
        let temp_path = self.path.with_extension("json.tmp");
        fs::write(&temp_path, &json)?;
        // Then rename it to the actual file
        fs::rename(&temp_path, &self.path)?;
        Ok(())
    }
}

#[async_trait]
impl Storage for JsonStorage {
    async fn save_entry(&mut self, entry: LogEntry) -> Result<(), StorageError> {
        // First check if an entry with this ID already exists
        if let Some(pos) = self.cached_entries.iter().position(|e| e.id == entry.id) {
            self.cached_entries[pos] = entry;
        } else {
            self.cached_entries.push(entry);
        }

        self.save_to_file()?;
        Ok(())
    }

    async fn get_entry(&self, id: &str) -> Result<Option<LogEntry>, StorageError> {
        Ok(self.cached_entries
            .iter()
            .find(|e| e.id == id)
            .cloned())
    }

    async fn add_entry(&mut self, entry: LogEntry) -> Result<(), StorageError> {
        Ok(())
    }

    async fn list_entries(&self) -> Result<Vec<LogEntry>, StorageError> {
        Ok(self.cached_entries.clone())
    }

    async fn update_entry(&mut self, entry: LogEntry) -> Result<(), StorageError> {
        if let Some(pos) = self.cached_entries.iter().position(|e| e.id == entry.id) {
            self.cached_entries[pos] = entry;
            self.save_to_file()?;
            Ok(())
        } else {
            Err(StorageError::Backend(format!("Entry with id {} not found", entry.id)))
        }
    }

    async fn delete_entry(&mut self, id: &str) -> Result<(), StorageError> {
        if let Some(pos) = self.cached_entries.iter().position(|e| e.id == id) {
            self.cached_entries.remove(pos);
            self.save_to_file()?;
            Ok(())
        } else {
            Err(StorageError::Backend(format!("Entry with id {} not found", id)))
        }
    }

    async fn clear(&mut self) -> Result<(), StorageError> {
        self.cached_entries.clear();
        self.save_to_file()?;
        Ok(())
    }

    fn format(&self) -> StorageFormat {
        StorageFormat::Json
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    async fn create_test_entry() -> LogEntry {
        LogEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            callsign: "W1AW".to_string(),
            frequency: 14.074,
            mode: "FT8".to_string(),
            rst_sent: Some("599".to_string()),
            rst_received: Some("599".to_string()),
            notes: Some("Test QSO".to_string()),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_json_storage_crud() -> Result<(), StorageError> {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test_log.json");
        
        let mut storage = JsonStorage::new(&path)?;

        // Test Create
        let entry = create_test_entry().await;
        let entry_id = entry.id.clone();
        storage.save_entry(entry).await?;

        // Test Read
        let retrieved = storage.get_entry(&entry_id).await?;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().callsign, "W1AW");

        // Test Update
        let mut updated_entry = create_test_entry().await;
        updated_entry.id = entry_id.clone();
        updated_entry.callsign = "K1ABC".to_string();
        storage.update_entry(updated_entry).await?;

        let retrieved = storage.get_entry(&entry_id).await?;
        assert_eq!(retrieved.unwrap().callsign, "K1ABC");

        // Test Delete
        storage.delete_entry(&entry_id).await?;
        let retrieved = storage.get_entry(&entry_id).await?;
        assert!(retrieved.is_none());

        Ok(())
    }
}