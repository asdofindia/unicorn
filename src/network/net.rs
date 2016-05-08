//! Wrapper around TcpListener with convenience methods.

use std::net::TcpListener;
use std::io::Error;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use messages::{Msg, encode};

use super::{Processor, Stream, Status};

/// Network structure that holds network topology information. Aka,
/// the network interface.
///
/// `Net` provides a higher-level abstraction of TCP listener
/// sockets. It does not impose any pattern, but it can be used to
/// implement basic request-reply and pub-sub messaging architectures.
///
/// `Net` wraps around `std::net::TcpListener` and uses channels to
/// communicate with incoming `TcpStream`s.
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
    status: Status
}

impl Net {
    /// Create a new network interface
    fn new(a: String, l: TcpListener, s: Status) -> Net {
        Net {
            addr: a,
            listener: l,
            status: s
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
                &self.stream_loop(processor);
            }
            _ => println!("[net] Network interface is not ready"),
        }

    }

    /// Loop over incoming listener streams and set processor
    fn stream_loop(&self, processor: &'static Processor) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let s = Arc::new(Mutex::new(Stream::new(stream)));
                    println!("Received stream. Sending...");
                    self.process_stream(s, processor);
                }
                Err(_) => println!("[net] Could not grab incoming stream"),
            }
        }
    }

    /// Process incoming TCP Streams
    fn process_stream<'p>(&self, s: Arc<Mutex<Stream>>, processor: &'static Processor) {
        let (msgtx, msgrx) = channel::<Msg>();
        let sarc_th = s.clone();
        thread::spawn(move || {
            loop {
                match sarc_th.lock() {
                    Ok(mut slock) => {
                        if let Some(st) = slock.recv() {
                            processor.process(st, msgtx.clone());
                        }
                    },
                    Err(e) => println!("[net] Cannot get a lock on stream. {}", e)
                }
            }
        });
        let sarc_loop = s.clone();
        thread::spawn(move || {
            let mut n = msgrx.iter();
            loop {
                if let Some(msg) = n.next() {
                    match encode(&msg) {
                        Some(st) => {
                            match sarc_loop.lock() {
                                Ok(mut slock) => {
                                    slock.send(st.as_bytes());
                                },
                                Err(e) => println!("[net] Cannot get a lock on stream. {}", e)
                            }
                        },
                        None => break
                    }
                }
            }
        });
    }
}
