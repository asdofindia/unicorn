use rpc::{RPC, RPCType};
use super::processor::process;

/// Run the core service
///
/// This function is called when you run `$ unicorn core`.
pub fn run() {
    debug!("Running core...");

    let mut crpc = RPC::new(RPCType::Server("ipc:///tmp/muktakosh-unicorn-core.ipc"));

    let mut buff = String::new();

    loop {
        match crpc.recv(&mut buff) {
            Ok(_) => {
                let reply: Vec<u8> = process(&buff);
                if reply.len() == 0 {
                    break;
                }
                crpc.send(reply).unwrap();
                buff.clear();
            },
            Err(e) => {
                error!("Got error while reading: {}", e);
                break;
            }
        }
    }

    match crpc.close() {
        Ok(_) => info!("Bye!"),
        Err(e) => panic!("Error while shutting down service. Reason: {}", e)
    }
}
