use std::net::UdpSocket;

fn print_buffer(buf: &[u8]) {
    if let Ok(res) = std::str::from_utf8(buf) {
        println!("{}", res);
    }
}

pub fn start_udp() {
    {
        let socket = UdpSocket::bind("127.0.0.1:7879").expect("Failed to bind udp socket");

        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 512];
            let (amt, src) = socket
                .recv_from(&mut buf)
                .expect("Failed to received UDP packet");

            print_buffer(&buf);

            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            let buf = &mut buf[..amt];
            buf.reverse();
            let _ = socket.send_to(buf, &src);
        }
    } // the socket is closed here
}
