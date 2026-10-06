use std::io::Read;

use crate::{GameProfile, MCType, MCUuid, ProtocolError};

/// Completes Login; protocol 777 includes a separate connection session UUID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginSuccess {
    pub profile: GameProfile,
    pub session_id: MCUuid,
}

impl MCType for LoginSuccess {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = self.profile.pack()?;
        bytes.extend(self.session_id.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            profile: GameProfile::unpack(source)?,
            session_id: MCUuid::unpack(source)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn matches_protocol_777_fields_including_session_id() {
        let packet = LoginSuccess {
            profile: GameProfile {
                uuid: uuid::Uuid::nil().into(),
                username: "a".into(),
                properties: Vec::new().into(),
            },
            session_id: uuid::Uuid::from_bytes([1; 16]).into(),
        };
        let mut expected = vec![0; 16];
        expected.extend([1, b'a', 0]);
        expected.extend([1; 16]);
        assert_eq!(packet.pack().unwrap(), expected);
        assert_eq!(
            LoginSuccess::unpack(&mut expected.as_slice()).unwrap(),
            packet
        );
        for length in 0..expected.len() {
            assert!(matches!(
                LoginSuccess::unpack(&mut &expected[..length]),
                Err(ProtocolError::Io(error)) if error.kind() == ErrorKind::UnexpectedEof
            ));
        }
    }
}
