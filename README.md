```sql
-- Enable SQLite foreign keys
PRAGMA foreign_keys = ON;

-- Core QSO table with required fields
CREATE TABLE qsos (
    id TEXT PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Required ADIF fields
    callsign TEXT NOT NULL,
    qso_date DATE NOT NULL,  -- YYYYMMDD format
    time_on TEXT NOT NULL,   -- HHMMSS format
    band TEXT NOT NULL,      -- From band table
    mode TEXT NOT NULL,      -- From mode table
    
    -- Common optional fields with indexes
    frequency DECIMAL(10,4),
    rst_sent TEXT,
    rst_received TEXT,
    grid_square TEXT,
    operator TEXT,
    power DECIMAL(10,2),
    name TEXT,
    qth TEXT,
    state TEXT,
    country TEXT,
    dxcc INTEGER,           -- DXCC entity code
    
    -- Metadata
    source TEXT,            -- Application that created the record
    plugin_version TEXT,    -- Plugin version if created by plugin
    
    -- Indexes
    CONSTRAINT idx_qso_basic UNIQUE (callsign, qso_date, time_on)
);

-- Reference tables
CREATE TABLE bands (
    name TEXT PRIMARY KEY NOT NULL,
    lower_freq DECIMAL(10,4),
    upper_freq DECIMAL(10,4),
    description TEXT
);

CREATE TABLE modes (
    name TEXT PRIMARY KEY NOT NULL,
    description TEXT
);

-- Custom fields table for plugin extensibility
CREATE TABLE custom_fields (
    qso_id TEXT NOT NULL,
    field_name TEXT NOT NULL,
    field_value TEXT,
    field_type TEXT NOT NULL,  -- Data type (text, number, date, etc)
    plugin_id TEXT NOT NULL,   -- Plugin that owns this field
    
    PRIMARY KEY (qso_id, field_name),
    FOREIGN KEY (qso_id) REFERENCES qsos(id) ON DELETE CASCADE
);

-- Plugin registration table
CREATE TABLE plugins (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    config JSON,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Database version for migrations
CREATE TABLE schema_versions (
    version INTEGER PRIMARY KEY NOT NULL,
    applied_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT
);

-- Triggers for updated_at
CREATE TRIGGER update_qsos_timestamp 
AFTER UPDATE ON qsos
BEGIN
    UPDATE qsos SET updated_at = CURRENT_TIMESTAMP 
    WHERE id = NEW.id;
END;

-- Indexes for common queries
CREATE INDEX idx_qsos_callsign ON qsos(callsign);
CREATE INDEX idx_qsos_date ON qsos(qso_date);
CREATE INDEX idx_qsos_band ON qsos(band);
CREATE INDEX idx_qsos_mode ON qsos(mode);
CREATE INDEX idx_qsos_grid ON qsos(grid_square);
CREATE INDEX idx_qsos_dxcc ON qsos(dxcc);
CREATE INDEX idx_custom_fields_lookup ON custom_fields(field_name, field_value);
```