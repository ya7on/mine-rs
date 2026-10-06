use std::time::Duration;

use mclib::packets::handshaking::serverbound::{Handshake, intent};
use mclib::packets::login::clientbound::{Disconnect, LoginSuccess};
use mclib::packets::login::serverbound::{LoginAcknowledged, LoginStart};
use mclib::{MCType, PacketFrame};
use server::config::StatusConfig;
use server::connection::Connection;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

async fn connect(version: i32) -> TcpStream {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let _ = Connection::new(
            stream,
            StatusConfig {
                motd: String::new(),
                max_players: 0,
            },
        )
        .run()
        .await;
    });
    let mut stream = TcpStream::connect(address).await.unwrap();
    let body = Handshake {
        protocol_version: version.into(),
        server_address: "localhost".into(),
        server_port: 25565,
        intent: intent::LOGIN.into(),
    }
    .pack()
    .unwrap();
    PacketFrame::new(0, body).write(&mut stream).await.unwrap();
    stream
}

async fn start(stream: &mut TcpStream, name: &str) {
    PacketFrame::new(
        0,
        LoginStart {
            name: name.into(),
            player_uuid: uuid::Uuid::nil().into(),
        }
        .pack()
        .unwrap(),
    )
    .write(stream)
    .await
    .unwrap();
}

async fn reply(stream: &mut TcpStream) -> PacketFrame {
    timeout(Duration::from_secs(2), PacketFrame::read(stream))
        .await
        .unwrap()
        .unwrap()
}

async fn closes(stream: &mut TcpStream) {
    assert_eq!(
        timeout(Duration::from_secs(2), stream.read(&mut [0]))
            .await
            .unwrap()
            .unwrap(),
        0
    );
}

#[tokio::test]
async fn completes_offline_login_until_configuration_boundary() {
    let mut stream = connect(777).await;
    start(&mut stream, "Notch").await;
    let frame = reply(&mut stream).await;
    assert_eq!(frame.packet_id.0, 2);
    let success = LoginSuccess::unpack(&mut frame.body.as_slice()).unwrap();
    assert_eq!(success.profile.username.as_ref(), "Notch");
    assert_eq!(
        success.profile.uuid.0.to_string(),
        "b50ad385-829d-3141-a216-7e7d7539ba7f"
    );
    assert!(success.profile.properties.0.is_empty());
    assert_eq!(success.session_id.0.get_version_num(), 4);
    PacketFrame::new(3, LoginAcknowledged.pack().unwrap())
        .write(&mut stream)
        .await
        .unwrap();
    closes(&mut stream).await;
}

#[tokio::test]
async fn rejects_incompatible_login_version_with_a_reason() {
    let mut stream = connect(764).await;
    let frame = reply(&mut stream).await;
    assert_eq!(frame.packet_id.0, 0);
    let disconnect = Disconnect::unpack(&mut frame.body.as_slice()).unwrap();
    assert!(disconnect.reason.as_ref().contains("777"));
    closes(&mut stream).await;
}

#[tokio::test]
async fn rejects_an_invalid_username() {
    let mut stream = connect(777).await;
    start(&mut stream, "bad name").await;
    assert_eq!(reply(&mut stream).await.packet_id.0, 0);
    closes(&mut stream).await;
}

#[tokio::test]
async fn rejects_nonempty_login_acknowledgement() {
    let mut stream = connect(777).await;
    start(&mut stream, "Notch").await;
    assert_eq!(reply(&mut stream).await.packet_id.0, 2);
    PacketFrame::new(3, vec![0])
        .write(&mut stream)
        .await
        .unwrap();
    closes(&mut stream).await;
}
