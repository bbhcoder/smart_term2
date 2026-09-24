use rusqlite::{params, Connection};

pub fn get_bound_user(conn: &Connection, target: &str) -> Option<String> {
    conn.query_row(
        "SELECT username FROM bindings WHERE target = ?1",
        params![target],
        |r| r.get(0),
    ).ok()
}
