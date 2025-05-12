use std::io::{Read, Write};
use zipline_mgr::BIND_ADDR_WINDS;

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        std::process::exit(1);
    }
    let mut sender = std::net::TcpStream::connect(BIND_ADDR_WINDS).unwrap();
    let str_arg = args.nth(1).unwrap();
    println!("sending {str_arg}");
    let mut buf = str_arg.into_bytes();
    sender.write(&buf).unwrap();
    let _ = sender.flush();
    println!("waiting...");
    let mut recv = vec![0; 3];
    sender.read_exact(&mut recv).unwrap();
    println!("{}", String::from_utf8_lossy(&recv));
}
