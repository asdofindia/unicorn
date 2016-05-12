//! Processes incoming messages to core and routes them to appropriate handlers

use messages::{Msg, decode, encode_bytes};
use messages::common::*;
use network::{Processor, Stream};

pub struct ProcessMsg;

impl Processor for ProcessMsg {
    /// Procedure for processing incoming messages
    fn process(&self, msg: String, stream: &mut Stream) {
        println!("[core] Processing: {:?}", msg);
        if let Some(msg) = decode(msg) {
            match msg {
                Msg::Heartbeat { id, count } => process_heartbeat_msg(id, count, stream),
                Msg::Status { id, state, msg } => process_status_msg(id, state, msg, stream),
                Msg::Ok => process_ok(),
                _ => process_error("Unknown message received".to_string(), stream)
            }
        } else {
            process_error("Unable to decode message".to_string(), stream);
        }
    }
}

unsafe impl Send for ProcessMsg {}

unsafe impl Sync for ProcessMsg {}

/// Processes error states
fn process_error(emsg: String, stream: &mut Stream) {
    stream.send(encode_bytes(&Msg::Error(emsg)));
}

/// Processes Heartbeat messages
fn process_heartbeat_msg(id: ID, count: i32, stream: &mut Stream) {
    println!("[core] Heartbeat: #{} from {}", count, id.uuid);
    // TODO: Need better error handling here
    stream.send(encode_bytes(&Msg::Ok));
}

/// Processes Status messages
fn process_status_msg(id: ID, state: State, m: Option<String>, stream: &mut Stream) {
    println!("[core] Status: {:?} from {}. Message: {:?}", state, id.uuid, m);
    // TODO: Need better error handling here
    stream.send(encode_bytes(&Msg::Ok));
}

/// Process Ok messages
fn process_ok() {
    println!("[core] Got OK");
}
