use std::io::Read;

use crate::{MCString, MCType, ProtocolError};

/// Ends login with a JSON text component describing the reason.
///
/// Unlike Configuration and Play disconnects, Login uses JSON, not network NBT.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disconnect {
    pub reason: MCString,
}

impl MCType for Disconnect {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        self.reason.pack()
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            reason: MCString::unpack(source)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn matches_the_known_json_wire_sequence() {
        let packet = Disconnect {
            reason: r#"{"text":"Bye"}"#.into(),
        };
        let bytes = b"\x0e{\"text\":\"Bye\"}";

        assert_eq!(packet.pack().unwrap(), bytes);
        assert_eq!(Disconnect::unpack(&mut bytes.as_slice()).unwrap(), packet);
    }

    #[test]
    fn rejects_a_truncated_reason() {
        assert!(matches!(
            Disconnect::unpack(&mut b"\x0e{\"text\":".as_slice()),
            Err(ProtocolError::Io(error)) if error.kind() == ErrorKind::UnexpectedEof
        ));
    }
}
