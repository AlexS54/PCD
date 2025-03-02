use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8123").expect("couldn't bind to address");
    socket.send_to(stringify!("I really like pineapples!").as_bytes(), "127.0.0.1:7879");
}
