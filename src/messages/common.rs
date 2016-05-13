//! Common message structures shared by more than one components

/// List of main component types in unicorn
#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
pub enum Components {
    Core = 0,
    Gateway = 1,
    DataStore = 2,
    DataElement = 3,
    DataHandler = 4,
    ClientAdapter = 5,
}

/// Different states in which a component can be at any given time
#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
pub enum State {
    CONNECTING = 0,
    READY = 1,
    BUSY = 2,
    RETRYING = 3,
    SUCCESS = 4,
    FAILED = 5,
    OFFLINE = 99,
}

/// Identity message. Used by every message that needs an ID
#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq)]
pub struct ID {
    pub uuid: String,
    pub component: Components,
}
