use std::{error::Error, fs::File, io::BufReader, net::SocketAddr};

use quinn::{Connection, Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls_pemfile;
use tokio::{self, spawn};

pub fn read_certs_from_file()
-> Result<(Vec<CertificateDer<'static>>, PrivateKeyDer<'static>), Box<dyn Error>> {
    let mut cert_chain_reader = BufReader::new(File::open("../cert.pem")?);
    let certs: Vec<CertificateDer<'static>> =
        rustls_pemfile::certs(&mut cert_chain_reader).collect::<Result<_, _>>()?;

    let mut key_reader = BufReader::new(File::open("../key.pem")?);
    // if the file starts with "BEGIN RSA PRIVATE KEY"
    // let mut keys = rustls_pemfile::rsa_private_keys(&mut key_reader)?;
    // if the file starts with "BEGIN PRIVATE KEY"
    let mut keys: Vec<PrivatePkcs8KeyDer> =
        rustls_pemfile::pkcs8_private_keys(&mut key_reader).collect::<Result<Vec<_>, _>>()?;

    assert_eq!(keys.len(), 1);
    let key = PrivateKeyDer::from(keys.remove(0));

    Ok((certs, key))
}

async fn handle_connection(mut connection: Connection) {
    let (mut send, mut recv) = connection
        .open_bi()
        .await
        .expect("Failed to open quic bidirectional stream!");

    let received = recv.read_to_end(64).await;
    if let Ok(data) = received {
        if let Ok(message) = String::from_utf8(data) {
            println!("Received message, {}", message);
        }
    }
}

#[tokio::main]
pub async fn start_quic() {
    let (certs, key) = read_certs_from_file().expect("Failed to read certs from files");
    let server_config =
        ServerConfig::with_single_cert(certs, key).expect("Failed to create quic server config");

    let endpoint = Endpoint::server(
        server_config,
        "127.0.0.1:7880".parse::<SocketAddr>().unwrap(),
    )
    .expect("Failed to start quic server!");

    // Start iterating over incoming connections.
    while let Some(conn) = endpoint.accept().await {
        let mut connection = conn.await.expect("Failed to get quic connection!");

        println!("Got quic connection!");

        spawn(handle_connection(connection));
    }
}
