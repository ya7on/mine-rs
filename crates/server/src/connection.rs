use std::io::{BufReader, Cursor, Write};
use std::net::TcpStream;
use std::time::Duration;

use mclib::PacketFrame;
use mclib::packets::handshaking::serverbound::{Handshake, intent};
use mclib::packets::status::clientbound::{PongResponse, StatusResponse};
use mclib::packets::status::serverbound::{PingRequest, StatusRequest};
use mclib::{MCType, ProtocolError};
use serde_json::json;

use crate::config::StatusConfig;

/// How long to wait for the next packet before dropping the connection.
const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// The protocol version this server speaks, reported in status responses.
const PROTOCOL_VERSION: i32 = 777;

/// The human-readable name of this server implementation.
const PROTOCOL_NAME: &str = "mine-rs";

/// Errors that end a status connection.
///
/// Every variant is fatal for the connection; the protocol has no disconnect
/// packet in the status state, so recovery means closing the socket.
#[derive(Debug)]
pub enum ConnectionError {
    Io(std::io::Error),
    Protocol(ProtocolError),
    UnexpectedPacket {
        packet_id: i32,
        expected: &'static str,
    },
    HandshakeRequired,
}

impl From<std::io::Error> for ConnectionError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<ProtocolError> for ConnectionError {
    fn from(error: ProtocolError) -> Self {
        Self::Protocol(error)
    }
}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::Protocol(error) => write!(formatter, "protocol error: {error}"),
            Self::UnexpectedPacket {
                packet_id,
                expected,
            } => {
                write!(
                    formatter,
                    "unexpected packet {packet_id}, expected {expected}"
                )
            }
            Self::HandshakeRequired => {
                formatter.write_str("connection closed before a handshake arrived")
            }
        }
    }
}

impl std::error::Error for ConnectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Protocol(error) => Some(error),
            _ => None,
        }
    }
}

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
    reader: BufReader<TcpStream>,
    stream: TcpStream,
    state: State,
    status: StatusConfig,
}

impl Connection {
    /// Prepares a connection handler for an already accepted stream.
    ///
    /// # Errors
    ///
    /// Returns [`ConnectionError`] when socket options cannot be applied or
    /// the stream cannot be cloned for writing.
    pub fn new(stream: TcpStream, status: StatusConfig) -> Result<Self, ConnectionError> {
        stream.set_read_timeout(Some(READ_TIMEOUT))?;
        let stream_for_writing = stream.try_clone()?;
        Ok(Self {
            reader: BufReader::new(stream),
            stream: stream_for_writing,
            state: State::Handshaking,
            status,
        })
    }

    /// Drives the connection from handshake to ping and closes it.
    ///
    /// # Errors
    ///
    /// Returns [`ConnectionError`] on any I/O or protocol violation; every
    /// error is fatal for the connection.
    pub fn run(&mut self) -> Result<(), ConnectionError> {
        loop {
            match self.state {
                State::Handshaking => {
                    let frame = PacketFrame::read(&mut self.reader)?;
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
                    let frame = PacketFrame::read(&mut self.reader)?;
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
                    )?;
                    self.state = State::AwaitingPingRequest;
                }
                State::AwaitingPingRequest => {
                    let frame = PacketFrame::read(&mut self.reader)?;
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
                    )?;
                    return Ok(());
                }
            }
        }
    }

    fn write_frame(&mut self, packet_id: i32, body: Vec<u8>) -> Result<(), ConnectionError> {
        PacketFrame::new(packet_id, body).write(&mut self.stream)?;
        self.stream.flush()?;
        Ok(())
    }
}
