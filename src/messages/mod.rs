//! Module for stating message contracts and decoding/encoding messages
//!
//! Example usage:
//!
//! ```
//! use unicorn::messages::{common, Msg, encode, decode};
//!
//! // All messages need an ID. So, create one first.
//! let id = common::ID {
//!     uuid: "_test".to_string(),
//!     component: common::Components::Core,
//! };
//!
//! // Create a Heartbeat message
//! let heartbeat = Msg::Heartbeat { id: id, count: 1 };
//!
//! // Encode the message to JSON bytes
//! let jbytes = encode(&heartbeat).unwrap();
//!
//! // Decode the JSON bytes to original message
//! let jmsg = decode(&jbytes).unwrap();
//! ```
//!

pub mod common;
pub mod core;
mod utils;

pub use self::utils::*;

/// Message structure definitions. Each variant of this enum forms a type of message.
#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
pub enum Msg {
    /// Status message. Used to send status of different components
    Status {
        id: common::ID,
        state: common::State,
        msg: Option<String>,
    },

    /// Heartbeat message. Used to check liveliness of connected peers.
    Heartbeat {
        id: common::ID,
        count: i32,
    },

    /// Error message containing a description
    Error(String),

    /// Ok message. Sent to indicate success but nothing useful to return
    Ok
}
