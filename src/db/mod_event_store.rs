use std::sync::Arc;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Result as SqlResult, Row, params};

use crate::db::mod_event::ModEvent;

/// Trait for storing and retrieving mod events.
pub trait ModEventStore: Send + Sync {
    /// Record a new mod event.
    fn record_event(&self, event: &ModEvent) -> SqlResult<()>;

    /// Get all events for a given mod_id, ordered by timestamp.
    fn get_mod_history(&self, mod_id: &str) -> SqlResult<Vec<ModEvent>>;

    /// Get the latest event for each mod (current state).
    fn get_latest_events(&self) -> SqlResult<Vec<ModEvent>>;

    /// Get all events in the database.
    fn get_all_events(&self) -> SqlResult<Vec<ModEvent>>;
}

/// SQLite-backed implementation of ModEventStore.
pub struct SqliteModEventStore {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl SqliteModEventStore {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }

    fn conn(&self) -> SqlResult<PooledConnection<SqliteConnectionManager>> {
        self.pool
            .get()
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))
    }

    /// Bulk insert mod events in a transaction.
    pub fn record_events(&self, events: &[ModEvent]) -> SqlResult<()> {
        let mut conn = self.conn()?;
        let tx = conn.transaction()?;
        for event in events {
            tx.execute(
                    r#"
                    INSERT INTO mod_events (
                        mod_id, source, steam_app_id, event_type, timestamp, version, path, name, authors, created, modified
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                    "#,
                    params![
                        event.mod_id,
                        event.serialize_source(),
                        event.serialize_steam_app_id(),
                        event.event_type,
                        event.timestamp,
                        event.version,
                        event.path,
                        event.name,
                        event.authors,
                        event.created,
                        event.modified,
                    ],
                )?;
        }
        tx.commit()?;
        Ok(())
    }

    fn row_to_event(row: &Row) -> SqlResult<ModEvent> {
        Ok(ModEvent {
            event_id: row.get("event_id")?,
            mod_id: row.get("mod_id")?,
            source: ModEvent::parse_source(&row.get::<_, String>("source")?),
            steam_app_id: row.get::<_, Option<String>>("steam_app_id")?,
            event_type: row.get("event_type")?,
            timestamp: row.get("timestamp")?,
            version: row.get::<_, Option<String>>("version")?,
            path: row.get("path")?,
            name: row.get("name")?,
            authors: row.get::<_, Option<String>>("authors")?,
            created: row.get::<_, Option<String>>("created")?,
            modified: row.get::<_, Option<String>>("modified")?,
        })
    }
}

impl ModEventStore for SqliteModEventStore {
    fn record_event(&self, event: &ModEvent) -> SqlResult<()> {
        let conn = self.conn()?;
        conn.execute(
            r#"
            INSERT INTO mod_events (
                mod_id, source, steam_app_id, event_type, timestamp, version, path, name, authors, created, modified
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            params![
                event.mod_id,
                event.serialize_source(),
                event.serialize_steam_app_id(),
                event.event_type,
                event.timestamp,
                event.version,
                event.path,
                event.name,
                event.authors,
                event.created,
                event.modified,
            ],
        )?;
        Ok(())
    }

    fn get_mod_history(&self, mod_id: &str) -> SqlResult<Vec<ModEvent>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT * FROM mod_events
            WHERE mod_id = ?1
            ORDER BY timestamp ASC
            "#,
        )?;
        let events = stmt
            .query_map(params![mod_id], Self::row_to_event)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(events)
    }

    fn get_latest_events(&self) -> SqlResult<Vec<ModEvent>> {
        let conn = self.conn()?;
        // Subquery to get the latest event_id for each mod_id
        let mut stmt = conn.prepare(
            r#"
            SELECT e.*
            FROM mod_events e
            INNER JOIN (
                SELECT mod_id, MAX(timestamp) as max_ts
                FROM mod_events
                GROUP BY mod_id
            ) latest
            ON e.mod_id = latest.mod_id AND e.timestamp = latest.max_ts
            "#,
        )?;
        let events = stmt
            .query_map([], Self::row_to_event)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(events)
    }

    fn get_all_events(&self) -> SqlResult<Vec<ModEvent>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            r#"
            SELECT * FROM mod_events
            ORDER BY timestamp ASC
            "#,
        )?;
        let events = stmt
            .query_map([], Self::row_to_event)?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(events)
    }
}
