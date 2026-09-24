use db::connection::get_connection;
use db::queries::bindings::get_bound_user;
use db::models::SessionState;

pub fn process(cwd: &str, _exit_code: i32, state: &mut SessionState) -> Result<(), String> {
    if let Ok(conn) = get_connection() {
        if let Some(user) = get_bound_user(&conn, cwd) {
            state.active_user = Some(user);
        }
    }
    Ok(())
}
