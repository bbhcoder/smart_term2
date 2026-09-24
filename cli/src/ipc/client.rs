use smartcore::abi::messages::{ClientRequest, DaemonResponse};
use smartcore::abi::protocol::{serialize_request, deserialize_response};
use std::io::{Read, Write};

#[cfg(unix)]
pub fn send_request(req: ClientRequest) -> Result<DaemonResponse, String> {
    use std::os::unix::net::UnixStream;
    let req_str = serialize_request(&req).map_err(|e| e.to_string())?;
    let mut stream = UnixStream::connect("/tmp/smart_term.sock").map_err(|e| e.to_string())?;
    stream.write_all(req_str.as_bytes()).map_err(|e| e.to_string())?;
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf).map_err(|e| e.to_string())?;
    let res_str = String::from_utf8_lossy(&buf[..n]).to_string();
    deserialize_response(&res_str).map_err(|e| e.to_string())
}

#[cfg(windows)]
pub fn send_request(req: ClientRequest) -> Result<DaemonResponse, String> {
    use std::net::TcpStream;
    let req_str = serialize_request(&req).map_err(|e| e.to_string())?;
    let mut stream = TcpStream::connect("127.0.0.1:47920").map_err(|e| e.to_string())?;
    stream.write_all(req_str.as_bytes()).map_err(|e| e.to_string())?;
    let mut buf = [0u8; 8192];
    let n = stream.read(&mut buf).map_err(|e| e.to_string())?;
    let res_str = String::from_utf8_lossy(&buf[..n]).to_string();
    deserialize_response(&res_str).map_err(|e| e.to_string())
}
