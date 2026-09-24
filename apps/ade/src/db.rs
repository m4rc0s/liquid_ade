use rusqlite::Connection;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum DbError {
    HomeDirNotFound,
    Io(std::io::Error),
    Sqlite(rusqlite::Error),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HomeDirNotFound => write!(f, "Unable to resolve user home directory"),
            Self::Io(e) => write!(f, "I/O error initializing database: {e}"),
            Self::Sqlite(e) => write!(f, "SQLite error: {e}"),
        }
    }
}

impl std::error::Error for DbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::HomeDirNotFound => None,
            Self::Io(e) => Some(e),
            Self::Sqlite(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for DbError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(err: rusqlite::Error) -> Self {
        Self::Sqlite(err)
    }
}

/// Resolves the canonical default database path `~/.liquid/liquid.db`.
pub fn default_db_path() -> Result<PathBuf, DbError> {
    let home = dirs::home_dir().ok_or(DbError::HomeDirNotFound)?;
    Ok(home.join(".liquid").join("liquid.db"))
}

/// Initializes embedded SQLite database at `~/.liquid/liquid.db`, ensuring directory
/// exists and running schema migrations for `projects`, `checkpoints`, and `settings`.
pub fn init_db() -> Result<Connection, DbError> {
    let path = default_db_path()?;
    init_db_at(&path)
}

/// Initializes embedded SQLite database at a specific path, ensuring parent directory
/// exists and running schema migrations.
pub fn init_db_at<P: AsRef<Path>>(path: P) -> Result<Connection, DbError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(path)?;

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL,
            last_opened_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS checkpoints (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            epic_id TEXT NOT NULL,
            task_id TEXT NOT NULL,
            phase TEXT NOT NULL,
            intent TEXT NOT NULL,
            data_json TEXT NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        "#,
    )?;

    Ok(conn)
}

/// Verifies that all expected tables exist in the given database connection.
pub fn verify_tables_exist(conn: &Connection) -> Result<bool, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT count(*) FROM sqlite_master WHERE type='table' AND name IN ('projects', 'checkpoints', 'settings')",
    )?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count == 3)
}
