use std::io::{Read, Write};
use zipline_mgr::BIND_ADDR_WINDS;

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        std::process::exit(1);
    }
    let mut sender = std::net::TcpStream::connect(BIND_ADDR_WINDS).unwrap();
    let str_arg = args.nth(1).unwrap();
    print!("sending {str_arg}...");
    let buf = str_arg.into_bytes();
    sender.write(&buf).unwrap();
    let mut recv = vec![0; 1];
    sender.read_exact(&mut recv).unwrap();
    println!(" success!");
}

