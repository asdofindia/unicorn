use network::Stream;
use messages::{self, Msg, encode_bytes};

/// Run the gateway service
///
/// This function is called when you run `$ unicorn gateway`.
pub fn run() {
    debug!("Running gateway...");

    // Address of the REP socket
    let core_addr = "127.0.0.1:60000".to_string();

    let mut stream: Stream;

    match Stream::connect(&core_addr, true) {
        Ok(s) => stream = s,
        Err(e) => {
            debug!("[gateway] Unable to connect to core at {}. Reason: {}", &core_addr, e);
            return
        }
    };

    let msg: Msg = Msg::Status {
        id: messages::common::ID {
            uuid: "_gateway".to_string(),
            component: messages::common::Components::Gateway,
        },
        state: messages::common::State::READY,
        msg: Some("Trying out stuff".to_string()),
    };

    stream.send(encode_bytes(&msg));
}
