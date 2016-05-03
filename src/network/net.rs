//! Wrapper around TcpListener with convenience methods.
//!
//! `Net` provides a higher-level abstraction of TCP listener sockets,
//! allowing implementations of basic request-reply and pub-sub
//! messaging architectures.
//!
//! `Net` wraps around `std::net::TcpListener` and uses channels to
//! communicate with incoming `TcpStream`s.

use std::net::TcpListener;
use std::io::Error;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use messages::Msg;

use super::Status;
use super::stream::Stream;

/// Network structure that holds network topology information. Aka, the network interface.
pub struct Net {
    addr: String,
    listener: TcpListener,
    status: Status,
    tx: Sender<Box<Stream>>
}

impl Net {
    /// Create a new network interface
    fn new(a: String, l: TcpListener, s: Status, tx: Sender<Box<Stream>>) -> Net {
        Net {
            addr: a,
            listener: l,
            status: s,
            tx: tx
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
    pub fn bind(addr: String) -> Result<(Net, Receiver<Box<Stream>>), Error> {
        match TcpListener::bind(&addr[..]) {
            Ok(l) => {
                let (tx, rx) = channel::<Box<Stream>>();
                Ok((Net::new(addr, l, Status::READY, tx), rx))
            }
            Err(e) => {
                println!("[net] Unable to bind to {}. Reason: {}", &addr, e);
                Err(e)
            }
        }
    }

    /// Process incoming TCP stream
    pub fn recv(self) {
        match self.status {
            Status::READY => {
                println!("[net] Ready to recv on {}", &self.addr);
                thread::spawn(move || {
                    self.stream_loop()
                });
            }
            _ => println!("[net] Network interface is not ready"),
        }

    }

    /// Loop over incoming listener streams and set processor
    fn stream_loop(self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let s = Box::new(Stream::new(stream));
                    println!("Received stream. Sending...");
                    self.tx.send(s).unwrap();
                }
                Err(_) => println!("[net] Could not grab incoming stream"),
            }
        }
    }

    /// Process incoming TCP Streams
    pub fn process_streams(rx: Receiver<Box<Stream>>, processor: &Fn(String, Sender<Msg>)) {
        let mut n = rx.iter();
        loop {
            match n.next() {
                Some(mut s) => {
                    let (msgtx, msgrx) = channel::<Msg>();
                    // @TODO: use `msgrx` to receive `Msg` sent by processor.
                    loop {
                        s.process(processor, msgtx.clone());
                    }
                },
                None => {
                    println!("[core] Net listener is possibly down. Dropping receiver.");
                    break;
                }
            }
        }
    }
}
