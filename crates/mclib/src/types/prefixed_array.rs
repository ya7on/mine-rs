use std::io::Read;

use crate::{MCType, MCVarInt, ProtocolError};

/// A VarInt-prefixed array of Minecraft types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MCPrefixedArray<T, const MAX_LENGTH: usize = { i32::MAX as usize }>(pub Vec<T>);

impl<T, const MAX_LENGTH: usize> From<Vec<T>> for MCPrefixedArray<T, MAX_LENGTH> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T: MCType, const MAX_LENGTH: usize> MCType for MCPrefixedArray<T, MAX_LENGTH> {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let length = self.0.len();
        if length > MAX_LENGTH {
            return Err(ProtocolError::Overflow);
        }

        let length = i32::try_from(length).map_err(|_| ProtocolError::Overflow)?;
        let mut bytes = MCVarInt(length).pack()?;
        for item in &self.0 {
            bytes.extend(item.pack()?);
        }
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let length = MCVarInt::unpack(source)?.0;
        let length = usize::try_from(length).map_err(|_| ProtocolError::InvalidData)?;
        if length > MAX_LENGTH {
            return Err(ProtocolError::Overflow);
        }

        let mut items = Vec::with_capacity(length.min(1024));
        for _ in 0..length {
            items.push(T::unpack(source)?);
        }
        Ok(Self(items))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrips() {
        let value: MCPrefixedArray<MCVarInt> = vec![1.into(), 2.into(), 3.into()].into();

        let encoded = value.pack().unwrap();

        assert_eq!(encoded, [0x03, 0x01, 0x02, 0x03]);
        assert_eq!(
            MCPrefixedArray::<MCVarInt>::unpack(&mut Cursor::new(encoded)).unwrap(),
            value
        );
    }

    #[test]
    fn roundtrips_empty_array() {
        let value: MCPrefixedArray<MCVarInt> = Vec::new().into();

        let encoded = value.pack().unwrap();

        assert_eq!(encoded, [0x00]);
        assert_eq!(
            MCPrefixedArray::<MCVarInt>::unpack(&mut Cursor::new(encoded)).unwrap(),
            value
        );
    }

    #[test]
    fn rejects_declared_lengths_over_the_limit() {
        let encoded = MCVarInt(100).pack().unwrap();

        let error = MCPrefixedArray::<MCVarInt, 64>::unpack(&mut Cursor::new(encoded)).unwrap_err();

        assert!(matches!(error, ProtocolError::Overflow));
    }
}
