use network::Stream;
use messages;

/// Run the gateway service
///
/// This function is called when you run `$ unicorn gateway`.
pub fn run() {
    println!("Running gateway...");

    // Address of the REP socket
    let core_addr = "127.0.0.1:60000".to_string();

    let mut stream = Stream::connect(core_addr).unwrap();

    let msg: messages::Msg = messages::Msg::Status {
        id: messages::common::ID {
            uuid: "_gateway".to_string(),
            component: messages::common::Components::Gateway,
        },
        state: messages::common::State::READY,
        msg: Some("Trying out stuff".to_string()),
    };

    stream.send(messages::encode(&msg).unwrap().as_bytes());

}
