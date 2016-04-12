//! Module for stating message contracts and decoding/encoding messages

extern crate rustc_serialize;
extern crate zmq;

use std::error::Error;
use std::{fmt, result};
use rustc_serialize::{json, Encodable, Decodable};

pub mod common;
pub mod core;


/// Main result type for messages
type MsgResult<T> = result::Result<T, MsgError>;

/// Message encode result object
pub type MsgEncodeResult<String> = MsgResult<String>;

/// Message decode result object
pub type MsgDecodeResult<T> = MsgResult<T>;

/// Generic error type for messages
#[allow(unstable)]
#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct MsgError {
    pub msg: String,
    pub errtype: MsgErrorType,
}

impl MsgError {
    pub fn new(msg: String, errtype: MsgErrorType) -> MsgError {
        MsgError {
            msg: msg,
            errtype: errtype,
        }
    }
}

impl fmt::Display for MsgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "(Message Error: {})", self.msg) }
}

impl Error for MsgError {
    fn description(&self) -> &str { &self.msg[..] }
}

/// Error types for messages
#[derive(RustcEncodable, RustcDecodable, Debug)]
pub enum MsgErrorType {
    EncodeError,
    DecodeError,
    InvalidComponentError,
}


/// Generic encoder for all messages. Encodes message structs to JSON strings
pub fn encode<T: Encodable>(msg: &T) -> MsgEncodeResult<String> {
    match json::encode(&msg) {
        Ok(msg) => Ok(msg),
        Err(e) => Err(MsgError::new(e.to_string(), MsgErrorType::EncodeError)),
    }
}

/// Generic decoder for all messages. Decodes JSON strings to message structs
pub fn decode<T: Decodable>(encodedstr: &str) -> MsgDecodeResult<T> {
    match json::decode(&encodedstr) {
        Ok(enc) => Ok(enc),
        Err(e) => Err(MsgError::new(e.to_string(), MsgErrorType::DecodeError)),
    }
}

/// Decodes ZeroMQ messages to message structs
pub fn decode_zmq<'a, T: Decodable>(msg: zmq::Message) -> MsgDecodeResult<T> {
    let s: &'a str = try!(msg.as_str());
    decode(&s)
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
            data: String,
        }

        let tj = Testjson {
            id: 1,
            data: "Hello".to_string(),
        };

        assert_eq!(encode(&tj).unwrap(), "{\"id\":1,\"data\":\"Hello\"}");
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
            things: Vec<i32>,
        }

        let tj = Testjson {
            id: 1,
            data: "Hello".to_string(),
            isit: true,
            things: vec![2, 3, 4],
        };
        let tjen = encode(&tj).unwrap();
        let tjde = decode(&tjen).unwrap();

        assert_eq!(tj, tjde);
    }
}
