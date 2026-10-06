use std::io::Read;

use crate::{MCString, MCType, MCUuid, ProtocolError};

/// Starts login with the client's username and claimed UUID.
///
/// The server decides the authenticated or offline identity; this UUID alone
/// does not prove the player's identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginStart {
    pub name: MCString<16>,
    pub player_uuid: MCUuid,
}

impl MCType for LoginStart {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut encoded = self.name.pack()?;
        encoded.extend(self.player_uuid.pack()?);
        Ok(encoded)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            name: MCString::unpack(source)?,
            player_uuid: MCUuid::unpack(source)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    const WIRE: [u8; 22] = [
        5, b'N', b'o', b't', b'c', b'h', 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01,
        0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef,
    ];

    #[test]
    fn matches_the_known_wire_sequence() {
        let packet = LoginStart {
            name: "Notch".into(),
            player_uuid: uuid::Uuid::from_u128(0x0123_4567_89ab_cdef_0123_4567_89ab_cdef).into(),
        };

        assert_eq!(packet.pack().unwrap(), WIRE);
        assert_eq!(LoginStart::unpack(&mut WIRE.as_slice()).unwrap(), packet);
    }

    #[test]
    fn rejects_every_truncated_prefix() {
        for length in 0..WIRE.len() {
            assert!(matches!(
                LoginStart::unpack(&mut &WIRE[..length]),
                Err(ProtocolError::Io(error)) if error.kind() == ErrorKind::UnexpectedEof
            ));
        }
    }

    #[test]
    fn enforces_the_username_limit_in_both_directions() {
        let packet = LoginStart {
            name: "a".repeat(16).into(),
            player_uuid: uuid::Uuid::nil().into(),
        };
        let bytes = packet.pack().unwrap();
        assert_eq!(LoginStart::unpack(&mut bytes.as_slice()).unwrap(), packet);

        let packet = LoginStart {
            name: "a".repeat(17).into(),
            ..packet
        };
        assert!(matches!(packet.pack(), Err(ProtocolError::Overflow)));
        let bytes = MCString::<17>::from("a".repeat(17)).pack().unwrap();
        assert!(matches!(
            LoginStart::unpack(&mut bytes.as_slice()),
            Err(ProtocolError::Overflow)
        ));
        // Reject an oversized length before reading or allocating its payload.
        assert!(matches!(
            LoginStart::unpack(&mut [49].as_slice()),
            Err(ProtocolError::Overflow)
        ));
    }
}
