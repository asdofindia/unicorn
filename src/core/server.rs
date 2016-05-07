use network::Net;

use super::processor::ProcessMsg;

/// Run the core service
///
/// This function is called when you run `$ unicorn core`.
pub fn run() {
    println!("Running core...");

    // Address of the listener socket
    // Max port: 65535 (u16 MAX)
    let addr = "127.0.0.1:60000".to_string();

    match Net::bind(addr) {
        Ok(net) => {
            println!("[core] Listening on {}", net.addr());
            static P: ProcessMsg = ProcessMsg{};
            net.recv(&P);
        }
        Err(_) => println!("[core] Error binding listener"),
    }
}
