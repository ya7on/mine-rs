use std::io::Read;

use crate::{MCString, MCType, MCVarInt, ProtocolError};

/// Opens a connection and switches the server into the target state.
///
/// The intent value selects the next protocol state: 1 for Status, 2 for
/// Login, and 3 for Transfer (which also enters the Login state).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handshake {
    pub protocol_version: MCVarInt,
    pub server_address: MCString<255>,
    pub server_port: u16,
    pub intent: MCVarInt,
}

/// Intent values accepted by the Handshake packet's intent field.
pub mod intent {
    /// Requests the server-list status protocol.
    pub const STATUS: i32 = 1;
    /// Starts the login process.
    pub const LOGIN: i32 = 2;
    /// Starts the login process after a Transfer.
    pub const TRANSFER: i32 = 3;
}

impl MCType for Handshake {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut encoded = self.protocol_version.pack()?;
        encoded.extend(self.server_address.pack()?);
        encoded.extend(self.server_port.to_be_bytes());
        encoded.extend(self.intent.pack()?);
        Ok(encoded)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            protocol_version: MCVarInt::unpack(source)?,
            server_address: MCString::unpack(source)?,
            server_port: {
                let mut bytes = [0_u8; 2];
                source.read_exact(&mut bytes)?;
                u16::from_be_bytes(bytes)
            },
            intent: MCVarInt::unpack(source)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn sample() -> Handshake {
        Handshake {
            protocol_version: 777.into(),
            server_address: "127.0.0.1".into(),
            server_port: 25_565,
            intent: intent::STATUS.into(),
        }
    }

    #[test]
    fn packs_fields_in_protocol_order() {
        let encoded = sample().pack().unwrap();

        // protocol version, address (length + bytes), port (big-endian), intent
        assert_eq!(
            encoded,
            [
                0x89, 0x06, // protocol version 777
                0x09, b'1', b'2', b'7', b'.', b'0', b'.', b'0', b'.', b'1', 0x63,
                0xdd, // port 25565
                0x01, // intent Status
            ]
        );
    }

    #[test]
    fn round_trips_through_the_encoded_form() {
        let encoded = sample().pack().unwrap();

        assert_eq!(
            Handshake::unpack(&mut Cursor::new(encoded)).unwrap(),
            sample()
        );
    }

    #[test]
    fn unpacks_a_known_wire_sequence() {
        let encoded = [
            0x89, 0x06, 0x09, b'l', b'o', b'c', b'a', b'l', b'h', b'o', b's', b't', 0x63, 0xdd,
            0x01,
        ];

        let handshake = Handshake::unpack(&mut Cursor::new(encoded)).unwrap();

        assert_eq!(handshake.protocol_version, 777.into());
        assert_eq!(handshake.server_address.as_ref(), "localhost");
        assert_eq!(handshake.server_port, 25_565);
        assert_eq!(handshake.intent, intent::STATUS.into());
    }

    #[test]
    fn rejects_addresses_over_255_characters() {
        let long_address = "a".repeat(256);
        let handshake = Handshake {
            protocol_version: 777.into(),
            server_address: long_address.into(),
            server_port: 25_565,
            intent: intent::STATUS.into(),
        };

        assert!(handshake.pack().is_err());
    }
}
