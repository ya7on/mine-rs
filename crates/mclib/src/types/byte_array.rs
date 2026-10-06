use std::io::Read;

use crate::{MCType, MCVarInt, ProtocolError};

/// A VarInt-prefixed Minecraft `Byte Array`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MCByteArray<const MAX_LENGTH: usize = { i32::MAX as usize }>(pub Vec<u8>);

impl<const MAX_LENGTH: usize> From<Vec<u8>> for MCByteArray<MAX_LENGTH> {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl<const MAX_LENGTH: usize> MCType for MCByteArray<MAX_LENGTH> {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let length = self.0.len();
        if length > MAX_LENGTH {
            return Err(ProtocolError::Overflow);
        }

        let length = i32::try_from(length).map_err(|_| ProtocolError::Overflow)?;
        let mut bytes = MCVarInt(length).pack()?;
        bytes.extend_from_slice(&self.0);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let length = MCVarInt::unpack(source)?.0;
        let length = usize::try_from(length).map_err(|_| ProtocolError::InvalidData)?;
        if length > MAX_LENGTH {
            return Err(ProtocolError::Overflow);
        }

        let mut bytes = vec![0_u8; length];
        source.read_exact(&mut bytes)?;
        Ok(Self(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrips() {
        let value = MCByteArray::<1024>(vec![1, 2, 3]);

        let encoded = value.pack().unwrap();

        assert_eq!(encoded, [0x03, 1, 2, 3]);
        assert_eq!(
            MCByteArray::<1024>::unpack(&mut Cursor::new(encoded)).unwrap(),
            value
        );
    }

    #[test]
    fn roundtrips_empty_array() {
        let value = MCByteArray::<1024>(Vec::new());

        let encoded = value.pack().unwrap();

        assert_eq!(encoded, [0x00]);
        assert_eq!(
            MCByteArray::<1024>::unpack(&mut Cursor::new(encoded)).unwrap(),
            value
        );
    }

    #[test]
    fn rejects_arrays_over_the_limit() {
        let value = MCByteArray::<2>(vec![1, 2, 3]);

        assert!(matches!(value.pack(), Err(ProtocolError::Overflow)));
    }

    #[test]
    fn rejects_declared_lengths_over_the_limit() {
        let encoded = MCVarInt(100).pack().unwrap();

        let error = MCByteArray::<64>::unpack(&mut Cursor::new(encoded)).unwrap_err();

        assert!(matches!(error, ProtocolError::Overflow));
    }
}
