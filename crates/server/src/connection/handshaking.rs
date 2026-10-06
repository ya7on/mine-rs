use std::io::Cursor;

use mclib::packets::handshaking::serverbound::{Handshake, intent};
use mclib::{MCType, ProtocolError};

use crate::ConnectionError;
use crate::connection::Connection;

/// Valid handshake intents; Transfer enters Login with transfer semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Status,
    Login { protocol_version: i32 },
    Transfer,
}

/// Validates the handshake; the coordinator chooses the requested protocol path.
pub async fn run(connection: &mut Connection) -> Result<Outcome, ConnectionError> {
    let frame = connection.read_frame().await?;
    if frame.packet_id.0 != 0 {
        return Err(ConnectionError::UnexpectedPacket {
            packet_id: frame.packet_id.0,
            expected: "handshake",
        });
    }

    let handshake = Handshake::unpack(&mut Cursor::new(frame.body))?;
    match handshake.intent.0 {
        intent::STATUS => Ok(Outcome::Status),
        intent::LOGIN => Ok(Outcome::Login {
            protocol_version: handshake.protocol_version.0,
        }),
        intent::TRANSFER => Ok(Outcome::Transfer),
        _ => Err(ProtocolError::InvalidData.into()),
    }
}
