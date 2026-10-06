use std::io::Read;

use crate::{MCType, ProtocolError};

/// An unsigned, big-endian two-byte Minecraft `Unsigned Short`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCUnsignedShort(pub u16);

impl From<u16> for MCUnsignedShort {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<MCUnsignedShort> for u16 {
    fn from(value: MCUnsignedShort) -> Self {
        value.0
    }
}

impl MCType for MCUnsignedShort {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(self.0.to_be_bytes().to_vec())
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; 2];
        source.read_exact(&mut bytes)?;
        Ok(Self(u16::from_be_bytes(bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrips_boundary_values() {
        for value in [0_u16, 1, u16::MAX] {
            let encoded = MCUnsignedShort(value).pack().unwrap();
            assert_eq!(
                MCUnsignedShort::unpack(&mut Cursor::new(encoded)).unwrap(),
                value.into()
            );
        }
    }
}
