//! Wrapper around TcpStream with convenience methods

use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Error;
use std::thread::sleep;
use std::time::Duration;
use std::string::String;

use bufstream::BufStream;

/// Structure that holds TcpStream and provides convenience methods
pub struct Stream {
    stream: BufStream<TcpStream>,
}

impl Stream {
    /// Create a new stream
    pub fn new(stream: TcpStream) -> Stream { Stream { stream: BufStream::new(stream) } }

    /// Try connecting to remote TCP socket only once
    fn connect_once(addr: &String) -> Result<Stream, Error> {
        match TcpStream::connect(&addr[..]) {
            Ok(s) => Ok(Stream::new(s)),
            Err(e) => {
                debug!("[net] Unable to connect to link");
                Err(e)
            }
        }
    }

    /// Keep trying for connection in a loop
    fn connect_loop(addr: &String) -> Result<Stream, Error> {
        loop {
            debug!("Retrying...");
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
    pub fn recv(&mut self) -> Option<String> {
        let mut buff = String::new();
        match self.stream.read_line(&mut buff) {
            Ok(n) => {
                if n > 0 && n <= buff.len() {
                    return Some(String::from((&buff[..]).trim()))
                }
                None
            }
            Err(e) => {
                debug!("[net] Unable to process incoming stream: {}", e);
                None
            }
        }
    }

    /// Procedure for sending messages through a specified stream
    pub fn send(&mut self, mut b: Vec<u8>) {
        b.push(0xA);
        match self.stream.write(&b) {
            Ok(n) => debug!("[net] Sent: Length: {}", n),
            Err(e) => debug!("[net] Error sending message: {}", e),
        }
    }

    /// Call flush on the buffer to send out currently stored messages
    /// immediately
    pub fn flush(&mut self) -> Result<(), Error> {
        self.stream.flush()
    }
}
