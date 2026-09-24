use tokio::io::{AsyncReadExt, AsyncWriteExt};
use smartcore::abi::messages::{ClientRequest, DaemonResponse};
use smartcore::abi::protocol::{deserialize_request, serialize_response};
use smartcore::cache::state_cache::StateCache;
use smartcore::engine::{preexec, precmd};

#[cfg(unix)]
pub async fn start_server(socket_path: &str, cache: StateCache) -> Result<(), String> {
    use tokio::net::UnixListener;
    use std::fs;
    let _ = fs::remove_file(socket_path);
    let listener = UnixListener::bind(socket_path).map_err(|e| e.to_string())?;
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let cache_clone = cache.clone();
            tokio::spawn(async move {
                let mut buf = vec![0u8; 8192];
                if let Ok(n) = stream.read(&mut buf).await {
                    if n > 0 {
                        if let Ok(req_str) = String::from_utf8(buf[..n].to_vec()) {
                            if let Ok(req) = deserialize_request(&req_str) {
                                let res = handle_request(req, &cache_clone).await;
                                if let Ok(res_str) = serialize_response(&res) {
                                    let _ = stream.write_all(res_str.as_bytes()).await;
                                }
                            }
                        }
                    }
                }
            });
        }
    }
}

#[cfg(windows)]
pub async fn start_server(_socket_path: &str, cache: StateCache) -> Result<(), String> {
    use tokio::net::TcpListener;
    let listener = TcpListener::bind("127.0.0.1:47920").await.map_err(|e| e.to_string())?;
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let cache_clone = cache.clone();
            tokio::spawn(async move {
                let mut buf = vec![0u8; 8192];
                if let Ok(n) = stream.read(&mut buf).await {
                    if n > 0 {
                        if let Ok(req_str) = String::from_utf8(buf[..n].to_vec()) {
                            if let Ok(req) = deserialize_request(&req_str) {
                                let res = handle_request(req, &cache_clone).await;
                                if let Ok(res_str) = serialize_response(&res) {
                                    let _ = stream.write_all(res_str.as_bytes()).await;
                                }
                            }
                        }
                    }
                }
            });
        }
    }
}

async fn handle_request(req: ClientRequest, cache: &StateCache) -> DaemonResponse {
    match req {
        ClientRequest::PreExec { command, cwd } => {
            let state = cache.get();
            match preexec::process(&command, &cwd, &state) {
                Ok(_) => DaemonResponse::Success,
                Err(e) => DaemonResponse::Error(e),
            }
        }
        ClientRequest::PreCmd { cwd, exit_code } => {
            let mut state = cache.get();
            if precmd::process(&cwd, exit_code, &mut state).is_ok() {
                cache.update(state.clone());
                DaemonResponse::StateSync(state)
            } else {
                DaemonResponse::Error("Prefetch Failed".to_string())
            }
        }
        ClientRequest::Interactive { command, args } => {
            let mut state = cache.get();
            if command == "namespace use" && !args.is_empty() {
                state.active_namespace = Some(args[0].clone());
                cache.update(state.clone());
                DaemonResponse::StateSync(state)
            } else if command == "namespace exit" {
                state.active_namespace = None;
                cache.update(state.clone());
                DaemonResponse::StateSync(state)
            } else {
                DaemonResponse::Error("Unknown Interactive Command".to_string())
            }
        }
    }
}