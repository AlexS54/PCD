use std::thread;

mod data;
mod quic;
mod tcp;
mod udp;

fn main() {
    let tcp_handle = thread::spawn(|| {
        tcp::start_tcp();
    });
    let udp_handle = thread::spawn(|| {
        udp::start_udp();
    });
    let quic_handle = thread::spawn(|| {
        quic::start_quic();
    });

    println!("Started server with all 3 protocols!");

    let _ = tcp_handle.join();
    let _ = udp_handle.join();
    let _ = quic_handle.join();
}
