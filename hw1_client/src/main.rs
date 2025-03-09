use std::net::TcpStream;

use data::{DataHeader, DataPacket, generate_data};

mod data;

fn test_tcp(total_length: u32, package_size: u32) {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to open TCP stream!");

    let mut total_parts = total_length / package_size;
    if total_length % package_size > 0 {
        total_parts += 1;
    }

    let header = DataHeader {
        parts_count: total_parts,
    };
    let _ = bincode::encode_into_std_write(header, &mut stream, bincode::config::standard());

    let mut part = 0;
    while part < total_parts {
        let packet = DataPacket {
            part,
            data: generate_data(package_size),
        };
        let _ = bincode::encode_into_std_write(packet, &mut stream, bincode::config::standard());
        part += 1;
    }

    println!("Done with TCP test!")
}

fn main() {
    test_tcp(500 * 1000 * 1000, 1000);
    // let socket = UdpSocket::bind("127.0.0.1:8123").expect("couldn't bind to address");
    // socket.send_to(stringify!("I really like pineapples!").as_bytes(), "127.0.0.1:7879");

    // let mut endpoint = Endpoint::client("127.0.0.1:8124".parse::<SocketAddr>().unwrap()).unwrap();

    // let mut roots = rustls::RootCertStore::empty();
    // roots.add(CertificateDer::from_pem_file("/Users/alexst/Uni/PCD/cert.pem").unwrap()).unwrap();

    // let client_config = rustls::ClientConfig::builder()
    //     .with_root_certificates(roots)
    //     .with_no_client_auth();

    // let quinn_config = QuicClientConfig::try_from(Arc::new(client_config)).unwrap();

    // let mut connection = endpoint.connect_with(ClientConfig::new(Arc::new(quinn_config)),
    //      "127.0.0.1:7880".parse::<SocketAddr>().unwrap(), "server_real_name").unwrap().await.unwrap();

    // let (mut send, mut recv) = connection
    //     .open_bi()
    //     .await
    //     .expect("Failed to open quic bidirectional stream!");

    // send.write_all(b"I like pineapples, baby!").await.expect("Failed to write to quic!");
    // send.finish().expect("Failed to finish quic!");
}
