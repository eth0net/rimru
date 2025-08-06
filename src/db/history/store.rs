use std::{convert::TryFrom, sync::Arc};

use anyhow::Context;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Error::FromSqlConversionFailure, Row, params, types::Type::Text};

use crate::game::mods::Source;

use super::{Event, EventType};

/// Trait for storing and retrieving mod events.
pub trait HistoryStore: Send + Sync {
    /// Record new mod events.
    fn record_events(&self, events: &[Event]) -> anyhow::Result<()>;

    /// Get all events for a given mod_id, ordered by timestamp.
    fn get_mod_history(&self, mod_id: &str) -> anyhow::Result<Vec<Event>>;

    /// Get the latest event for each mod (current state).
    fn get_latest_events(&self) -> anyhow::Result<Vec<Event>>;

    /// Get all events in the database.
    fn get_all_events(&self) -> anyhow::Result<Vec<Event>>;
}

/// SQLite-backed implementation of HistoryStore.
pub struct SqliteHistoryStore {
    pool: Arc<Pool<SqliteConnectionManager>>,
}

impl SqliteHistoryStore {
    pub fn new(pool: Arc<Pool<SqliteConnectionManager>>) -> Self {
        Self { pool }
    }

    fn conn(&self) -> anyhow::Result<PooledConnection<SqliteConnectionManager>> {
        self.pool
            .get()
            .with_context(|| "Failed to get SQLite connection from pool")
    }

    fn row_to_event(row: &Row) -> rusqlite::Result<Event> {
        let event_type_str = row.get::<_, String>("event_type")?;
        let event_type = match EventType::try_from(event_type_str.as_str()) {
            Ok(et) => et,
            Err(e) => {
                log::error!("{e}: {event_type_str}");
                return Err(FromSqlConversionFailure(0, Text, e.into_boxed_dyn_error()));
            }
        };

        let source_str = row.get::<_, String>("source")?;
        let source = match Source::try_from(source_str.as_str()) {
            Ok(src) => src,
            Err(e) => {
                log::error!("{e}: {source_str}");
                Source::Unknown
            }
        };

        Ok(Event {
            event_id: row.get("event_id")?,
            event_type,
            timestamp: row.get("timestamp")?,
            mod_id: row.get("mod_id")?,
            name: row.get("name")?,
            version: row.get::<_, Option<String>>("version")?,
            authors: row.get::<_, Option<String>>("authors")?,
            steam_app_id: row.get::<_, Option<String>>("steam_app_id")?,
            path: row.get("path")?,
            source,
            created: row.get::<_, Option<String>>("created")?,
            modified: row.get::<_, Option<String>>("modified")?,
        })
    }
}

impl HistoryStore for SqliteHistoryStore {
    /// Bulk insert mod events in a transaction.
    fn record_events(&self, events: &[Event]) -> anyhow::Result<()> {
        let mut conn = self
            .conn()
            .context("Failed to get DB connection for record_events")?;
        let tx = conn
            .transaction()
            .context("Failed to start transaction for record_events")?;
        for event in events {
            tx.execute(
                r#"
                    INSERT INTO history (
                        event_type,
                        timestamp,
                        mod_id,
                        name,
                        version,
                        authors,
                        steam_app_id,
                        path,
                        source,
                        created,
                        modified
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                    "#,
                params![
                    event.event_type,
                    event.timestamp,
                    event.mod_id,
                    event.name,
                    event.version,
                    event.authors,
                    event.steam_app_id,
                    event.path,
                    event.source,
                    event.created,
                    event.modified,
                ],
            )
            .with_context(|| format!("Failed to insert event into history: {event:?}"))?;
        }
        tx.commit()
            .context("Failed to commit transaction for record_events")?;
        Ok(())
    }

    fn get_mod_history(&self, mod_id: &str) -> anyhow::Result<Vec<Event>> {
        let conn = self
            .conn()
            .context("Failed to get DB connection for get_mod_history")?;
        let mut stmt = conn
            .prepare(
                r#"
            SELECT * FROM history
            WHERE mod_id = ?1
            ORDER BY timestamp ASC
            "#,
            )
            .with_context(|| {
                format!("Failed to prepare statement for get_mod_history, mod_id: {mod_id}")
            })?;
        let events = stmt
            .query_map(params![mod_id], Self::row_to_event)
            .with_context(|| format!("Failed to query events for mod_id: {mod_id}"))?
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| format!("Failed to collect events for mod_id: {mod_id}"))?;
        Ok(events)
    }

    fn get_latest_events(&self) -> anyhow::Result<Vec<Event>> {
        let conn = self
            .conn()
            .context("Failed to get DB connection for get_latest_events")?;
        // Subquery to get the latest event_id for each mod_id
        let mut stmt = conn
            .prepare(
                r#"
            SELECT e.*
            FROM history e
            INNER JOIN (
                SELECT mod_id, MAX(timestamp) as max_ts
                FROM history
                GROUP BY mod_id
            ) latest
            ON e.mod_id = latest.mod_id AND e.timestamp = latest.max_ts
            "#,
            )
            .context("Failed to prepare statement for get_latest_events")?;
        let events = stmt
            .query_map([], Self::row_to_event)
            .context("Failed to query latest events")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect latest events")?;
        Ok(events)
    }

    fn get_all_events(&self) -> anyhow::Result<Vec<Event>> {
        let conn = self
            .conn()
            .context("Failed to get DB connection for get_all_events")?;
        let mut stmt = conn
            .prepare(
                r#"
            SELECT * FROM history
            ORDER BY timestamp ASC
            "#,
            )
            .context("Failed to prepare statement for get_all_events")?;
        let events = stmt
            .query_map([], Self::row_to_event)
            .context("Failed to query all events")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect all events")?;
        Ok(events)
    }
}
