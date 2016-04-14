//! Utilities for encoding and decoding messages

extern crate rustc_serialize;
extern crate zmq;

use std::error::Error;
use rustc_serialize::json;

use super::Msg;

/// Generic encoder for all messages. Encodes message structures to JSON strings
pub fn encode(msg: &Msg) -> Option<String> {
    match json::encode(&msg) {
        Ok(msg) => Some(msg.to_string()),
        Err(e) => {
            println!("Cannot encode message: {}", e.description());
            None
        }
    }
}

/// Generic decoder for all messages. Decodes JSON strings to message structures
pub fn decode(encodedstr: &str) -> Option<Msg> {
    match json::decode(&encodedstr) {
        Ok(enc) => Some(enc),
        Err(e) => {
            println!("Cannot decode message: {}", e.description());
            None
        }
    }
}

/// Decodes ZeroMQ messages to message structures
pub fn decode_zmq<'a>(msg: &'a zmq::Message) -> Option<Msg> {
    match msg.as_str() {
        Some(mstr) => decode(mstr),
        None => {
            println!("Cannot decode zmq message.");
            None
        }
    }
}


/// Unit tests for messages utility methods
#[cfg(test)]
mod tests {

    use super::{encode, decode, decode_zmq, zmq};
    use super::super::{Msg, common};

    fn dummy() -> Msg {
        let id = common::ID {
            uuid: "_test".to_string(),
            component: common::Components::Core,
        };
        Msg::Heartbeat { id: id, count: 1 }
    }

    /// Test message::encode
    #[test]
    fn test_message_encode() {

        let tj = dummy();

        assert_eq!(encode(&tj).unwrap(),
                   "{\"variant\":\"Heartbeat\",\"fields\":[{\"uuid\":\"_test\",\"component\":\"Core\"},1]}");
    }

    /// Test message::decode
    #[test]
    fn test_message_decode() {
        let tj = dummy();
        let tjen = encode(&tj).unwrap();
        let tjde = decode(&tjen).unwrap();

        assert_eq!(tj, tjde);
    }

    /// Test message::decode_zmq
    #[test]
    fn test_message_decode_zmq() {
        let tj = dummy();
        let tjen = encode(&tj).unwrap();
        let zmqmsg = zmq::Message::from_slice(&tjen.as_bytes()).unwrap();
        let tjde = decode_zmq(&zmqmsg).unwrap();

        assert_eq!(tj, tjde);
    }
}
