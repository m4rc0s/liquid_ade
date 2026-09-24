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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ProjectRecord {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at: String,
    pub last_opened_at: String,
}

/// Lists all registered projects ordered by most recently opened.
pub fn list_projects(conn: &Connection) -> Result<Vec<ProjectRecord>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name, path, created_at, last_opened_at FROM projects ORDER BY last_opened_at DESC",
    )?;
    let project_iter = stmt.query_map([], |row| {
        Ok(ProjectRecord {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            created_at: row.get(3)?,
            last_opened_at: row.get(4)?,
        })
    })?;

    let mut projects = Vec::new();
    for p in project_iter {
        projects.push(p?);
    }
    Ok(projects)
}

/// Opens an existing project or registers a new project record, updating its `last_opened_at`.
pub fn open_or_register_project(
    conn: &Connection,
    path: &str,
    name_opt: Option<&str>,
) -> Result<ProjectRecord, rusqlite::Error> {
    let existing = conn.query_row(
        "SELECT id, name, path, created_at, last_opened_at FROM projects WHERE path = ?1",
        [path],
        |row| {
            Ok(ProjectRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
                created_at: row.get(3)?,
                last_opened_at: row.get(4)?,
            })
        },
    );

    match existing {
        Ok(mut record) => {
            conn.execute(
                "UPDATE projects SET last_opened_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now') WHERE id = ?1",
                [&record.id],
            )?;
            let updated_last_opened: String = conn.query_row(
                "SELECT last_opened_at FROM projects WHERE id = ?1",
                [&record.id],
                |row| row.get(0),
            )?;
            record.last_opened_at = updated_last_opened;
            Ok(record)
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            let id: String =
                conn.query_row("SELECT lower(hex(randomblob(16)))", [], |row| row.get(0))?;
            let derived_name = Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("project");
            let name = name_opt.unwrap_or(derived_name);

            conn.execute(
                "INSERT INTO projects (id, name, path, created_at, last_opened_at) 
                 VALUES (?1, ?2, ?3, strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))",
                [&id, name, path],
            )?;

            let (created_at, last_opened_at): (String, String) = conn.query_row(
                "SELECT created_at, last_opened_at FROM projects WHERE id = ?1",
                [&id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )?;

            Ok(ProjectRecord {
                id,
                name: name.to_string(),
                path: path.to_string(),
                created_at,
                last_opened_at,
            })
        }
        Err(e) => Err(e),
    }
}
