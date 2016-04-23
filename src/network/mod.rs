//! Message queue abstraction layer that handles low level network topology and
//! exposes a simple higher level API

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::Error;

/// List of states for a network connection
pub enum Status {
    DISCONNECTED,
    CONNECTING,
    READY,
    RECONNECTING,
    TIMEOUT,
    ERROR,
}

/// Network structure that holds network topology information. Aka, the network interface.
pub struct Net {
    addr: String,
    listener: Option<TcpListener>,
    status: Status,
}

impl Net {
    /// Create a new network interface
    pub fn new(addr: &str) -> Net {
        Net {
            addr: addr.to_string(),
            listener: None,
            status: Status::DISCONNECTED,
        }
    }

    /// Start a listener on an existing network interface
    pub fn bind(&mut self) -> Result<&mut Net, String> {
        self.listener = match TcpListener::bind(&self.addr[..]) {
            Ok(l) => {
                self.status = Status::READY;
                Some(l)
            }
            Err(e) => {
                println!("[mq] unable to bind to {}. Reason: {}", self.addr, e);
                None
            }
        };
        Ok(self)
    }

    /// Process incoming TCP stream
    pub fn recv(self, processor: &Fn(String, Stream)) {
        match self.status {
            Status::READY => {
                println!("[net] Ready to recv on {}", &self.addr);
                match self.listener {
                    Some(l) => {
                        for stream in l.incoming() {
                            match stream {
                                Ok(stream) => {
                                    let s = Stream::new(stream);
                                    s.process(processor);
                                }
                                Err(_) => println!(""),
                            }
                        }
                    },
                    None => println!("[net] Listener is not available")
                }
            }
            _ => println!("[net] Network interface is not ready"),
        }

    }
}

/// Structure that holds TcpStream and provides convenience methods
pub struct Stream {
    stream: TcpStream,
}

impl Stream {
    /// Create a new stream
    pub fn new(stream: TcpStream) -> Stream { Stream { stream: stream } }

    /// Connect to a TCP listener socket and get back a stream
    pub fn connect(addr: String) -> Result<Stream, Error> {
        match TcpStream::connect(&addr[..]) {
            Ok(s) => Ok(Stream::new(s)),
            Err(e) => {
                println!("[net] Unable to connect to link");
                Err(e)
            }
        }
    }

    /// Process a given TcpStream and invoke processor
    pub fn process(mut self, processor: &Fn(String, Stream)) {
        let mut buff = String::new();
        match self.stream.read_to_string(&mut buff) {
            Ok(n) => {
                if n <= buff.len() {
                    processor(buff, self);
                }
            }
            Err(e) => {
                println!("[net] Unable to process incoming stream: {}", e);
            }
        }
    }

    /// Procedure for sending messages through a specified stream
    pub fn send(&mut self, b: &[u8]) {
        match self.stream.write(b) {
            Ok(n) => println!("[net] Sent: Length: {}", n),
            Err(e) => println!("[net] Error sending message: {}", e),
        }
    }
}
