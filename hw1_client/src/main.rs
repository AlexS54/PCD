use std::{
    collections::HashSet,
    net::{TcpStream, UdpSocket},
    time::{Duration, SystemTime},
};

use clap::Parser;
use data::{ConfirmPacketVariant, DataHeader, DataPacket, UdpPacket, generate_data};
use options::Options;

mod data;
mod options;

fn compute_total_parts(total_length: u32, package_size: u32) -> u32 {
    let mut total_parts = total_length / package_size;
    if total_length % package_size > 0 {
        total_parts += 1;
    }

    total_parts
}

fn test_tcp(total_length: u32, package_size: u32) {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to open TCP stream!");

    let total_parts = compute_total_parts(total_length, package_size);

    let mut messages = 0u32;
    let mut bytes: usize = 0;

    let start = SystemTime::now();

    let header = DataHeader {
        parts_count: total_parts,
    };
    bytes += bincode::encode_into_std_write(header, &mut stream, bincode::config::standard())
        .expect("Failed to write header");
    messages += 1;

    let mut part = 0;
    while part < total_parts {
        let packet = DataPacket {
            part,
            data: generate_data(package_size),
        };
        bytes += bincode::encode_into_std_write(packet, &mut stream, bincode::config::standard())
            .expect("Failed to encode into TCP stream!");
        messages += 1;
        part += 1;
    }

    println!("Protocol: TCP, Messages: {}, Bytes: {}", messages, bytes);
    println!("Time: {:?}", SystemTime::elapsed(&start).unwrap());
}

fn send_udp_packet(packet: &UdpPacket, buffer: &mut [u8], socket: &UdpSocket) -> usize {
    let length = bincode::encode_into_slice(packet, buffer, bincode::config::standard())
        .expect("Failed to encode udp packet into buffer");

    socket
        .send_to(&buffer[..length], "127.0.0.1:7879")
        .expect("Failed to send to UDP socket!")
}

fn recv_confirm_packet(buffer: &mut [u8], socket: &UdpSocket) -> Option<ConfirmPacketVariant> {
    match socket.recv(buffer) {
        Ok(length) => {
            let decoded: (ConfirmPacketVariant, usize) =
                bincode::decode_from_slice(&buffer[..length], bincode::config::standard())
                    .expect("Failed to decode ConfirmPacketVariant!");
            return Some(decoded.0);
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::TimedOut => {
                return None;
            }
            _ => {
                panic!("Failed to receive buffer from UDP socket!")
            }
        },
    }
}

fn test_udp(total_length: u32, package_size: u32, stop_and_wait: bool) {
    let socket = UdpSocket::bind("127.0.0.1:8123").expect("couldn't bind to address");
    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("Failed to set read timeout!");

    let total_parts = compute_total_parts(total_length, package_size);

    let mut messages: u32 = 0;
    let mut bytes: usize = 0;

    let mut buffer = vec![0u8; package_size as usize + 128];
    let start = SystemTime::now();

    let header_packet = UdpPacket::Header(DataHeader {
        parts_count: total_parts,
    });
    loop {
        bytes += send_udp_packet(&header_packet, &mut buffer, &socket);
        messages += 1;
        if let Some(variant) = recv_confirm_packet(&mut buffer, &socket) {
            println!("Got confirm packet! {:?}", variant);
            break;
        };
    }

    for part in 0..total_parts {
        loop {
            let packet = UdpPacket::DataPart(DataPacket {
                part,
                data: generate_data(package_size),
            });
            bytes += send_udp_packet(&packet, &mut buffer, &socket);
            messages += 1;

            if !stop_and_wait {
                break;
            }

            if let Some(confirm) = recv_confirm_packet(&mut buffer, &socket) {
                if let ConfirmPacketVariant::Part(confirmed_part) = confirm {
                    if confirmed_part == part {
                        break;
                    }
                }
            }
        }
    }

    let packet = UdpPacket::Reset;
    bytes += send_udp_packet(&packet, &mut buffer, &socket);
    messages += 1;

    println!("Protocol: UDP, Messages: {}, Bytes: {}", messages, bytes);
    println!("Time: {:?}", SystemTime::elapsed(&start).unwrap());
}

fn main() {
    let args = Options::parse();
    println!("{:?}", args);

    let total_length = match args.transfer_amount {
        options::TransferAmount::Small => 1000 * 1000 * 500,
        options::TransferAmount::Big => 1000 * 1000 * 1000,
    };

    match args.protocol {
        options::Protocol::TCP => test_tcp(total_length, args.message_size),
        options::Protocol::UDP => test_udp(total_length, args.message_size, args.stop_and_wait),
    }
}
