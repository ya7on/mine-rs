use std::io::Read;

use crate::{MCType, ProtocolError};

/// A Minecraft `UUID`, encoded as two big-endian unsigned 64-bit halves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MCUuid(pub uuid::Uuid);

impl From<uuid::Uuid> for MCUuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
    }
}

impl From<MCUuid> for uuid::Uuid {
    fn from(value: MCUuid) -> Self {
        value.0
    }
}

impl MCType for MCUuid {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(self.0.as_bytes().to_vec())
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; 16];
        source.read_exact(&mut bytes)?;
        Ok(Self(uuid::Uuid::from_bytes(bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrips() {
        let value = uuid::Uuid::from_u128(0x0123_4567_89ab_cdef_0123_4567_89ab_cdef);

        let encoded = MCUuid(value).pack().unwrap();

        assert_eq!(
            encoded,
            [
                0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
                0xcd, 0xef
            ]
        );
        assert_eq!(
            MCUuid::unpack(&mut Cursor::new(encoded)).unwrap(),
            value.into()
        );
    }
}
