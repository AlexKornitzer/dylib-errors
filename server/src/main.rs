#[macro_use]
extern crate shared;

use std::convert::TryFrom;
use std::time::Duration;

use shared::nng::options::{Options, SendTimeout};
use shared::nng::{Message, Protocol, Socket};
use shared::{CommandRequest, CommandResponse};

fn main() {
    info!("i am the server");
    let socket = Socket::new(Protocol::Rep0).unwrap();
    socket
        .set_opt::<SendTimeout>(Some(Duration::from_secs(60)))
        .unwrap();
    socket.listen(&format!("ipc:///tmp/dylibs")).unwrap();

    loop {
        match socket.recv() {
            Ok(request) => {
                let request = CommandRequest::try_from(request).unwrap();
                process(request);
                let response = Message::try_from(CommandResponse::Ack).unwrap();
                if let Err(err) = socket.send(response) {
                    error!("failed to send response: {:?}", err);
                }
            }
            Err(err) => {
                error!("error receiving: {:?}", err);
            }
        }
    }
}

fn process(request: CommandRequest) {
    info!("processing request: {:?}", request);
}
