//! Network abstraction layer that handles low level network topology
//! and exposes a simple higher level API

mod net;
mod stream;

/// List of states for a network connection
pub enum Status {
    DISCONNECTED,
    CONNECTING,
    READY,
    RECONNECTING,
    TIMEOUT,
    ERROR,
}

pub use self::net::Net;
pub use self::stream::Stream;
