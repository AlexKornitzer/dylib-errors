#[macro_use]
extern crate shared;

use std::time::Duration;

use shared::nng::options::{Options, SendTimeout};
use shared::nng::{Protocol, Socket};

fn main() {
    let socket = Socket::new(Protocol::Rep0).unwrap();
    socket
        .set_opt::<SendTimeout>(Some(Duration::from_secs(60)))
        .unwrap();
    socket.listen(&format!("ipc:///tmp/dylibs")).unwrap();
}
