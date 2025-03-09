use bincode::decode_from_std_read;

use crate::data::{DataHeader, DataPacket};
use std::{
    net::{TcpListener, TcpStream},
    thread,
};

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
    let header: DataHeader = decode_from_std_read(&mut stream, bincode::config::standard())
        .expect("Did not decode data header!");

    println!("Got header with length {}", header.parts_count);

    let mut parts_read = 0;
    while parts_read < header.parts_count {
        let _: DataPacket = decode_from_std_read(&mut stream, bincode::config::standard())
            .expect("Failed to decode data packet!");
        parts_read += 1;
    }

    println!("Done with all parts!");
}
