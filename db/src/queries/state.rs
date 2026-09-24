use rusqlite::{params, Connection, Result};
use crate::models::SessionState;

pub fn get_state(conn: &Connection) -> Result<SessionState> {
    let get_val = |k: &str| -> Option<String> {
        conn.query_row("SELECT value FROM session_state WHERE key = ?1", params![k], |r| r.get(0)).ok()
    };
    Ok(SessionState {
        active_user: get_val("active_user"),
        active_namespace: get_val("active_namespace"),
        active_project: get_val("active_project"),
    })
}

pub fn set_state(conn: &Connection, key: &str, value: Option<&str>) -> Result<()> {
    if let Some(v) = value {
        conn.execute("INSERT OR REPLACE INTO session_state (key, value) VALUES (?1, ?2)", params![key, v])?;
    } else {
        conn.execute("DELETE FROM session_state WHERE key = ?1", params![key])?;
    }
    Ok(())
}
