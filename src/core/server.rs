use network::Net;
use super::processor;

/// Run the core service
///
/// This function is called when you run `$ unicorn core`.
pub fn run() {
    println!("Running core...");

    // Address of the listener socket
    // Max port: 65535 (u16 MAX)
    let addr = "127.0.0.1:60000".to_string();

    let mut net = Net::new(&addr);
    match net.bind() {
        Ok(_) => {
            println!("[core] Listening on {}", &addr);
            net.recv(&processor::process_msg);
        }
        Err(_) => println!("[core] Error binding listener"),
    }
}
