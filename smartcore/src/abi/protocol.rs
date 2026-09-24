use crate::abi::messages::{ClientRequest, DaemonResponse};
use serde_json::{Result, to_string, from_str};

pub fn serialize_request(req: &ClientRequest) -> Result<String> {
    to_string(req)
}

pub fn deserialize_request(data: &str) -> Result<ClientRequest> {
    from_str(data)
}

pub fn serialize_response(res: &DaemonResponse) -> Result<String> {
    to_string(res)
}

pub fn deserialize_response(data: &str) -> Result<DaemonResponse> {
    from_str(data)
}
