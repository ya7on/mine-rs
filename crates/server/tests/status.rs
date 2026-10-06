use std::io::Cursor;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

use mclib::PacketFrame;
use mclib::packets::handshaking::serverbound::{Handshake, intent};
use mclib::packets::status::clientbound::{PongResponse, StatusResponse};
use mclib::packets::status::serverbound::{PingRequest, StatusRequest};
use mclib::{MCLong, MCType};
use serde_json::Value;
use server::config::StatusConfig;
use server::connection::Connection;

const STATUS: StatusConfig = StatusConfig {
    motd: String::new(),
    max_players: 0,
};

/// Accepts one connection and serves it with the test status settings.
struct TestServer {
    address: std::net::SocketAddr,
}

impl TestServer {
    async fn start() -> Self {
        Self::with_status(STATUS).await
    }

    async fn with_status(status: StatusConfig) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let _ = Connection::new(stream, status).run().await;
        });
        Self { address }
    }

    async fn connect(&self) -> TcpStream {
        TcpStream::connect(self.address).await.unwrap()
    }
}

fn handshake_frame() -> PacketFrame {
    let handshake = Handshake {
        protocol_version: 777.into(),
        server_address: "127.0.0.1".into(),
        server_port: 25_565,
        intent: intent::STATUS.into(),
    };
    PacketFrame::new(0, handshake.pack().unwrap())
}

fn status_request_frame() -> PacketFrame {
    PacketFrame::new(0, StatusRequest.pack().unwrap())
}

fn ping_request_frame(timestamp: i64) -> PacketFrame {
    let ping = PingRequest {
        timestamp: MCLong(timestamp),
    };
    PacketFrame::new(1, ping.pack().unwrap())
}

async fn send(stream: &mut TcpStream, frame: &PacketFrame) -> Result<(), std::io::Error> {
    frame.write(stream).await.map_err(std::io::Error::other)
}

/// Reads one reply frame. Returns `None` when the server closed the socket.
async fn read_reply(stream: &mut TcpStream) -> Result<Option<PacketFrame>, std::io::Error> {
    match timeout(Duration::from_secs(5), PacketFrame::read(stream))
        .await
        .unwrap()
    {
        Ok(frame) => Ok(Some(frame)),
        Err(mclib::ProtocolError::Io(error))
            if error.kind() == std::io::ErrorKind::UnexpectedEof =>
        {
            Ok(None)
        }
        Err(error) => Err(std::io::Error::other(error)),
    }
}

async fn read_closes(stream: &mut TcpStream) {
    // The server must close the socket, yielding a clean EOF.
    let mut byte = [0_u8; 1];
    match timeout(Duration::from_secs(5), stream.read(&mut byte))
        .await
        .unwrap()
    {
        Ok(0) => {}
        other => panic!("expected EOF, got {other:?}"),
    }
}

#[tokio::test]
async fn completes_the_full_status_exchange() {
    let server = TestServer::start().await;
    let mut stream = server.connect().await;

    send(&mut stream, &handshake_frame()).await.unwrap();
    send(&mut stream, &status_request_frame()).await.unwrap();
    let status = StatusResponse::unpack(&mut Cursor::new(
        read_reply(&mut stream).await.unwrap().unwrap().body,
    ))
    .unwrap();
    let status: Value =
        serde_json::from_str(status.json_response.as_ref()).expect("status is valid JSON");
    assert_eq!(status["version"]["protocol"], 777);
    assert_eq!(status["version"]["name"], "mine-rs");
    assert_eq!(status["players"]["online"], 0);
    send(&mut stream, &ping_request_frame(1_234)).await.unwrap();
    let pong = PongResponse::unpack(&mut Cursor::new(
        read_reply(&mut stream).await.unwrap().unwrap().body,
    ))
    .unwrap();
    assert_eq!(pong.timestamp, MCLong(1_234));

    read_closes(&mut stream).await;
}

#[tokio::test]
async fn reports_the_configured_status_settings() {
    let server = TestServer::with_status(StatusConfig {
        motd: "test motd".to_owned(),
        max_players: 42,
    })
    .await;
    let mut stream = server.connect().await;

    send(&mut stream, &handshake_frame()).await.unwrap();
    send(&mut stream, &status_request_frame()).await.unwrap();
    let reply = read_reply(&mut stream).await.unwrap().unwrap();
    let status = StatusResponse::unpack(&mut Cursor::new(reply.body)).unwrap();
    let document: Value =
        serde_json::from_str(status.json_response.as_ref()).expect("status is valid JSON");

    assert_eq!(document["description"]["text"], "test motd");
    assert_eq!(document["players"]["max"], 42);
}

#[tokio::test]
async fn closes_connections_with_an_unsupported_transfer_intent() {
    let server = TestServer::start().await;
    let mut stream = server.connect().await;

    let login_handshake = Handshake {
        protocol_version: 777.into(),
        server_address: "127.0.0.1".into(),
        server_port: 25_565,
        intent: intent::TRANSFER.into(),
    };
    send(
        &mut stream,
        &PacketFrame::new(0, login_handshake.pack().unwrap()),
    )
    .await
    .unwrap();

    read_closes(&mut stream).await;
}

#[tokio::test]
async fn closes_connections_that_send_a_second_status_request() {
    let server = TestServer::start().await;
    let mut stream = server.connect().await;

    send(&mut stream, &handshake_frame()).await.unwrap();
    send(&mut stream, &status_request_frame()).await.unwrap();
    assert!(read_reply(&mut stream).await.unwrap().is_some());

    // The protocol allows only one status request per connection.
    send(&mut stream, &status_request_frame()).await.unwrap();
    read_closes(&mut stream).await;
}

#[tokio::test]
async fn completes_a_ping_only_exchange() {
    let server = TestServer::start().await;
    let mut stream = server.connect().await;

    send(&mut stream, &handshake_frame()).await.unwrap();
    send(&mut stream, &ping_request_frame(1)).await.unwrap();
    let reply = read_reply(&mut stream).await.unwrap().unwrap();
    assert_eq!(reply.packet_id.0, 1);
    let pong = PongResponse::unpack(&mut Cursor::new(reply.body)).unwrap();
    assert_eq!(pong.timestamp, MCLong(1));
    read_closes(&mut stream).await;
}

#[tokio::test]
async fn closes_connections_sending_an_unknown_packet_in_the_status_state() {
    let server = TestServer::start().await;
    let mut stream = server.connect().await;

    send(&mut stream, &handshake_frame()).await.unwrap();
    send(&mut stream, &PacketFrame::new(2, Vec::new()))
        .await
        .unwrap();
    read_closes(&mut stream).await;
}

#[tokio::test]
async fn closes_connections_that_speak_before_the_handshake() {
    let server = TestServer::start().await;
    let mut stream = server.connect().await;

    send(&mut stream, &status_request_frame()).await.unwrap();
    read_closes(&mut stream).await;
}

#[tokio::test]
async fn closes_connections_after_pong() {
    let server = TestServer::start().await;
    let mut stream = server.connect().await;

    send(&mut stream, &handshake_frame()).await.unwrap();
    send(&mut stream, &status_request_frame()).await.unwrap();
    assert!(read_reply(&mut stream).await.unwrap().is_some());
    send(&mut stream, &ping_request_frame(7)).await.unwrap();
    assert!(read_reply(&mut stream).await.unwrap().is_some());

    read_closes(&mut stream).await;
}

#[tokio::test]
async fn completes_exchange_with_frames_sent_together() {
    let server = TestServer::start().await;
    let mut stream = server.connect().await;
    let bytes: Vec<u8> = [
        handshake_frame(),
        status_request_frame(),
        ping_request_frame(9),
    ]
    .iter()
    .flat_map(|frame| frame.pack().unwrap())
    .collect();
    stream.write_all(&bytes).await.unwrap();
    assert_eq!(
        read_reply(&mut stream).await.unwrap().unwrap().packet_id.0,
        0
    );
    let pong = read_reply(&mut stream).await.unwrap().unwrap();
    assert_eq!(
        PongResponse::unpack(&mut Cursor::new(pong.body))
            .unwrap()
            .timestamp,
        MCLong(9)
    );
    read_closes(&mut stream).await;
}

#[tokio::test]
async fn stalled_connection_does_not_block_another_exchange() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let serving = tokio::spawn(async move {
        let (idle, _) = listener.accept().await.unwrap();
        let idle_task = tokio::spawn(async move {
            let _ = Connection::new(idle, STATUS).run().await;
        });
        let (active, _) = listener.accept().await.unwrap();
        Connection::new(active, STATUS).run().await.unwrap();
        idle_task.abort();
        let _ = idle_task.await;
    });
    let mut idle = TcpStream::connect(address).await.unwrap();
    idle.write_all(&[0x80]).await.unwrap();
    let mut stream = TcpStream::connect(address).await.unwrap();
    send(&mut stream, &handshake_frame()).await.unwrap();
    send(&mut stream, &status_request_frame()).await.unwrap();
    assert!(read_reply(&mut stream).await.unwrap().is_some());
    send(&mut stream, &ping_request_frame(7)).await.unwrap();
    assert!(read_reply(&mut stream).await.unwrap().is_some());
    read_closes(&mut stream).await;
    serving.await.unwrap();
}
