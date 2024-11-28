use sqlx;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("SQL error: {0}")]
    Sqlx(sqlx::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("ADIF error: {0}")]
    Adif(String),

    #[error("Data validation error: {0}")]
    Validation(String),

    #[error("Storage backend error: {0}")]
    Backend(String),

    #[error("Entry not found: {0}")]
    NotFound(String),

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Entry already exists")]
    EntryExists,

    #[error("Parse error: {0}")]
    ParseError(String),

}

impl From<sqlx::Error> for StorageError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => StorageError::NotFound("Entry not found".to_string()),
            sqlx::Error::Database(db_error) => {
                StorageError::Database(db_error.message().to_string())
            }
            _ => StorageError::Database(error.to_string()),
        }
    }
}

// If you're using sqlx::migrate::MigrateError separately
impl From<sqlx::migrate::MigrateError> for StorageError {
    fn from(error: sqlx::migrate::MigrateError) -> Self {
        StorageError::Migration(error.to_string())
    }
}
