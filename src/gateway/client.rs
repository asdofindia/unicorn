extern crate zmq;

use std::thread;
use std::time::Duration;

/// Run the gateway service
///
/// This function is called when you run `$ unicorn gateway`.
pub fn run() {
    println!("Running gateway...");

    // Address of the REP socket
    let addr_rep = "tcp://127.0.0.1:69369";

    // Create a new ZMQ context
    let mut ctx = zmq::Context::new();

    // Create a REQ socket
    let mut req_socket = match ctx.socket(zmq::REQ) {
        Ok(socket) => socket,
        Err(e) => panic!("Error creating REQ socket: {:?}", e),
    };

    // Bind the REQ socket
    match req_socket.connect(addr_rep) {
        Ok(()) => println!("[gateway] connected to core at {}", addr_rep),
        Err(e) => panic!("Error connecting to REP socket at \"{}\": {:?}", addr_rep, e),
    }

    // We store messages in this buffer
    let mut msg = match zmq::Message::new() {
        Ok(msg) => msg,
        Err(e) => panic!("Error creating new message: {:?}", e)
    };

    // Loop to listen
    loop {
        // Send message
        match req_socket.send(b"Gateway PING", 0) {
            Ok(()) => println!("[gateway] Sending message: {}", "Gateway PING"),
            Err(e) => panic!("Error sending request: {:?}", e)
        }

        // Receive message
        match req_socket.recv(&mut msg, 0) {
            Ok(()) => println!("[gateway] Receiving message: {}", msg.as_str().unwrap()),
            Err(e) => panic!("Error decoding response message: {:?}", e)
        }
        thread::sleep(Duration::new(1, 0));
    }
}
