use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn get_db_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    if std::env::var("CARGO_MANIFEST_DIR").is_ok() {
        path.push(".smart_term_v2_dev.sqlite");
    } else {
        path.push(".smart_term_v2.sqlite");
    }
    path
}

pub fn get_connection() -> Result<Connection> {
    let conn = Connection::open(get_db_path())?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.pragma_update(None, "busy_timeout", "5000")?;
    Ok(conn)
}
