use rpc::{RPC, RPCType};
use messages::Msg;

/// Run the gateway service
///
/// This function is called when you run `$ unicorn gateway`.
pub fn run() {
    debug!("Running gateway...");

    let mut core_rpc = RPC::new(RPCType::Client("ipc:///tmp/muktakosh-unicorn-core.ipc"));

    let reply = core_rpc.execute(Msg::Ok).unwrap();

    println!("Received from core: \n\t{:?}\n", reply);

}
