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
    ///
    /// **Note**: This method will cause a `panic!()` if number of
    /// workers specified is less than 1.
    pub fn num_workers(&mut self, i: usize) {
        if i < 1 {
            panic!("Number of workers cannot be less than 1");
        }
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
            if buff.len() == 0 {
                break;
            }
            s.send(buff);
            let _ = s.flush();
        } else {
            break;
        }
    }
    drop(s);
}

#[cfg(test)]
mod test {
    #![allow(unused_variables)]

    use super::Net;
    use super::super::Status;

    #[test]
    fn test_new_net() {
        let n: Net = Net::bind("127.0.0.1:60000".to_string()).unwrap();
        assert!(match n.addr().as_ref() {
            "127.0.0.1:60000" => true,
            _ => false
        });
        assert!(match n.status() {
            Status::READY => true,
            _ => false
        });
        assert_eq!(n.num_workers, 4);
    }

    #[test]
    fn test_num_workers() {
        let mut n: Net = Net::bind("127.0.0.1:60001".to_string()).unwrap();
        n.num_workers(20);
        assert_eq!(n.num_workers, 20);
    }

    #[test]
    #[should_panic(expected = "Number of workers cannot be less than 1")]
    fn test_num_workers_panic() {
        let mut n: Net = Net::bind("127.0.0.1:60002".to_string()).unwrap();
        n.num_workers(0);
    }

    #[test]
    #[should_panic(expected = "Address already in use")]
    fn test_net_bind_panic() {
        let n = Net::bind("127.0.0.1:60003".to_string()).unwrap();
        let n1 = Net::bind("127.0.0.1:60003".to_string()).unwrap();
    }
}
