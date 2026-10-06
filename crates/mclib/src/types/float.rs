use std::io::Read;

use crate::{MCType, ProtocolError};

/// A big-endian single-precision IEEE 754 Minecraft `Float`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MCFloat(pub f32);

impl From<f32> for MCFloat {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<MCFloat> for f32 {
    fn from(value: MCFloat) -> Self {
        value.0
    }
}

impl MCType for MCFloat {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(self.0.to_be_bytes().to_vec())
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; 4];
        source.read_exact(&mut bytes)?;
        Ok(Self(f32::from_be_bytes(bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn is_big_endian() {
        assert_eq!(MCFloat(1.0).pack().unwrap(), [0x3F, 0x80, 0x00, 0x00]);
    }

    #[test]
    fn roundtrips_values() {
        for value in [0.0_f32, -1.5, 90.0, f32::MIN, f32::MAX] {
            let encoded = MCFloat(value).pack().unwrap();
            assert_eq!(
                MCFloat::unpack(&mut Cursor::new(encoded)).unwrap(),
                value.into()
            );
        }
    }
}
