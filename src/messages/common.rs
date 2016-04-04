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

/// Heartbeat message
#[derive(RustcEncodable, RustcDecodable)]
pub struct Heartbeat {
    pub component: Components,
    pub count: i32,
}
