//! Internal communication distribution abstraction for `unicorn`.

pub use nanomsg::Protocol;

use nanomsg::{Socket, Endpoint, Result};
use messages::{Msg, encode_bytes, decode};
use std::io::Read;

pub enum RPCType {
    Connect,
    Bind
}

/// Performs RPC-like data exchange over `nanomsg::Protocol::Rep` sockets.
pub struct RPC {
    socket: Socket,
    endpoint: Endpoint,
    socket_type: Protocol
}

impl RPC {
    fn create_socket(socktype: Protocol) -> Socket {
        match Socket::new(socktype) {
            Ok(s) => s,
            Err(e) => panic!("[rpc] Error creating {:?} socket. Error: {}", socktype, e),
        }
    }

    fn create_device_socket(socktype: Protocol) -> Socket {
        match Socket::new_for_device(socktype) {
            Ok(s) => s,
            Err(e) => panic!("[rpc] Error creating device socket for {:?}. Error: {}", socktype, e),
        }
    }

    fn connect(mut socket: Socket, addr: &str) -> (Socket, Endpoint) {
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

    fn generate(socket: Socket, sock_type: Protocol, conn_type: RPCType, addr: &str) -> RPC {
        let (socket, endpoint) = match conn_type {
            RPCType::Connect => RPC::connect(socket, addr),
            RPCType::Bind => RPC::bind(socket, addr),
        };
        RPC {
            socket: socket,
            endpoint: endpoint,
            socket_type: sock_type
        }
    }

    /// Link two sockets using a `nanomsg`
    /// [`device`](http://nanomsg.org/v0.8/nn_device.3.HTML).
    pub fn link(&self, s2: &RPC) -> Result<()> {
        Socket::device(&self.socket, &s2.socket)
    }

    /// Create a new `RPC` instance
    pub fn new(sock_type: Protocol, conn_type: RPCType, addr: &str) -> RPC {
        let socket = RPC::create_socket(sock_type);
        RPC::generate(socket, sock_type, conn_type, addr)
    }

    /// Create a new `RPC` instance for use in `nanomsg` `device`
    pub fn new_for_device(sock_type: Protocol, conn_type: RPCType, addr: &str) -> RPC {
        let socket = RPC::create_device_socket(sock_type);
        RPC::generate(socket, sock_type, conn_type, addr)
    }

    /// Run a remote command by sending a `message` to remote socket
    /// and reading a reply for the operation from the remote socket.
    pub fn execute(&mut self, m: Msg) -> Result<Msg> {
        debug!("[rpc] Executing: {:?}", &m);
        try!(self.send_msg(&m));
        let mut buff = String::new();
        try!(self.recv(&mut buff));
        debug!("[rpc] Received execution response: {}", &buff);
        match decode(&buff) {
            Some(msg) => Ok(msg),
            None => Ok(Msg::Error("Unable to decode message received from remote endpoint".to_string())),
        }
    }

    /// Receive from socket and write to given String
    pub fn recv(&mut self, buff: &mut String) -> Result<()> {
        debug!("[rpc] Attempting to read socket.");
        try!(self.socket.read_to_string(buff));
        Ok(())
    }

    /// Send `Msg` on socket, after encoding `Msg` to byte vector.
    pub fn send_msg(&mut self, m: &Msg) -> Result<()> {
        debug!("[rpc] Attempting to send Msg on socket.");
        try!(self.send(encode_bytes(m)));
        Ok(())
    }

    /// Send byte vector on socket
    pub fn send(&mut self, v: Vec<u8>) -> Result<()> {
        debug!("[rpc] Attempting to send {} bytes on socket.", v.len());
        try!(self.socket.nb_write(v.as_slice()));
        Ok(())
    }

    /// Shutdown endpoint
    pub fn close(mut self) -> Result<()> {
        debug!("[rpc] Attempting to shutdown socket.");
        try!(self.endpoint.shutdown());
        Ok(())
    }

    /// Return socket type
    pub fn get_socket_type(&self) -> Protocol {
        self.socket_type
    }
}
