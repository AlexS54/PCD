use bincode::decode_from_std_read;

use crate::data::{DataHeader, DataPacket};
use std::{
    net::{TcpListener, TcpStream},
    thread,
};

fn size_of_vector<T>(vector: &Vec<T>) -> usize {
    std::mem::size_of::<Vec<T>>() + vector.capacity() * std::mem::size_of::<T>()
}

fn size_of_data_packet(packet: &DataPacket) -> usize {
    size_of::<u32>() + size_of_vector(&packet.data)
}

pub fn start_tcp() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut messages_received = 0u32;
    let mut bytes_received: usize = 0;

    let header: DataHeader = decode_from_std_read(&mut stream, bincode::config::standard())
        .expect("Did not decode data header!");
    messages_received += 1;
    bytes_received += size_of_val(&header);

    println!("Got header with length {}", header.parts_count);

    let mut parts_read = 0;
    while parts_read < header.parts_count {
        let data_packet: DataPacket =
            decode_from_std_read(&mut stream, bincode::config::standard())
                .expect("Failed to decode data packet!");

        messages_received += 1;
        bytes_received += size_of_data_packet(&data_packet);

        parts_read += 1;
    }

    println!(
        "Protocol: TCP, Messages: {}, Bytes: {}",
        messages_received, bytes_received
    );
}
