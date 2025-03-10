use std::net::UdpSocket;

use crate::data::{ConfirmPacketVariant, UdpPacket};

fn get_confirm_variant(received: &UdpPacket) -> Option<ConfirmPacketVariant> {
    match received {
        UdpPacket::Header(_) => Some(ConfirmPacketVariant::Header),
        UdpPacket::DataPart(data_packet) => Some(ConfirmPacketVariant::Part(data_packet.part)),
        UdpPacket::Reset => None,
    }
}

pub fn start_udp() {
    {
        let socket = UdpSocket::bind("127.0.0.1:7879").expect("Failed to bind udp socket");
        let mut messages = 0u32;
        let mut bytes: usize = 0;

        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 65535 + 1024];
            let (amt, src) = socket
                .recv_from(&mut buf)
                .expect("Failed to received UDP packet");
            messages += 1;
            bytes += amt;

            let received: (UdpPacket, usize) =
                bincode::decode_from_slice(&buf[..amt], bincode::config::standard())
                    .expect("Failed to decode UdpPacket");

            // println!("Received {:?}", received.0);
            if let UdpPacket::Reset = received.0 {
                println!("Protocol: UDP, Messages: {}, Bytes: {}", messages, bytes);
                messages = 0;
                bytes = 0;
            }

            if let Some(confirm_variant) = get_confirm_variant(&received.0) {
                let confirm_length = bincode::encode_into_slice(
                    confirm_variant,
                    &mut buf,
                    bincode::config::standard(),
                )
                .expect("Failed to encode confirm variant!");

                socket
                    .send_to(&buf[..confirm_length], src)
                    .expect("Failed to send response!");
            }
        }
    } // the socket is closed here
}
