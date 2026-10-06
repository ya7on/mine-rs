use std::io::Read;

use crate::{MCType, ProtocolError};

/// A signed, big-endian four-byte Minecraft `Int`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCInt(pub i32);

impl From<i32> for MCInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<MCInt> for i32 {
    fn from(value: MCInt) -> Self {
        value.0
    }
}

impl MCType for MCInt {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(self.0.to_be_bytes().to_vec())
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; 4];
        source.read_exact(&mut bytes)?;
        Ok(Self(i32::from_be_bytes(bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn is_big_endian() {
        assert_eq!(MCInt(0x0102_0304).pack().unwrap(), [0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn roundtrips_boundary_values() {
        for value in [i32::MIN, -1, 0, 1, i32::MAX] {
            let encoded = MCInt(value).pack().unwrap();
            assert_eq!(
                MCInt::unpack(&mut Cursor::new(encoded)).unwrap(),
                value.into()
            );
        }
    }
}
