//! Processes incoming messages to core and routes them to appropriate handlers

use std::sync::mpsc::Sender;

use messages::{Msg, decode};
use messages::common::*;
use network::Processor;

pub struct ProcessMsg;

impl Processor for ProcessMsg {
    /// Procedure for processing incoming messages
    fn process(&self, msg: String, mtx: Sender<Msg>) {
        println!("[core] Processing: {:?}", msg);
        if let Some(msg) = decode(msg) {
            match msg {
                Msg::Heartbeat { id, count } => process_heartbeat_msg(id, count, mtx.clone()),
                Msg::Status { id, state, msg } => process_status_msg(id, state, msg, mtx.clone()),
                _ => mtx.send(Msg::Error("Unmatched message type".to_string())).unwrap()
            }
        } else {
            mtx.send(Msg::Error("Unable to decode message".to_string())).unwrap();
        }
    }
}

unsafe impl Send for ProcessMsg {}

unsafe impl Sync for ProcessMsg {}

/// Processes Heartbeat messages
fn process_heartbeat_msg(id: ID, count: i32, mtx: Sender<Msg>) {
    println!("[core] Heartbeat: #{} from {}", count, id.uuid);
    mtx.send(Msg::Ok).unwrap();
}

/// Processes Status messages
fn process_status_msg(id: ID, state: State, m: Option<String>, mtx: Sender<Msg>) {
    println!("[core] Status: {:?} from {}. Message: {:?}", state, id.uuid, m);
    mtx.send(Msg::Ok).unwrap();
}
