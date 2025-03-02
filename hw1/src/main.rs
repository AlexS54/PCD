use std::thread;

mod tcp;
mod udp;

fn main() {
    let tcp_handle = thread::spawn(|| {
        tcp::start_tcp();
    });
    let udp_handle = thread::spawn(|| {
        udp::start_udp();
    });

    let _ = tcp_handle.join();
    let _ = udp_handle.join();
}
