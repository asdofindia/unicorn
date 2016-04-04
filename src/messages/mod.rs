//! Module for stating message contracts and decoding/encoding messages

extern crate rustc_serialize;
use rustc_serialize::{json, Encodable, Decodable};

pub mod common;
pub mod core;

/// Generic encoder for all messages. Encodes message structs to JSON strings
pub fn encode<T: Encodable>(msg: &T) -> String {
    json::encode(&msg).unwrap()
}

/// Generic decoder for all messages. Decodes JSON strings to message structs
pub fn decode<T: Decodable>(encodedstr: &str) -> T {
    json::decode(&encodedstr).unwrap()
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

        assert_eq!(tj, tjde);
    }
}
