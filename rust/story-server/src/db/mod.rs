pub mod migrations;

use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

pub fn initialize_database<P: AsRef<Path>>(db_path: P) -> Result<Connection> {
    let conn = Connection::open(&db_path)
        .with_context(|| format!("Failed to open database at {:?}", db_path.as_ref()))?;

    // Enable foreign keys
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Run migrations
    migrations::run_migrations(&conn)?;

    log::info!("Database initialized at {:?}", db_path.as_ref());
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_initialize_database() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let conn = initialize_database(&db_path).unwrap();

        // Verify foreign keys are enabled
        let fk_enabled: i32 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(fk_enabled, 1);
    }
}
