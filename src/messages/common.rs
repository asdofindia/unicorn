//! Common message structures shared by more than one components
#![allow(unstable)]


/// List of main components in unicorn
#[derive(RustcEncodable, RustcDecodable)]
pub enum Components {
    ClientAdapter,
    Core,
    DataElement,
    DataHandler,
    DataStore,
    Gateway,
}

/// Identify message. Used by every message that needs an ID
#[derive(RustcEncodable, RustcDecodable)]
pub struct ID {
    pub id: String,
    pub component: Components,
}

/// Heartbeat message. Used to check liveliness of connected peers.
#[derive(RustcEncodable, RustcDecodable)]
pub struct Heartbeat {
    pub id: ID,
    pub count: i32,
}

/// Ready message. Sent when a component has connected and is ready.
#[derive(RustcEncodable, RustcDecodable)]
pub struct Ready {
    pub id: ID,
}
