use std::io::{Read, Write};

use zipline_mgr::BIND_ADDR_WINDS as BIND_ADDR;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
    let listen_thread = std::thread::spawn(move || {
        let listener = std::net::TcpListener::bind(BIND_ADDR).unwrap();
        for conn in listener.incoming() {
            let mut buf: Vec<u8> = vec![];
            if let Ok(mut conn) = conn {
                let _ = conn.read_to_end(&mut buf).unwrap();
                tx.send(buf).unwrap();
                // println!("{}", String::from_utf8_lossy(&buf));
                // conn.write_all(&String::from("recv").into_bytes()).unwrap();
                // println!("responded");
            }
        }
    });
}
