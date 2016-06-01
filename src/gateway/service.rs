use rpc::{RPC, RPCType, Protocol};
use messages::Msg;

/// Run the gateway service
///
/// This function is called when you run `$ unicorn gateway`.
pub fn run() {
    debug!("[gateway] Starting gateway...");

    let mut core_rpc = RPC::new(Protocol::Req, RPCType::Connect, "ipc:///tmp/muktakosh-unicorn-core.ipc");

    let reply = core_rpc.execute(Msg::Ok).unwrap();

    println!("Received from core: \n\t{:?}\n", reply);

}
