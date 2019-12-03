extern crate env_logger;
#[macro_use]
extern crate shared;
extern crate nng; // NOTE: Still have nng errors even if we don't use the re-export...

use std::convert::TryFrom;
use std::process;
use std::time::Duration;

use nng::options::{Options, SendTimeout};
use nng::{Message, Protocol, Socket};
use shared::{CommandRequest, CommandResponse};

fn main() {
    env_logger::init();
    info!("i am the serverctl");

    println!("[+] Messaging server...");
    let request = CommandRequest::Noop;
    let response = match send_control_request(request) {
        Ok(resp) => resp,
        Err(err) => {
            println!("[x] Failed to communicate with server: {}", err);
            debug!("{:?}", err);
            process::exit(1);
        }
    };
    match response {
        CommandResponse::Ack => {
            println!("[+] Server responded!");
        }
    }
}

fn send_control_request(
    request: CommandRequest,
) -> Result<CommandResponse, Box<dyn std::error::Error>> {
    let socket = Socket::new(Protocol::Req0).unwrap();
    socket
        .set_opt::<SendTimeout>(Some(Duration::from_secs(60)))
        .unwrap();
    socket.dial(&format!("ipc:///tmp/dylibs")).unwrap();
    let request = Message::try_from(request)?;
    if let Err(e) = socket.send(request) {
        return Err(format!("failed to send request: {:?}", e).into());
    }
    let response = socket.recv()?;
    Ok(CommandResponse::try_from(response)?)
}
