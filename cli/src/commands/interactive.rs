use crate::NamespaceCommands;
use crate::ipc::client::send_request;
use smartcore::abi::messages::ClientRequest;

pub fn handle_namespace(action: NamespaceCommands) {
    let (cmd, args) = match action {
        NamespaceCommands::Use { name } => ("namespace use".to_string(), vec![name]),
        NamespaceCommands::Exit => ("namespace exit".to_string(), vec![]),
    };
    let req = ClientRequest::Interactive { command: cmd, args };
    match send_request(req) {
        Ok(res) => println!("\x1b[32m[System] {:?}\x1b[0m", res),
        Err(_) => println!("\x1b[31m[System] SmartD Daemon is not running!\x1b[0m"),
    }
}
