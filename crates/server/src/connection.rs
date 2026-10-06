use std::io::{Cursor, Error, ErrorKind};
use std::time::Duration;

use mclib::MCType;
use mclib::PacketFrame;
use mclib::packets::handshaking::serverbound::{Handshake, intent};
use mclib::packets::status::clientbound::{PongResponse, StatusResponse};
use mclib::packets::status::serverbound::{PingRequest, StatusRequest};
use serde_json::json;
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::ConnectionError;
use crate::config::StatusConfig;
const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// The protocol version this server speaks, reported in status responses.
const PROTOCOL_VERSION: i32 = 777;

/// The human-readable name of this server implementation.
const PROTOCOL_NAME: &str = "mine-rs";

/// The phases a connection passes through, mirroring the protocol states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Handshaking,
    AwaitingStatusRequest,
    AwaitingPingRequest,
}

/// Serves one TCP connection: a handshake followed by the status exchange.
#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    state: State,
    status: StatusConfig,
}

impl Connection {
    /// Prepares a connection handler for an already accepted stream.
    #[must_use]
    pub const fn new(stream: TcpStream, status: StatusConfig) -> Self {
        Self {
            stream,
            state: State::Handshaking,
            status,
        }
    }

    /// Drives the connection from handshake to ping and closes it.
    ///
    /// # Errors
    ///
    /// Returns [`ConnectionError`] on any I/O or protocol violation; every
    /// error is fatal for the connection.
    pub async fn run(&mut self) -> Result<(), ConnectionError> {
        loop {
            // Timeout is fatal: never restart a cancelled partial frame read.
            let frame = timeout(READ_TIMEOUT, PacketFrame::read(&mut self.stream))
                .await
                .map_err(|_| Error::new(ErrorKind::TimedOut, "frame read timed out"))??;
            match self.state {
                State::Handshaking => {
                    if frame.packet_id.0 != 0 {
                        return Err(ConnectionError::UnexpectedPacket {
                            packet_id: frame.packet_id.0,
                            expected: "handshake",
                        });
                    }

                    let handshake = Handshake::unpack(&mut Cursor::new(frame.body))?;
                    if handshake.intent.0 != intent::STATUS {
                        return Err(ConnectionError::UnexpectedPacket {
                            packet_id: handshake.intent.0,
                            expected: "status intent (1)",
                        });
                    }

                    self.state = State::AwaitingStatusRequest;
                }
                State::AwaitingStatusRequest => {
                    if frame.packet_id.0 != 0 {
                        return Err(ConnectionError::UnexpectedPacket {
                            packet_id: frame.packet_id.0,
                            expected: "status request",
                        });
                    }

                    StatusRequest::unpack(&mut Cursor::new(frame.body))?;
                    let json_response = json!({
                        "version": {
                            "name": PROTOCOL_NAME,
                            "protocol": PROTOCOL_VERSION,
                        },
                        "players": {
                            "max": self.status.max_players,
                            "online": 0,
                        },
                        "description": {
                            "text": self.status.motd,
                        },
                    })
                    .to_string();
                    self.write_frame(
                        0,
                        StatusResponse {
                            json_response: json_response.as_str().into(),
                        }
                        .pack()?,
                    )
                    .await?;
                    self.state = State::AwaitingPingRequest;
                }
                State::AwaitingPingRequest => {
                    if frame.packet_id.0 != 1 {
                        return Err(ConnectionError::UnexpectedPacket {
                            packet_id: frame.packet_id.0,
                            expected: "ping request",
                        });
                    }

                    let ping = PingRequest::unpack(&mut Cursor::new(frame.body))?;
                    self.write_frame(
                        1,
                        PongResponse {
                            timestamp: ping.timestamp,
                        }
                        .pack()?,
                    )
                    .await?;
                    return Ok(());
                }
            }
        }
    }

    async fn write_frame(&mut self, packet_id: i32, body: Vec<u8>) -> Result<(), ConnectionError> {
        PacketFrame::new(packet_id, body)
            .write(&mut self.stream)
            .await?;
        Ok(())
    }
}
