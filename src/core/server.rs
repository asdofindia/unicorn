extern crate zmq;

/// Run the core service
///
/// This function is called when you run `$ unicorn core`.
pub fn run() {
    println!("Running core...");

    // Address of the REP socket
    let addr_rep = "tcp://*:69369";

    // Create a new ZMQ context
    let mut ctx = zmq::Context::new();

    // Create a REP socket
    let mut rep_socket = match ctx.socket(zmq::REP) {
        Ok(socket) => socket,
        Err(e) => panic!("Error creating REP socket: {:?}", e),
    };

    // Bind the REP socket
    match rep_socket.bind(addr_rep) {
        Ok(()) => println!("core is listening on {}", addr_rep),
        Err(e) => panic!("Error binding REP socket to \"{}\": {:?}", addr_rep, e),
    }

    // We store messages in this buffer
    let mut msg = match zmq::Message::new() {
        Ok(msg) => msg,
        Err(e) => panic!("Error creating new message: {:?}", e),
    };

    // Loop to listen
    loop {
        match rep_socket.recv(&mut msg, 0) {
            Ok(()) => println!("[core] Received request: {}", msg.as_str().unwrap()),
            Err(e) => panic!("Error decoding request message: {:?}", e),
        }
        match rep_socket.send(b"ACK", 0) {
            Ok(()) => println!("[core] Sending response: \"{}\"", "ACK"),
            Err(e) => panic!("Error sending response: {:?}", e),
        }
    }
}
