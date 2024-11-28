use std::path::PathBuf;
use sqlx::Sqlite;
use tokio::sync::Mutex;
use std::sync::Arc;

use super::{
    Storage, StorageError, StorageFormat,
};
use crate::storage::{JsonStorage, AdifStorage, SqliteStorage};


use crate::LogEntry;

pub struct StorageManager {
    storage: Arc<Mutex<Box<dyn Storage>>>,
    format: StorageFormat,
    path: PathBuf,
}

impl StorageManager {
    pub async fn new(format: StorageFormat, path: PathBuf) -> Result<Self, StorageError> {
        let storage: Box<dyn Storage> = match format {
            StorageFormat::Json => Box::new(JsonStorage::new(&path)?),
            StorageFormat::Adif => Box::new(AdifStorage::new(&path)?),
            StorageFormat::Sqlite => Box::new(SqliteStorage::new(&path).await?),
        };

        Ok(Self {
            storage: Arc::new(Mutex::new(storage)),
            format,
            path,
        })
    }

    pub async fn save_entry(&mut self, entry: LogEntry) -> Result<(), StorageError> {
        let mut storage = self.storage.lock().await;
        storage.save_entry(entry).await
    }

    pub async fn list_entries(&self) -> Result<Vec<LogEntry>, StorageError> {
        let storage = self.storage.lock().await;
        storage.list_entries().await
    }

    pub async fn delete_entry(&mut self, id: &str) -> Result<(), StorageError> {
        let mut storage = self.storage.lock().await;
        storage.delete_entry(id).await
    }

    pub async fn add_entry(&mut self, entry: LogEntry) -> Result<(), StorageError> {
        let mut storage = self.storage.lock().await;
        storage.add_entry(entry).await
    }
    pub async fn export_adif(&self) -> Result<String, StorageError> {
        let entries = self.list_entries().await?;
        
        match self.format {
            StorageFormat::Adif => {
                let storage = self.storage.lock().await;
                // Since we're already using ADIF storage, just read the file
                std::fs::read_to_string(storage.path())
                    .map_err(StorageError::Io)
            },
            _ => {
                // Convert entries to ADIF format
                // let adif_storage = AdifStorage::new(&PathBuf::from(""))?;
                Ok(AdifStorage::entries_to_adif(&entries))
            }
        }
    }

    pub async fn import_adif(&mut self, content: &str) -> Result<(), StorageError> {
        // let temp_storage = AdifStorage::new(&PathBuf::from(""))?;
        let entries = AdifStorage::adif_to_entries(content);

        let mut storage = self.storage.lock().await;
        if let Ok(entries) = entries {
            for log_entry in entries {
                storage.save_entry(log_entry).await?;
            }
        }

        Ok(())
    }

    pub fn get_format(&self) -> StorageFormat {
        self.format
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}