use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result as SqlResult};

pub mod history;

/// Returns the application's data directory (platform-specific).
pub fn data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("failed to get data directory")
        .join("rimru")
}

/// Returns the path to the application's SQLite database file.
pub fn database_file() -> PathBuf {
    data_dir().join("rimru.sqlite3")
}

/// Type alias for the connection pool.
pub type DbPool = Pool<SqliteConnectionManager>;
pub type SharedDbPool = Arc<DbPool>;

/// Initialize the SQLite database file and return a connection pool.
/// Ensures the parent directory exists before opening the database.
pub fn create_pool<P: AsRef<Path>>(db_path: P) -> DbPool {
    let db_path = db_path.as_ref();

    // Ensure parent directory exists
    if let Some(parent) = db_path.parent()
        && let Err(e) = fs::create_dir_all(parent)
    {
        panic!("Failed to create database directory {parent:?}: {e}");
    }

    let manager = SqliteConnectionManager::file(db_path);
    Pool::new(manager).expect("Failed to create SQLite connection pool")
}

/// Initialize the database: create pool, run migrations, and return shared pool.
pub fn init() -> SharedDbPool {
    let db_path = database_file();
    let pool = Arc::new(create_pool(&db_path));
    // Run migrations on a direct connection (not pooled)
    {
        let conn =
            Connection::open(&db_path).expect("Failed to open SQLite connection for migrations");
        run_migrations(&conn).expect("Failed to run database migrations");
    }
    pool
}

/// Get a pooled connection from the pool.
pub fn get_conn(pool: &DbPool) -> PooledConnection<SqliteConnectionManager> {
    pool.get().expect("Failed to get DB connection from pool")
}

/// Run database migrations (create tables if they don't exist).
/// Extend this function as you add more tables.
pub fn run_migrations(conn: &Connection) -> SqlResult<()> {
    // Example: mod_events table
    conn.execute_batch(
        r#"
        DROP TABLE IF EXISTS history;
        CREATE TABLE IF NOT EXISTS history (
            event_id      INTEGER PRIMARY KEY AUTOINCREMENT,
            event_type    TEXT NOT NULL,
            timestamp     TEXT NOT NULL,
            mod_id        TEXT NOT NULL,
            name          TEXT NOT NULL,
            version       TEXT,
            authors       TEXT,
            steam_app_id  TEXT,
            path          TEXT NOT NULL,
            source        TEXT NOT NULL,
            created       TEXT,
            modified      TEXT
        );
        "#,
    )?;
    Ok(())
}
