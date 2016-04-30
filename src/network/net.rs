//! Wrapper around TcpListener with convenience methods

use std::net::TcpListener;
use std::io::Error;

use super::Status;
use super::stream::Stream;

/// Network structure that holds network topology information. Aka, the network interface.
pub struct Net {
    pub addr: String,
    listener: TcpListener,
    pub status: Status,
}

impl Net {
    /// Create a new network interface
    pub fn new(a: String, l: TcpListener, s: Status) -> Net {
        Net {
            addr: a,
            listener: l,
            status: s,
        }
    }

    /// Start a listener on an existing network interface
    pub fn bind(addr: String) -> Result<Net, Error> {
        match TcpListener::bind(&addr[..]) {
            Ok(l) => {
                Ok(Net::new(addr, l, Status::READY))
            }
            Err(e) => {
                println!("[net] Unable to bind to {}. Reason: {}", &addr, e);
                Err(e)
            }
        }
    }

    /// Process incoming TCP stream
    pub fn recv(self, processor: &Fn(String, Stream)) {
        match self.status {
            Status::READY => {
                println!("[net] Ready to recv on {}", &self.addr);
                self.stream_loop(processor)
            }
            _ => println!("[net] Network interface is not ready"),
        }

    }

    /// Loop over incoming listener streams and set processor
    fn stream_loop(self, processor: &Fn(String, Stream)) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let s = Stream::new(stream);
                    s.process(processor);
                }
                Err(_) => println!("[net] Could not grab incoming stream"),
            }
        }
    }
}
