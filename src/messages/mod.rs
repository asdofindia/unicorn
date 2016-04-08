//! Module for stating message contracts and decoding/encoding messages

extern crate rustc_serialize;

use std::error::Error;
use std::fmt;
use rustc_serialize::{json, Encodable, Decodable};

pub mod common;
pub mod core;


/// Generic error type for messages
#[allow(unstable)]
#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct MsgError {
    pub msg: String
}

impl fmt::Display for MsgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Message Error: {})", self.msg)
    }
}

impl Error for MsgError {
    fn description(&self) -> &str {
        &self.msg[..]
    }
}


/// Generic encoder for all messages. Encodes message structs to JSON strings
pub fn encode<T: Encodable>(msg: &T) -> String {
    match json::encode(&msg) {
        Ok(msgstr) => msgstr,
        Err(e) => {
            encode(&MsgError {
                msg: e.to_string()
            })
        }
    }
}

/// Generic decoder for all messages. Decodes JSON strings to message structs
pub fn decode<T: Decodable>(encodedstr: &str) -> Result<T, json::DecoderError> {
    json::decode(&encodedstr)
}


/// Unit tests for messages methods
#[cfg(test)]
mod tests {

    use super::{encode, decode};

    /// Test message::encode
    #[test]
    fn test_message_encode() {

        #[allow(unstable)]
        #[derive(RustcEncodable)]
        struct Testjson {
            id: i32,
            data: String
        }

        let tj = Testjson { id: 1, data: "Hello".to_string() };

        assert_eq!(encode(&tj), "{\"id\":1,\"data\":\"Hello\"}");
    }

    /// Test message::decode
    #[test]
    fn test_message_decode() {

        #[allow(unstable)]
        #[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
        struct Testjson {
            id: i32,
            data: String,
            isit: bool,
            things: Vec<i32>
        }

        let tj = Testjson { id: 1, data: "Hello".to_string(), isit: true, things: vec![2, 3, 4] };
        let tjen = encode(&tj);
        let tjde = decode(&tjen);

        assert_eq!(tj, tjde.unwrap());
    }
}
