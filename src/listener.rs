use std::io::{Read, Write};

use crate::message::{parse_message_type, Message};

pub fn start_listener(bind_addr: &str) -> (std::sync::mpsc::Receiver<Message>, std::thread::JoinHandle<()>) {
    let (tx, rx) = std::sync::mpsc::channel::<Message>();
    let bind_addr = bind_addr.to_string();
    let listen_thread = std::thread::spawn(move || {
        let listener = std::net::TcpListener::bind(bind_addr).unwrap();
        for conn in listener.incoming() {
            if let Ok(mut conn) = conn {
                let mut msg_type: Vec<u8> = vec![0; 4];
                let _ = conn.read_exact(&mut msg_type);
                // dbg!(&msg_type);
                let mut msg_len: Vec<u8> = vec![0; 3];
                let _ = conn.read_exact(&mut msg_len);
                // dbg!(&msg_len);
                let msg_len = String::from_utf8_lossy(&msg_len).parse::<u32>().unwrap();
                let mut buf = vec![0; msg_len as usize];
                conn.read_exact(&mut buf).unwrap();
                // dbg!(&buf);
                let parser = parse_message_type(&msg_type).unwrap();
                let msg = parser(buf);
                tx.send(msg).unwrap();
                let _ = conn.write("1".as_bytes());
            }
        }
    });
    return (rx, listen_thread)
}
