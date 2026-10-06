use std::io::Read;

use crate::{MCType, ProtocolError};

/// A signed, big-endian two-byte Minecraft `Short`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCShort(pub i16);

impl From<i16> for MCShort {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

impl From<MCShort> for i16 {
    fn from(value: MCShort) -> Self {
        value.0
    }
}

impl MCType for MCShort {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(self.0.to_be_bytes().to_vec())
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; 2];
        source.read_exact(&mut bytes)?;
        Ok(Self(i16::from_be_bytes(bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn is_big_endian() {
        assert_eq!(MCShort(0x0102).pack().unwrap(), [0x01, 0x02]);
    }

    #[test]
    fn roundtrips_boundary_values() {
        for value in [i16::MIN, -1, 0, 1, i16::MAX] {
            let encoded = MCShort(value).pack().unwrap();
            assert_eq!(
                MCShort::unpack(&mut Cursor::new(encoded)).unwrap(),
                value.into()
            );
        }
    }
}
