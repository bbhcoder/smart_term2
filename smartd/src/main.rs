pub mod server;
pub mod workers;

use smartcore::cache::state_cache::StateCache;
use db::models::SessionState;
use db::connection::get_connection;
use db::queries::state::get_state;
use db::queries::schema::init_tables;

#[tokio::main]
async fn main() {
    let initial_state = if let Ok(conn) = get_connection() {
        let _ = init_tables(&conn);
        get_state(&conn).unwrap_or(SessionState {
            active_user: None,
            active_namespace: None,
            active_project: None,
        })
    } else {
        SessionState {
            active_user: None,
            active_namespace: None,
            active_project: None,
        }
    };

    let cache = StateCache::new(initial_state);
    let socket_path = "/tmp/smart_term.sock";

    let worker_cache = cache.clone();
    tokio::spawn(async move {
        workers::sync::start_sync_worker(worker_cache).await;
    });

    let _ = server::ipc::start_server(socket_path, cache).await;
}
