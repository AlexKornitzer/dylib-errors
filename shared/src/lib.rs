pub extern crate bincode;
extern crate consul; // NOTE: Comment out to remove all linker errors bar those related to nng
pub extern crate log;
pub extern crate nng;
#[macro_use]
extern crate serde;

// Re-export macros
pub use log::{debug, error, info, warn};

use nng::Message;
use std::convert::TryFrom;

mod error;

macro_rules! from_message {
    ($i:ident) => {
        impl TryFrom<Message> for $i {
            type Error = crate::error::Error;
            fn try_from(msg: Message) -> Result<Self, Self::Error> {
                Ok(bincode::deserialize(msg.as_slice()).map_err(crate::error::bincode)?)
            }
        }
    };
}

macro_rules! into_message {
    ($i:ident) => {
        impl TryFrom<$i> for Message {
            type Error = crate::error::Error;
            fn try_from(i: $i) -> Result<Self, Self::Error> {
                Ok(Message::from(
                    bincode::serialize(&i)
                        .map_err(crate::error::bincode)?
                        .as_slice(),
                ))
            }
        }
    };
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CommandRequest {
    Noop,
}
from_message!(CommandRequest);
into_message!(CommandRequest);

#[derive(Debug, Deserialize, Serialize)]
pub enum CommandResponse {
    Ack,
}
from_message!(CommandResponse);
into_message!(CommandResponse);
