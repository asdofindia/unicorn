//! Processes incoming messages to core and routes them to appropriate handlers

use messages::{Msg, decode};
use messages::common::*;
use network::Stream;

/// Procedure for processing incoming messages
pub fn process_msg(msg: String, mut stream: Stream) {
    println!("[core] Processing: {:?}", msg);
    if let Some(msg) = decode(msg) {
        match msg {
            Msg::Heartbeat { id, count } => process_heartbeat_msg(id, count, &mut stream),
            Msg::Status { id, state, msg } => process_status_msg(id, state, msg, &mut stream),
        }
    } else {
        stream.send(b"Error processing message");
    }
}

/// Processes Heartbeat messages
fn process_heartbeat_msg(id: ID, count: i32, stream: &mut Stream) {
    println!("[core] Heartbeat: #{} from {}", count, id.uuid);
    stream.send(b"OK");
}

/// Processes Status messages
fn process_status_msg(id: ID, state: State, m: Option<String>, stream: &mut Stream) {
    println!("[core] Status: {:?} from {}. Message: {:?}", state, id.uuid, m);
    stream.send(b"OK");
}
