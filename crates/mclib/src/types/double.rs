use std::io::Read;

use crate::{MCType, ProtocolError};

/// A big-endian double-precision IEEE 754 Minecraft `Double`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MCDouble(pub f64);

impl From<f64> for MCDouble {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<MCDouble> for f64 {
    fn from(value: MCDouble) -> Self {
        value.0
    }
}

impl MCType for MCDouble {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(self.0.to_be_bytes().to_vec())
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; 8];
        source.read_exact(&mut bytes)?;
        Ok(Self(f64::from_be_bytes(bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn is_big_endian() {
        assert_eq!(
            MCDouble(1.0).pack().unwrap(),
            [0x3F, 0xF0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn roundtrips_values() {
        for value in [0.0_f64, -1.5, 100.0, f64::MIN, f64::MAX] {
            let encoded = MCDouble(value).pack().unwrap();
            assert_eq!(
                MCDouble::unpack(&mut Cursor::new(encoded)).unwrap(),
                value.into()
            );
        }
    }
}
