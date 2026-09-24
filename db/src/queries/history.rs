use rusqlite::{params, Connection, Result};
use crate::models::{HistoryEntry, SessionState};

pub fn insert_command(conn: &Connection, cmd: &str, cwd: &str, state: &SessionState) -> Result<()> {
    conn.execute(
        "INSERT INTO history (command, cwd, namespace, project, user) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![cmd, cwd, state.active_namespace, state.active_project, state.active_user],
    )?;
    Ok(())
}

pub fn get_history(conn: &Connection, limit: u32) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT command FROM history ORDER BY id DESC LIMIT ?1")?;
    let cmds = stmt.query_map(params![limit], |row| row.get(0))?.filter_map(Result::ok).collect();
    Ok(cmds)
}

pub fn export_history(
    conn: &Connection,
    namespace: Option<&str>,
    project: Option<&str>,
    user: Option<&str>,
) -> Result<Vec<HistoryEntry>> {
    let mut query = String::from("SELECT id, command, cwd, namespace, project, user, created_at FROM history WHERE 1=1");
    let mut p: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(ns) = namespace { query.push_str(" AND namespace = ?"); p.push(Box::new(ns.to_string())); }
    if let Some(pr) = project { query.push_str(" AND project = ?"); p.push(Box::new(pr.to_string())); }
    if let Some(u) = user { query.push_str(" AND user = ?"); p.push(Box::new(u.to_string())); }

    query.push_str(" ORDER BY id ASC");
    
    let mut stmt = conn.prepare(&query)?;
    let p_refs: Vec<&dyn rusqlite::ToSql> = p.iter().map(|x| x.as_ref()).collect();
    
    let rows = stmt.query_map(rusqlite::params_from_iter(p_refs), |row| {
        Ok(HistoryEntry {
            id: row.get(0)?,
            command: row.get(1)?,
            cwd: row.get(2)?,
            namespace: row.get(3)?,
            project: row.get(4)?,
            user: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;

    Ok(rows.filter_map(Result::ok).collect())
}
