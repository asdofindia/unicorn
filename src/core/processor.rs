//! Processes incoming messages to core and routes them to appropriate handlers

extern crate zmq;

use messages::{Msg, decode_zmq, send};
use messages::common::*;

/// Procedure for processing incoming ZMQ messages
pub fn process_msg(zmq_msg: &mut zmq::Message, sock: &mut zmq::Socket) {
    if let Some(msg) = decode_zmq(&zmq_msg) {
        println!("[core] Processing: {:?}", msg);
        match msg {
            Msg::Heartbeat{ id, count } => process_heartbeat_msg(id, count, sock),
            Msg::Status{ id, state, msg } => process_status_msg(id, state, msg, sock),
        }
    } else {
        send(b"Error processing message", sock);
    }
}

/// Processes Heartbeat messages
fn process_heartbeat_msg(id: ID, count: i32, sock: &mut zmq::Socket) {
    println!("[core] Heartbeat: #{} from {}", count, id.uuid);
    send(b"Hi", sock);
}

/// Processes Status messages
fn process_status_msg(id: ID, state: State, m: Option<String>, sock: &mut zmq::Socket) {
    println!("[core] Status: {:?} from {}. Message: {:?}", state, id.uuid, m);
    send(b"OK", sock);
}
