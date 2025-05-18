use zipline_mgr::{listener::start_listener, BIND_ADDR_WINDS as BIND_ADDR};

fn main() {
    let (rx, listen_thread) = start_listener(BIND_ADDR);
    loop {
        if let Ok(msg) = rx.try_recv() {
            println!("Received: {}", &msg);
        }
    }
    // listen_thread.join();
}

