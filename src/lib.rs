//! Amateur Radio Logging Application
//! 
//! This library provides functionality for managing amateur radio contacts
//! with support for multiple storage formats and a terminal user interface.

pub mod app;
pub mod storage;
pub mod ui;

// Re-export main types for convenience
pub use app::App;
pub use storage::StorageManager;
pub use app::LogEntry;