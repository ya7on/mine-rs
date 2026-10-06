use std::io::Read;

use crate::{MCType, ProtocolError};

/// A signed, one-byte Minecraft `Byte`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCByte(pub i8);

impl From<i8> for MCByte {
    fn from(value: i8) -> Self {
        Self(value)
    }
}

impl From<MCByte> for i8 {
    fn from(value: MCByte) -> Self {
        value.0
    }
}

impl MCType for MCByte {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(self.0.to_le_bytes().to_vec())
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut byte = [0_u8];
        source.read_exact(&mut byte)?;
        Ok(Self(i8::from_le_bytes(byte)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn encodes_two_complement() {
        assert_eq!(MCByte(-1).pack().unwrap(), [0xFF]);
        assert_eq!(MCByte(127).pack().unwrap(), [0x7F]);
    }

    #[test]
    fn roundtrips_boundary_values() {
        for value in [i8::MIN, -1, 0, 1, i8::MAX] {
            let encoded = MCByte(value).pack().unwrap();
            assert_eq!(
                MCByte::unpack(&mut Cursor::new(encoded)).unwrap(),
                value.into()
            );
        }
    }
}
