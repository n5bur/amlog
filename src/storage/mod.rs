//! Storage handling for amateur radio logs
//! Supports multiple backends including JSON, ADIF

mod error;
mod manager;
pub mod formats;
mod types;

pub use error::StorageError;
pub use manager::StorageManager;
pub use types::{Storage, StorageFormat};

// Re-export concrete implementations
pub use formats::json::JsonStorage;
pub use formats::adif::AdifStorage;

// Constants
pub(crate) const DEFAULT_BUFFER_SIZE: usize = 1024;