use std::io::Read;

use crate::{MCType, ProtocolError};

/// An unsigned, one-byte Minecraft `Unsigned Byte`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCUnsignedByte(pub u8);

impl From<u8> for MCUnsignedByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<MCUnsignedByte> for u8 {
    fn from(value: MCUnsignedByte) -> Self {
        value.0
    }
}

impl MCType for MCUnsignedByte {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(vec![self.0])
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut byte = [0_u8];
        source.read_exact(&mut byte)?;
        Ok(Self(byte[0]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrips_boundary_values() {
        for value in [0_u8, 1, 127, 128, u8::MAX] {
            let encoded = MCUnsignedByte(value).pack().unwrap();
            assert_eq!(
                MCUnsignedByte::unpack(&mut Cursor::new(encoded)).unwrap(),
                value.into()
            );
        }
    }
}
