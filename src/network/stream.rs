//! Wrapper around TcpStream with convenience methods

use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Error;
use std::thread::sleep;
use std::time::Duration;

/// Structure that holds TcpStream and provides convenience methods
pub struct Stream {
    stream: TcpStream,
}

impl Stream {
    /// Create a new stream
    pub fn new(stream: TcpStream) -> Stream { Stream { stream: stream } }

    /// Try connecting to remote TCP socket only once
    fn connect_once(addr: &String) -> Result<Stream, Error> {
        match TcpStream::connect(&addr[..]) {
            Ok(s) => Ok(Stream::new(s)),
            Err(e) => {
                println!("[net] Unable to connect to link");
                Err(e)
            }
        }
    }

    /// Keep trying for connection in a loop
    fn connect_loop(addr: &String) -> Result<Stream, Error> {
        loop {
            println!("Retrying...");
            if let Ok(s) = Stream::connect_once(addr) {
                return Ok(s)
            }
            sleep(Duration::from_secs(1));
        }
    }

    /// Connect to a TCP listener socket and get back a stream
    pub fn connect(addr: &String, retry: bool) -> Result<Stream, Error> {
        match Stream::connect_once(addr) {
            Ok(s) => Ok(s),
            Err(e) => {
                if retry {
                    return Stream::connect_loop(addr)
                }
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
