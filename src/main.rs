use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8443").unwrap();
    let identity = get_identity();
    let acceptor = TlsAcceptor::new(identity).unwrap();
    let acceptor = Arc::new(acceptor);
    
    fn handle_client(mut stream: TlsStream<TcpStream>) {
        let _peer_cert = stream.peer_certificate().unwrap();
        println!("Peer certificate: {:#?}", _peer_cert.map(|c| c.to_der().unwrap()));
        let mut buf = [0; 1024];
        let read = stream.read(&mut buf).unwrap();
        let received = std::str::from_utf8(&buf[0..read]).unwrap();
        stream
            .write_all(format!("received '{}'", received).as_bytes())
            .unwrap();
    }
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let stream = acceptor.accept(stream).unwrap();
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Err: {e}")
             }
        }
    }
}

fn get_identity() -> Identity {
    let mut identity = File::open("./certs/identity.p12").expect("identity file must exist");
    let mut certs = vec![];
    identity.read_to_end(&mut certs).expect("should be able to read contents of identity file");
    let identity = Identity::from_pkcs12(&certs, "ACSRS").expect("password should match");
    identity
}