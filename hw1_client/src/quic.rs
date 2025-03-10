fn doesnt_work() {
    let mut endpoint = Endpoint::client("127.0.0.1:8124".parse::<SocketAddr>().unwrap()).unwrap();

    let mut roots = rustls::RootCertStore::empty();
    roots
        .add(CertificateDer::from_pem_file("/Users/alexst/Uni/PCD/cert.pem").unwrap())
        .unwrap();

    let client_config = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    let quinn_config = QuicClientConfig::try_from(Arc::new(client_config)).unwrap();

    let mut connection = endpoint
        .connect_with(
            ClientConfig::new(Arc::new(quinn_config)),
            "127.0.0.1:7880".parse::<SocketAddr>().unwrap(),
            "server_real_name",
        )
        .unwrap()
        .await
        .unwrap();

    let (mut send, mut recv) = connection
        .open_bi()
        .await
        .expect("Failed to open quic bidirectional stream!");

    send.write_all(b"I like pineapples, baby!")
        .await
        .expect("Failed to write to quic!");

    send.finish();
}
