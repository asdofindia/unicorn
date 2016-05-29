extern crate unicorn;

use unicorn::network::*;
use std::thread::spawn;
use std::io::prelude::*;
use std::sync::mpsc::channel;

/// Processor for test. Implements an echo.
struct P {}

impl Processor for P {
    fn process(&self, s: String) -> Vec<u8> {
        if s == "KILL" {
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
        let mut n = Net::bind("127.0.0.1:61000".to_string()).unwrap();
        n.num_workers(1);
        static TESTP: P = P {};
        n.recv(&TESTP);
    });

    let mut st = Stream::connect(&"127.0.0.1:61000".to_string(), true).unwrap();
    st.send("Test loop".to_string().into_bytes());
    let _ = st.flush();

    loop {
        if let Some(s) = st.recv() {
            assert_eq!(s, "Test loop".to_string());
            break;
        }
    }
    drop(net);
}


#[test]
fn test_stream_multiple_message_one_connection() {
    let net = spawn(move || {
        let mut n = Net::bind("127.0.0.1:61001".to_string()).unwrap();
        n.num_workers(1);
        static TESTP: P = P {};
        n.recv(&TESTP);
    });

    let mut st = Stream::connect(&"127.0.0.1:61001".to_string(), true).unwrap();
    st.send("Test loop 1".to_string().into_bytes());
    st.send("Test loop 2".to_string().into_bytes());
    let _ = st.flush();

    let mut i = 0;
    loop {
        if i == 2 {
            break;
        }
        if let Some(s) = st.recv() {
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
        let mut n = Net::bind("127.0.0.1:61002".to_string()).unwrap();
        n.num_workers(1);
        static TESTP: P = P {};
        n.recv(&TESTP);
    });

    let mut st = Stream::connect(&"127.0.0.1:61002".to_string(), true).unwrap();
    st.send("KILL".to_string().into_bytes());
    let _ = st.flush();

    st.send("Test loop after KILL".to_string().into_bytes());
    let _ = st.flush();

    loop {
        assert_eq!(st.recv(), None);
        break;
    }
    drop(net);
}


fn run_multiple_message_multiple_connection_async(addr: String, num_workers: usize, num_conn: i64) {
    let addr_spawn = addr.clone();
    let net = spawn(move || {
        let mut n = Net::bind(addr_spawn).unwrap();
        n.num_workers(num_workers);
        static TESTP: P = P {};
        n.recv(&TESTP);
    });

    let (tx, rx) = channel::<(i64, Stream)>();
    let conn_spawn = num_conn.clone();
    let connector = spawn(move || {
        for i in 0..conn_spawn {
            tx.send((i, Stream::connect(&addr, true).unwrap())).unwrap();
        }
        drop(tx);
    });

    let mut c: i64 = 0;
    let mut x = rx.iter();
    loop {
        if let Some((i, mut stream)) = x.next() {
            for sr in 0..10 {
                let s = format!("Test loop {} - {}", &i, &sr);
                stream.send(s.clone().into_bytes());
            }
            let _ = stream.flush();
            for sr in 0..10 {
                assert_eq!(stream.recv(), Some(format!("Test loop {} - {}", &i, &sr)));
            }
            stream.send("KILL".to_string().into_bytes());
            c += 1;
        } else {
            if c < num_conn - 1 {
                panic!(format!("Premature stop of test at {}", &c));
            }
            break;
        }
    }

    connector.join().unwrap();

    drop(net);
}


#[test]
fn test_stream_multiple_message_multiple_connection_short_async() {
    run_multiple_message_multiple_connection_async("127.0.0.1:61004".to_string(), 4, 500);
}

#[test]
#[ignore]
fn test_stream_multiple_message_multiple_connection_long_async() {
    run_multiple_message_multiple_connection_async("127.0.0.1:61005".to_string(), 10, 10000);
}

fn run_multiple_message_multiple_connection_sync(addr: String, num_workers: usize, num_conn: i64) {
    let addr_spawn = addr.clone();
    let net = spawn(move || {
        let mut n = Net::bind(addr_spawn).unwrap();
        n.num_workers(num_workers);
        static TESTP: P = P {};
        n.recv(&TESTP);
    });

    for i in 0..num_conn {
        let mut stream = Stream::connect(&addr, true).unwrap();
        for sr in 0..10 {
            let s = format!("Test loop {} - {}", &i, &sr);
            stream.send(s.clone().into_bytes());
        }
        let _ = stream.flush();
        for sr in 0..10 {
            assert_eq!(stream.recv(), Some(format!("Test loop {} - {}", &i, &sr)));
        }
        stream.send("KILL".to_string().into_bytes());
    }

    drop(net);
}

#[test]
fn test_stream_multiple_message_multiple_connection_short_sync() {
    run_multiple_message_multiple_connection_sync("127.0.0.1:61006".to_string(), 4, 500);
}

#[test]
#[ignore]
fn test_stream_multiple_message_multiple_connection_long_sync() {
    run_multiple_message_multiple_connection_sync("127.0.0.1:61007".to_string(), 10, 10000);
}
