use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use mclib::PacketFrame;
use mclib::packets::handshaking::serverbound::{Handshake, intent};
use mclib::packets::status::clientbound::{PongResponse, StatusResponse};
use mclib::packets::status::serverbound::{PingRequest, StatusRequest};
use mclib::types::{MCLong, MCType};
use server::connection::Connection;

const STATUS_JSON: &str = r#"{"version":{"name":"mine-rs","protocol":777}}"#;

/// Accepts one connection and serves it with a test status document.
struct TestServer {
    address: std::net::SocketAddr,
}

impl TestServer {
    fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();

        thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            let mut connection = Connection::new(stream, STATUS_JSON.to_owned()).unwrap();
            let _ = connection.run();
        });

        Self { address }
    }

    fn connect(&self) -> TcpStream {
        TcpStream::connect(self.address).unwrap()
    }
}

fn handshake_frame() -> PacketFrame {
    let handshake = Handshake {
        protocol_version: 777.into(),
        server_address: "127.0.0.1".into(),
        server_port: 25_565,
        intent: intent::STATUS.into(),
    };
    PacketFrame::new(0, handshake.pack().unwrap()).unwrap()
}

fn status_request_frame() -> PacketFrame {
    PacketFrame::new(0, StatusRequest.pack().unwrap()).unwrap()
}

fn ping_request_frame(timestamp: i64) -> PacketFrame {
    let ping = PingRequest {
        timestamp: MCLong(timestamp),
    };
    PacketFrame::new(1, ping.pack().unwrap()).unwrap()
}

fn send(stream: &mut TcpStream, frame: &PacketFrame) -> Result<(), std::io::Error> {
    stream.write_all(&frame.pack().unwrap())?;
    stream.flush()
}

/// Reads one reply frame. Returns `None` when the server closed the socket.
fn read_reply(stream: &mut TcpStream) -> Result<Option<PacketFrame>, std::io::Error> {
    match PacketFrame::read(stream) {
        Ok(frame) => Ok(Some(frame)),
        Err(mclib::types::ProtocolError::Io(error))
            if error.kind() == std::io::ErrorKind::UnexpectedEof =>
        {
            Ok(None)
        }
        Err(error) => Err(std::io::Error::other(error)),
    }
}

fn read_closes(stream: &mut TcpStream) {
    // The server must close the socket, yielding a clean EOF.
    let mut byte = [0_u8; 1];
    match stream.read(&mut byte) {
        Ok(0) => {}
        other => panic!("expected EOF, got {other:?}"),
    }
}

#[test]
fn completes_the_full_status_exchange() {
    let server = TestServer::start();
    let mut stream = server.connect();

    send(&mut stream, &handshake_frame()).unwrap();
    send(&mut stream, &status_request_frame()).unwrap();
    let status = StatusResponse::unpack(&mut Cursor::new(
        read_reply(&mut stream).unwrap().unwrap().body,
    ))
    .unwrap();
    assert_eq!(status.json_response.as_ref(), STATUS_JSON);

    send(&mut stream, &ping_request_frame(1_234)).unwrap();
    let pong = PongResponse::unpack(&mut Cursor::new(
        read_reply(&mut stream).unwrap().unwrap().body,
    ))
    .unwrap();
    assert_eq!(pong.timestamp, MCLong(1_234));

    read_closes(&mut stream);
}

#[test]
fn closes_connections_without_a_status_intent() {
    let server = TestServer::start();
    let mut stream = server.connect();

    let login_handshake = Handshake {
        protocol_version: 777.into(),
        server_address: "127.0.0.1".into(),
        server_port: 25_565,
        intent: intent::LOGIN.into(),
    };
    send(
        &mut stream,
        &PacketFrame::new(0, login_handshake.pack().unwrap()).unwrap(),
    )
    .unwrap();

    read_closes(&mut stream);
}

#[test]
fn closes_connections_that_send_a_second_status_request() {
    let server = TestServer::start();
    let mut stream = server.connect();

    send(&mut stream, &handshake_frame()).unwrap();
    send(&mut stream, &status_request_frame()).unwrap();
    assert!(read_reply(&mut stream).unwrap().is_some());

    // The protocol allows only one status request per connection.
    send(&mut stream, &status_request_frame()).unwrap();
    read_closes(&mut stream);
}

#[test]
fn closes_connections_sending_an_unknown_packet_in_the_status_state() {
    let server = TestServer::start();
    let mut stream = server.connect();

    send(&mut stream, &handshake_frame()).unwrap();
    send(&mut stream, &ping_request_frame(1)).unwrap();
    read_closes(&mut stream);
}

#[test]
fn closes_connections_that_speak_before_the_handshake() {
    let server = TestServer::start();
    let mut stream = server.connect();

    send(&mut stream, &status_request_frame()).unwrap();
    read_closes(&mut stream);
}

#[test]
fn closes_connections_after_pong() {
    let server = TestServer::start();
    let mut stream = server.connect();

    send(&mut stream, &handshake_frame()).unwrap();
    send(&mut stream, &status_request_frame()).unwrap();
    assert!(read_reply(&mut stream).unwrap().is_some());
    send(&mut stream, &ping_request_frame(7)).unwrap();
    assert!(read_reply(&mut stream).unwrap().is_some());

    read_closes(&mut stream);
}
