use std::io::Cursor;

use mclib::MCType;
use mclib::packets::status::clientbound::{PongResponse, StatusResponse};
use mclib::packets::status::serverbound::{PingRequest, StatusRequest};
use serde_json::json;

use crate::ConnectionError;
use crate::connection::Connection;

/// The protocol version this server speaks, reported in status responses.
const PROTOCOL_VERSION: i32 = 777;

/// The human-readable name of this server implementation.
const PROTOCOL_NAME: &str = "mine-rs";

/// Successful Status exchanges; neither continues into another protocol phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    StatusAndPing,
    PingOnly,
}

/// Optionally answers a status request, then echoes the ping.
pub async fn run(connection: &mut Connection) -> Result<Outcome, ConnectionError> {
    let frame = connection.read_frame().await?;
    if frame.packet_id.0 == 1 {
        pong(connection, frame).await?;
        return Ok(Outcome::PingOnly);
    }
    if frame.packet_id.0 != 0 {
        return Err(ConnectionError::UnexpectedPacket {
            packet_id: frame.packet_id.0,
            expected: "status request or ping request",
        });
    }

    StatusRequest::unpack(&mut Cursor::new(frame.body))?;
    let json_response = json!({
        "version": {
            "name": PROTOCOL_NAME,
            "protocol": PROTOCOL_VERSION,
        },
        "players": {
            "max": connection.status.max_players,
            "online": 0,
        },
        "description": {
            "text": connection.status.motd,
        },
    })
    .to_string();
    connection
        .write_frame(
            0,
            StatusResponse {
                json_response: json_response.as_str().into(),
            }
            .pack()?,
        )
        .await?;

    let frame = connection.read_frame().await?;
    if frame.packet_id.0 != 1 {
        return Err(ConnectionError::UnexpectedPacket {
            packet_id: frame.packet_id.0,
            expected: "ping request",
        });
    }

    pong(connection, frame).await?;
    Ok(Outcome::StatusAndPing)
}

async fn pong(
    connection: &mut Connection,
    frame: mclib::PacketFrame,
) -> Result<(), ConnectionError> {
    let ping = PingRequest::unpack(&mut Cursor::new(frame.body))?;
    connection
        .write_frame(
            1,
            PongResponse {
                timestamp: ping.timestamp,
            }
            .pack()?,
        )
        .await?;
    Ok(())
}
