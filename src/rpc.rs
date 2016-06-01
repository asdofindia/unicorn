//! Performs RPC-like data exchange over `nanomsg::Protocol::Rep` sockets.

use nanomsg::{Protocol, Socket, Endpoint, Result};
use messages::{Msg, encode_bytes, decode};
use std::io::{Read, Write};

pub enum RPCType<'r> {
    Client(&'r str),
    Server(&'r str),
}

pub struct RPC {
    socket: Socket,
    endpoint: Endpoint,
}

impl RPC {
    fn create_socket(socktype: Protocol) -> Socket {
        match Socket::new(socktype) {
            Ok(s) => s,
            Err(e) => panic!("[rpc] Error creating {:?} socket. Error: {}", socktype, e),
        }
    }

    fn connect_remote(mut socket: Socket, addr: &str) -> (Socket, Endpoint) {
        let endpoint = match socket.connect(addr.as_ref()) {
            Ok(ce) => {
                debug!("[rpc] Connected to remote Rep endpoint at {}", &addr);
                ce
            }
            Err(e) => panic!("[rpc] Unable to connect to remote Rep endpoint: {}\nError: {}", &addr, e),
        };
        (socket, endpoint)
    }

    fn bind(mut socket: Socket, addr: &str) -> (Socket, Endpoint) {
        let endpoint = match socket.bind(addr.as_ref()) {
            Ok(ce) => {
                debug!("[rpc] Listening at {}", &addr);
                ce
            }
            Err(e) => panic!("[rpc] Unable to bind Rep endpoint: {}\nError: {}", &addr, e),
        };
        (socket, endpoint)
    }

    /// Create a new `RPC` binding it to a remote Rep endpoint
    pub fn new(rtype: RPCType) -> RPC {
        let (socket, endpoint) = match rtype {
            RPCType::Client(addr) => RPC::connect_remote(RPC::create_socket(Protocol::Req), addr),
            RPCType::Server(addr) => RPC::bind(RPC::create_socket(Protocol::Rep), addr),
        };
        RPC {
            socket: socket,
            endpoint: endpoint,
        }
    }

    /// Run a remote command by sending a `message` to remote socket
    /// and reading a reply for the operation from the remote socket.
    pub fn execute(&mut self, m: Msg) -> Result<Msg> {
        debug!("[rpc] Executing: {:?}", &m);
        try!(self.send_msg(&m));
        let mut buff = String::new();
        try!(self.recv(&mut buff));
        match decode(&buff) {
            Some(msg) => Ok(msg),
            None => Ok(Msg::Error("Unable to decode message received from remote endpoint".to_string())),
        }
    }

    /// Receive from socket and write to given String
    pub fn recv(&mut self, buff: &mut String) -> Result<()> {
        try!(self.socket.read_to_string(buff));
        Ok(())
    }

    /// Send `Msg` on socket, after encoding `Msg` to byte vector.
    pub fn send_msg(&mut self, m: &Msg) -> Result<()> {
        try!(self.send(encode_bytes(m)));
        Ok(())
    }

    /// Send byte vector on socket
    pub fn send(&mut self, v: Vec<u8>) -> Result<()> {
        try!(self.socket.write_all(v.as_slice()));
        Ok(())
    }

    /// Shutdown endpoint
    pub fn close(mut self) -> Result<()> {
        try!(self.endpoint.shutdown());
        Ok(())
    }
}
