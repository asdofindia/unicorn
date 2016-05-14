extern crate unicorn;

use unicorn::network::*;
use std::thread::spawn;
use std::io::prelude::*;

/// Processor for test. Implements an echo.
struct P {}

impl Processor for P {
    fn process(&self, s: String) -> Vec<u8> {
        println!("Got incoming string: {}", &s);
        if s == "KILL" {
            println!("Exiting. Killing Net.");
            return vec![]

        }
        Vec::from(&s[..])
    }
}

unsafe impl Send for P {}
unsafe impl Sync for P {}

#[test]
fn test_net_bind_recv_loop() {
    let net = spawn(move || {
        let n = Net::bind("127.0.0.1:61000".to_string()).unwrap();
        static TESTP: P = P {};
        n.recv(&TESTP);
    });

    let mut st = Stream::connect(&"127.0.0.1:61000".to_string(), true).unwrap();
    println!("Sending stream");
    st.send("Test loop".to_string().into_bytes());
    let _ = st.flush();

    loop {
        if let Some(s) = st.recv() {
            println!("Received");
            assert_eq!(s, "Test loop".to_string());
            break;
        }
    }
    drop(net);
}


#[test]
fn test_stream_multiple_message_one_connection() {
    let net = spawn(move || {
        let n = Net::bind("127.0.0.1:61001".to_string()).unwrap();
        static TESTP: P = P {};
        n.recv(&TESTP);
    });

    let mut st = Stream::connect(&"127.0.0.1:61001".to_string(), true).unwrap();
    println!("Sending stream");
    st.send("Test loop 1".to_string().into_bytes());
    st.send("Test loop 2".to_string().into_bytes());
    let _ = st.flush();

    let mut i = 0;
    loop {
        if i == 2 {
            break;
        }
        if let Some(s) = st.recv() {
            println!("Received back {}", &s);
            assert!(match s.as_ref() {
                "Test loop 1" => true,
                "Test loop 2" => true,
                _ => false
            });
            i += 1;
        }

    }

    drop(net);
}

#[test]
fn test_stream_drop_on_empty_processor_response() {
    let net = spawn(move || {
        let n = Net::bind("127.0.0.1:61002".to_string()).unwrap();
        static TESTP: P = P {};
        n.recv(&TESTP);
    });

    let mut st = Stream::connect(&"127.0.0.1:61002".to_string(), true).unwrap();
    st.send("KILL".to_string().into_bytes());
    let _ = st.flush();

    println!("Test loop after KILL");
    st.send("Test loop after KILL".to_string().into_bytes());
    let _ = st.flush();

    loop {
        assert_eq!(st.recv(), None);
        break;
    }
    drop(net);
}
