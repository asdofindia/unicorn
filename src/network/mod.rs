//! Network abstraction layer that handles low level network topology
//! and exposes a simple higher level API

mod net;
mod stream;

pub use self::net::Net;
pub use self::stream::Stream;

use std::sync::mpsc::Sender;
use messages::Msg;

/// List of states for a network connection
#[derive(Copy, Clone)]
pub enum Status {
    DISCONNECTED,
    CONNECTING,
    READY,
    RECONNECTING,
    TIMEOUT,
    ERROR,
}

/// Provides an interface for creating processor types
pub trait Processor: Send + Sync {
    fn process(&self, String, Sender<Msg>);
}
