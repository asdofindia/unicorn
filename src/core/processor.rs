//! Processes incoming messages to core and routes them to appropriate handlers

use messages::{Msg, decode, encode_bytes};
use messages::common::*;

/// Procedure for processing incoming messages
pub fn process(m: &String) -> Vec<u8> {
    let buff: Vec<u8>;
    debug!("[core] Processing: {:?}", m);
    if let Some(decoded) = decode(m) {
        buff = match decoded {
            Msg::Heartbeat { id, count } => process_heartbeat_msg(id, count),
            Msg::Status { id, state, msg } => process_status_msg(id, state, msg),
            Msg::Ok => process_ok(),
            _ => process_error("Unknown message received".to_string()),
        }
    } else {
        buff = process_error("Unable to decode message".to_string());
    }
    buff
}

/// Processes error states
fn process_error(emsg: String) -> Vec<u8> {
    debug!("[core] Error processing message: {}", &emsg);
    encode_bytes(&Msg::Error(emsg))
}

/// Processes Heartbeat messages
fn process_heartbeat_msg(id: ID, count: i32) -> Vec<u8> {
    debug!("[core] Heartbeat: #{} from {}", count, id.uuid);
    // TODO: Need better error handling here
    encode_bytes(&Msg::Ok)
}

/// Processes Status messages
fn process_status_msg(id: ID, state: State, m: Option<String>) -> Vec<u8> {
    debug!("[core] Status: {:?} from {}. Message: {:?}", state, id.uuid, m);
    // TODO: Need better error handling here
    encode_bytes(&Msg::Ok)
}

/// Process Ok messages
fn process_ok() -> Vec<u8> {
    debug!("[core] Got OK");
    encode_bytes(&Msg::Ok)
}
