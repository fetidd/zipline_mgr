use std::io::{Read, Write};
use zipline_mgr::BIND_ADDR_WINDS;

fn main() {
    let mut args = std::env::args();
    if args.len() < 2 {
        std::process::exit(1);
    }
    let mut sender = std::net::TcpStream::connect(BIND_ADDR_WINDS).unwrap();
    let str_arg = args.nth(1).unwrap();
    let buf = str_arg.into_bytes();
    sender.write_all(&buf).unwrap();
    println!("waiting...");
    let mut recv = vec![];
    sender.read(&mut recv).unwrap();
    println!("{}", "done");
}
