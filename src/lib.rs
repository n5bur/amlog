// Main library exports
pub mod app;
pub mod storage;
pub mod ui;
// pub mod db;

// Re-export main types for convenience
pub use app::App;
pub use app::AppMode;
pub use app::LogEntry;