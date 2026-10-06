use std::io::Cursor;

use md5::{Digest, Md5};

use mclib::packets::login::clientbound::{Disconnect, LoginSuccess};
use mclib::packets::login::serverbound::{LoginAcknowledged, LoginStart};
use mclib::{GameProfile, MCType, ProtocolError};
use serde_json::json;

use crate::ConnectionError;
use crate::connection::Connection;

/// Runs offline Login without encryption or compression.
pub async fn run(
    connection: &mut Connection,
    protocol_version: i32,
) -> Result<Option<GameProfile>, ConnectionError> {
    if protocol_version != 777 {
        reject(connection, "Unsupported protocol version; expected 777").await?;
        return Ok(None);
    }

    let frame = connection.read_frame().await?;
    if frame.packet_id.0 != 0 {
        return Err(ConnectionError::UnexpectedPacket {
            packet_id: frame.packet_id.0,
            expected: "login start",
        });
    }
    let mut body = frame.body.as_slice();
    let start = LoginStart::unpack(&mut body)?;
    if !body.is_empty() {
        return Err(ProtocolError::InvalidData.into());
    }
    let name = start.name.as_ref();
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        reject(connection, "Invalid username").await?;
        return Ok(None);
    }

    // Offline identities are server-derived, not the UUID claimed by the client.
    let profile = GameProfile {
        uuid: uuid::Builder::from_md5_bytes(
            Md5::digest(format!("OfflinePlayer:{name}").as_bytes()).into(),
        )
        .into_uuid()
        .into(),
        username: start.name,
        properties: Vec::new().into(),
    };
    connection
        .write_frame(
            2,
            LoginSuccess {
                profile: profile.clone(),
                session_id: uuid::Uuid::new_v4().into(),
            }
            .pack()?,
        )
        .await?;

    let frame = connection.read_frame().await?;
    if frame.packet_id.0 != 3 {
        return Err(ConnectionError::UnexpectedPacket {
            packet_id: frame.packet_id.0,
            expected: "login acknowledged",
        });
    }
    if !frame.body.is_empty() {
        return Err(ProtocolError::InvalidData.into());
    }
    LoginAcknowledged::unpack(&mut Cursor::new(frame.body))?;
    Ok(Some(profile))
}

async fn reject(connection: &mut Connection, reason: &str) -> Result<(), ConnectionError> {
    connection
        .write_frame(
            0,
            Disconnect {
                reason: json!({ "text": reason }).to_string().into(),
            }
            .pack()?,
        )
        .await
}
