//! Utilities for encoding and decoding messages

extern crate rustc_serialize;

use std::error::Error;
use rustc_serialize::json;

use super::Msg;

/// Generic encoder for all messages. Encodes message structures to JSON strings
pub fn encode(msg: &Msg) -> Option<String> {
    match json::encode(&msg) {
        Ok(msg) => Some(msg.to_string()),
        Err(e) => {
            debug!("Cannot encode message: {}", e);
            None
        }
    }
}

/// Convenience method to message structures directly to serialized bytes
///
/// **Warning**: This function does not guarantee notifying error
/// states. It will return a slice of `u8` either way.
pub fn encode_bytes<'a>(msg: &Msg) -> Vec<u8> {
    encode(&msg).unwrap_or("".to_string()).into_bytes()
}

/// Generic decoder for all messages. Decodes JSON strings to message structures
pub fn decode(encodedstr: &String) -> Option<Msg> {
    match json::decode(encodedstr.as_ref()) {
        Ok(enc) => Some(enc),
        Err(e) => {
            debug!("Cannot decode message: {}", e.description());
            None
        }
    }
}


/// Unit tests for messages utility methods
#[cfg(test)]
mod tests {

    use super::{encode, encode_bytes, decode};
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

    /// Test message::encode_bytes
    #[test]
    fn test_message_encode_bytes() {
        let tj = dummy();
        {
            let tjen = encode_bytes(&tj);
            let tjde = decode(&String::from_utf8(tjen).unwrap()).unwrap();

            assert_eq!(tj, tjde);
        }
        {
            let tjen = encode(&tj);
            let tjen_bytes = encode_bytes(&tj);

            assert_eq!(tjen.unwrap().into_bytes(), tjen_bytes);
        }
    }
}
