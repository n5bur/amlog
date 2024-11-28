use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqlitePool};
use sqlx::{ConnectOptions, Row};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::storage::{Storage, StorageError, StorageFormat};
use crate::LogEntry;

pub struct SqliteStorage {
    pool: SqlitePool,
    path: PathBuf,
}

impl SqliteStorage {
    pub async fn new(path: &PathBuf) -> Result<Self, StorageError> {
        // Create SqliteConnectOptions and set create_if_missing to true
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);

        // Optionally, set additional options
        // For example, set logging level if needed
        // .log_statements(tracing::log::LevelFilter::Debug);

        // Establish the connection pool with the options
        let pool = SqlitePoolOptions::new()
            .max_connections(5) // Adjust as needed
            .connect_with(options)
            .await
            .map_err(StorageError::Sqlx)?;

        // Initialize database schema if necessary
        initialize_db(&pool).await?;

        Ok(Self {
            pool,
            path: path.clone(),
        })
    }
}

async fn initialize_db(pool: &SqlitePool) -> Result<(), StorageError> {
    // Create the table with the updated schema
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS log_entries (
            id TEXT PRIMARY KEY,
            timestamp TEXT NOT NULL,
            callsign TEXT NOT NULL,
            frequency REAL NOT NULL,
            mode TEXT NOT NULL,
            rst_sent TEXT,
            rst_received TEXT,
            notes TEXT,
            name TEXT,
            qth TEXT,
            state TEXT,
            country TEXT,
            dxcc INTEGER,
            band TEXT,
            operator TEXT,
            grid TEXT,
            power REAL,
            custom_fields TEXT
        )
        ",
    )
    .execute(pool)
    .await
    .map_err(StorageError::Sqlx)?;
    Ok(())
}


#[async_trait]
impl Storage for SqliteStorage {
    async fn get_entry(&self, id: &str) -> Result<Option<LogEntry>, StorageError> {
        let row = sqlx::query("SELECT * FROM log_entries WHERE id = ?1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(StorageError::Sqlx)?;

        if let Some(row) = row {
            let id: String = row.try_get("id")?;
            let timestamp_str: String = row.try_get("timestamp")?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| StorageError::ParseError(e.to_string()))?
                .with_timezone(&Utc);
            let callsign: String = row.try_get("callsign")?;
            let frequency: f64 = row.try_get("frequency")?;
            let mode: String = row.try_get("mode")?;
            let rst_sent: Option<String> = row.try_get("rst_sent")?;
            let rst_received: Option<String> = row.try_get("rst_received")?;
            let notes: Option<String> = row.try_get("notes")?;
            let name: Option<String> = row.try_get("name")?;
            let qth: Option<String> = row.try_get("qth")?;
            let state: Option<String> = row.try_get("state")?;
            let country: Option<String> = row.try_get("country")?;
            let dxcc: Option<i64> = row.try_get("dxcc")?;
            let band: Option<String> = row.try_get("band")?;
            let operator: Option<String> = row.try_get("operator")?;
            let grid: Option<String> = row.try_get("grid")?;
            let power: Option<f32> = row.try_get("power")?;
            let custom_fields_json: Option<String> = row.try_get("custom_fields")?;
            let custom_fields: HashMap<String, String> = custom_fields_json
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default();

            Ok(Some(LogEntry {
                id,
                timestamp,
                callsign,
                frequency,
                mode,
                rst_sent,
                rst_received,
                notes,
                name,
                qth,
                state,
                country,
                dxcc: dxcc.map(|v| v as u32),
                band,
                operator,
                grid,
                power,
                custom_fields,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_entry(&mut self, entry: LogEntry) -> Result<(), StorageError> {
        sqlx::query(
            "
            UPDATE log_entries SET
                timestamp = ?2,
                callsign = ?3,
                frequency = ?4,
                mode = ?5,
                rst_sent = ?6,
                rst_received = ?7,
                notes = ?8,
                name = ?9,
                qth = ?10,
                state = ?11,
                country = ?12,
                dxcc = ?13,
                band = ?14,
                operator = ?15,
                grid = ?16,
                power = ?17,
                custom_fields = ?18
            WHERE id = ?1
            ",
        )
        .bind(&entry.id)
        .bind(entry.timestamp.to_rfc3339())
        .bind(&entry.callsign)
        .bind(entry.frequency)
        .bind(&entry.mode)
        .bind(&entry.rst_sent)
        .bind(&entry.rst_received)
        .bind(&entry.notes)
        .bind(&entry.name)
        .bind(&entry.qth)
        .bind(&entry.state)
        .bind(&entry.country)
        .bind(entry.dxcc)
        .bind(&entry.band)
        .bind(&entry.operator)
        .bind(&entry.grid)
        .bind(entry.power)
        .bind(serde_json::to_string(&entry.custom_fields).unwrap_or_else(|_| "{}".to_string()))
        .execute(&self.pool)
        .await
        .map_err(StorageError::Sqlx)?;
        Ok(())
    }

    async fn clear(&mut self) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM log_entries")
            .execute(&self.pool)
            .await
            .map_err(StorageError::Sqlx)?;
        Ok(())
    }

    fn format(&self) -> StorageFormat {
        StorageFormat::Sqlite
    }
    async fn save_entry(&mut self, entry: LogEntry) -> Result<(), StorageError> {
        // Check if the entry exists
        let existing = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM log_entries WHERE id = ?1")
            .bind(&entry.id)
            .fetch_one(&self.pool)
            .await
            .map_err(StorageError::Sqlx)?;

        if existing > 0 {
            // Update existing entry
            sqlx::query(
                "
                UPDATE log_entries SET
                    timestamp = ?2,
                    callsign = ?3,
                    frequency = ?4,
                    mode = ?5,
                    rst_sent = ?6,
                    rst_received = ?7,
                    notes = ?8,
                    name = ?9,
                    qth = ?10,
                    state = ?11,
                    country = ?12,
                    dxcc = ?13,
                    band = ?14,
                    operator = ?15,
                    grid = ?16,
                    power = ?17,
                    custom_fields = ?18
                WHERE id = ?1
                ",
            )
            .bind(&entry.id)
            .bind(entry.timestamp.to_rfc3339())
            .bind(&entry.callsign)
            .bind(entry.frequency)
            .bind(&entry.mode)
            .bind(&entry.rst_sent)
            .bind(&entry.rst_received)
            .bind(&entry.notes)
            .bind(&entry.name)
            .bind(&entry.qth)
            .bind(&entry.state)
            .bind(&entry.country)
            .bind(entry.dxcc)
            .bind(&entry.band)
            .bind(&entry.operator)
            .bind(&entry.grid)
            .bind(entry.power)
            .bind(serde_json::to_string(&entry.custom_fields).unwrap_or_else(|_| "{}".to_string()))
            .execute(&self.pool)
            .await
            .map_err(StorageError::Sqlx)?;
        } else {
            // Insert new entry
            sqlx::query(
                "
                INSERT INTO log_entries (
                    id, timestamp, callsign, frequency, mode, rst_sent, rst_received,
                    notes, name, qth, state, country, dxcc, band, operator, grid,
                    power, custom_fields
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)
                ",
            )
            .bind(&entry.id)
            .bind(entry.timestamp.to_rfc3339())
            .bind(&entry.callsign)
            .bind(entry.frequency)
            .bind(&entry.mode)
            .bind(&entry.rst_sent)
            .bind(&entry.rst_received)
            .bind(&entry.notes)
            .bind(&entry.name)
            .bind(&entry.qth)
            .bind(&entry.state)
            .bind(&entry.country)
            .bind(entry.dxcc)
            .bind(&entry.band)
            .bind(&entry.operator)
            .bind(&entry.grid)
            .bind(entry.power)
            .bind(serde_json::to_string(&entry.custom_fields).unwrap_or_else(|_| "{}".to_string()))
            .execute(&self.pool)
            .await
            .map_err(StorageError::Sqlx)?;
        }
        Ok(())
    }
    
    async fn list_entries(&self) -> Result<Vec<LogEntry>, StorageError> {
        let rows = sqlx::query("SELECT * FROM log_entries")
            .fetch_all(&self.pool)
            .await
            .map_err(StorageError::Sqlx)?;

        let mut entries = Vec::new();
        for row in rows {
            let id: String = row.try_get("id")?;
            let timestamp_str: String = row.try_get("timestamp")?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| StorageError::ParseError(e.to_string()))?
                .with_timezone(&Utc);
            let callsign: String = row.try_get("callsign")?;
            let frequency: f64 = row.try_get("frequency")?;
            let mode: String = row.try_get("mode")?;
            let rst_sent: Option<String> = row.try_get("rst_sent")?;
            let rst_received: Option<String> = row.try_get("rst_received")?;
            let notes: Option<String> = row.try_get("notes")?;
            let name: Option<String> = row.try_get("name")?;
            let qth: Option<String> = row.try_get("qth")?;
            let state: Option<String> = row.try_get("state")?;
            let country: Option<String> = row.try_get("country")?;
            let dxcc: Option<i64> = row.try_get("dxcc")?;
            let band: Option<String> = row.try_get("band")?;
            let operator: Option<String> = row.try_get("operator")?;
            let grid: Option<String> = row.try_get("grid")?;
            let power: Option<f32> = row.try_get("power")?;
            let custom_fields_json: Option<String> = row.try_get("custom_fields")?;
            let custom_fields: HashMap<String, String> = custom_fields_json
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default();

            entries.push(LogEntry {
                id,
                timestamp,
                callsign,
                frequency,
                mode,
                rst_sent,
                rst_received,
                notes,
                name,
                qth,
                state,
                country,
                dxcc: dxcc.map(|v| v as u32),
                band,
                operator,
                grid,
                power,
                custom_fields,
            });
        }
        Ok(entries)
    }

    async fn delete_entry(&mut self, id: &str) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM log_entries WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(StorageError::Sqlx)?;
        Ok(())
    }

    async fn add_entry(&mut self, entry: LogEntry) -> Result<(), StorageError> {
        // Ensure entry doesn't already exist
        let existing = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM log_entries WHERE id = ?1")
            .bind(&entry.id)
            .fetch_one(&self.pool)
            .await
            .map_err(StorageError::Sqlx)?;

        if existing > 0 {
            return Err(StorageError::EntryExists);
        }

        // Insert new entry
        sqlx::query(
            "
            INSERT INTO log_entries (
                id, timestamp, callsign, frequency, mode, rst_sent, rst_received,
                notes, name, qth, state, country, dxcc, band, operator, grid,
                power, custom_fields
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)
            ",
        )
        .bind(&entry.id)
        .bind(entry.timestamp.to_rfc3339())
        .bind(&entry.callsign)
        .bind(entry.frequency)
        .bind(&entry.mode)
        .bind(&entry.rst_sent)
        .bind(&entry.rst_received)
        .bind(&entry.notes)
        .bind(&entry.name)
        .bind(&entry.qth)
        .bind(&entry.state)
        .bind(&entry.country)
        .bind(entry.dxcc)
        .bind(&entry.band)
        .bind(&entry.operator)
        .bind(&entry.grid)
        .bind(entry.power)
        .bind(serde_json::to_string(&entry.custom_fields).unwrap_or_else(|_| "{}".to_string()))
        .execute(&self.pool)
        .await
        .map_err(StorageError::Sqlx)?;

        Ok(())
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }
}
