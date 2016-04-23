//! Wrapper around TcpListener with convenience methods

use std::net::TcpListener;

use super::Status;
use super::stream::Stream;

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
                self.status = Status::ERROR;
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
                    Some(l) => Net::stream_loop(l, processor),
                    None => println!("[net] Listener is not available"),
                }
            }
            _ => println!("[net] Network interface is not ready"),
        }

    }

    /// Loop over incoming listener streams and set processor
    fn stream_loop(l: TcpListener, processor: &Fn(String, Stream)) {
        for stream in l.incoming() {
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
