//! Wrapper around TcpListener with convenience methods.

use std::net::TcpListener;
use std::io::Error;
use threadpool::ThreadPool;

use super::{Processor, Stream, Status};

/// Network structure that holds network topology information. Aka,
/// the network interface.
///
/// `Net` provides a higher-level abstraction of TCP listener
/// sockets. It multiplexes incoming connections into parallel threads
/// and runs a given `Processor` against all messages that the
/// connection receives.
///
/// ## Using `Net`
///
/// `Net` works in 2 steps:
///
/// - Bind a listener using `Net::bind(addr)`. This starts a
/// TcpListener on the specified address and returns an instance of
/// `Net`.
/// - Start accepting connections by calling `.recv(processor)` on the
/// returned `Net` instance.
pub struct Net {
    addr: String,
    listener: TcpListener,
    status: Status,
    num_workers: usize
}

impl Net {
    /// Create a new network interface
    fn new(a: String, l: TcpListener, s: Status) -> Net {
        Net {
            addr: a,
            listener: l,
            status: s,
            num_workers: 4
        }
    }

    /// Returns the addr
    pub fn addr(&self) -> String {
        self.addr.clone()
    }

    /// Returns the status
    pub fn status(&self) -> Status {
        self.status.clone()
    }

    /// Set number of workers for processing incoming streams in parallel
    pub fn num_workers(&mut self, i: usize) {
        self.num_workers = i;
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
    pub fn recv(&self, processor: &'static Processor) {
        match self.status {
            Status::READY => {
                println!("[net] Ready to recv on {}", &self.addr);
                self.stream_loop(processor);
            }
            _ => println!("[net] Network interface is not ready"),
        }

    }

    /// Loop over incoming listener streams and set processor
    fn stream_loop(&self, processor: &'static Processor) {
        let pool = ThreadPool::new_with_name("netstream".to_string(), self.num_workers);
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let s = Stream::new(stream);
                    pool.execute(move || {
                        process_stream(s, processor);
                    });
                }
                Err(_) => println!("[net] Could not grab incoming stream"),
            }
        }
    }
}


/// Process incoming TCP Streams
fn process_stream(mut s: Stream, processor: &'static Processor) {
    loop {
        if let Some(st) = s.recv() {
            let buff: Vec<u8> = processor.process(st);
            s.send(buff);
        } else {
            break;
        }
    }
}
