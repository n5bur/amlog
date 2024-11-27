use async_trait::async_trait;
use std::path::PathBuf;
use crate::LogEntry;
use super::StorageError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StorageFormat {
    Json,
    Adif,
}

#[async_trait]
pub trait Storage: Send + Sync {
    /// Save a new log entry
    async fn save_entry(&mut self, entry: LogEntry) -> Result<(), StorageError>;

    /// Retrieve a specific entry by ID
    async fn get_entry(&self, id: &str) -> Result<Option<LogEntry>, StorageError>;

    /// List all entries
    async fn list_entries(&self) -> Result<Vec<LogEntry>, StorageError>;

    /// Update an existing entry
    async fn update_entry(&mut self, entry: LogEntry) -> Result<(), StorageError>;

    /// Delete an entry
    async fn delete_entry(&mut self, id: &str) -> Result<(), StorageError>;

    /// Clear all entries
    async fn clear(&mut self) -> Result<(), StorageError>;

    /// Get the storage format
    fn format(&self) -> StorageFormat;

    /// Get the storage path
    fn path(&self) -> &PathBuf;
}

pub trait StorageValidator {
    fn validate_entry(&self, entry: &LogEntry) -> Result<(), StorageError>;
}