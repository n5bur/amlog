use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;
use std::path::Path;

pub async fn initialize_db(db_path: &Path) -> Result<Pool<Sqlite>, sqlx::Error> {
    // Convert path to database URL
    let db_url = format!("sqlite:{}", db_path.display());

    // Create database if it doesn't exist
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await?;
    }

    // Create connection pool with some sensible defaults
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .after_connect(|conn, _| Box::pin(async move {
            sqlx::query!("PRAGMA foreign_keys = ON").execute(conn).await?;
            Ok(())
        }))
        .connect(&db_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tempfile::TempDir;

    pub async fn setup_test_db() -> (TempDir, Pool<Sqlite>) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let pool = initialize_db(&db_path).await.unwrap();
        (temp_dir, pool)
    }
}

// Optional: Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub max_connections: u32,
    pub enable_wal: bool,
    pub journal_mode: JournalMode,
    pub foreign_keys: bool,
}

#[derive(Debug, Clone)]
pub enum JournalMode {
    Delete,
    Truncate,
    Persist,
    Memory,
    Wal,
    Off,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            max_connections: 5,
            enable_wal: true,
            journal_mode: JournalMode::Wal,
            foreign_keys: true,
        }
    }
}

// Helper function for applying database configuration
async fn apply_config(pool: &Pool<Sqlite>, config: &DatabaseConfig) -> Result<(), sqlx::Error> {
    let mut conn = pool.acquire().await?;

    if config.enable_wal {
        sqlx::query("PRAGMA journal_mode=WAL").execute(&mut *conn).await?;
    }

    if config.foreign_keys {
        sqlx::query!("PRAGMA foreign_keys=ON").execute(&mut *conn).await?;
    }

    // Set journal mode
    let journal_mode = match config.journal_mode {
        JournalMode::Delete => "DELETE",
        JournalMode::Truncate => "TRUNCATE",
        JournalMode::Persist => "PERSIST",
        JournalMode::Memory => "MEMORY",
        JournalMode::Wal => "WAL",
        JournalMode::Off => "OFF",
    };
    let pragma_query = format!("PRAGMA journal_mode={}", journal_mode);
    sqlx::query(&pragma_query)
        .execute(&mut *conn)
        .await?;

    Ok(())
}

pub async fn initialize_db_with_config(
    db_path: &Path,
    config: DatabaseConfig,
) -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = initialize_db(db_path).await?;
    apply_config(&pool, &config).await?;
    Ok(pool)
}