use std::io::Read;

use crate::{MCType, ProtocolError};

/// A signed Minecraft variable-length long, at most ten bytes on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCVarLong(pub i64);

impl From<i64> for MCVarLong {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<MCVarLong> for i64 {
    fn from(value: MCVarLong) -> Self {
        value.0
    }
}

impl MCType for MCVarLong {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut value = self.0.cast_unsigned();
        let mut bytes = Vec::with_capacity(10);

        for _ in 0..10 {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            if value == 0 {
                bytes.push(byte);
                break;
            }
            bytes.push(byte | 0x80);
        }
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut result = 0_i64;
        let mut byte = [0_u8];

        for index in 0..10 {
            source.read_exact(&mut byte)?;
            result |= i64::from(byte[0] & 0x7f) << (7 * index);
            if byte[0] & 0x80 == 0 {
                return Ok(Self(result));
            }
        }

        Err(ProtocolError::Overflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn matches_wiki_samples() {
        assert_eq!(MCVarLong(0).pack().unwrap(), [0x00]);
        assert_eq!(MCVarLong(1).pack().unwrap(), [0x01]);
        assert_eq!(
            MCVarLong(2_147_483_647).pack().unwrap(),
            [0xFF, 0xFF, 0xFF, 0xFF, 0x07]
        );
        assert_eq!(
            MCVarLong(i64::MAX).pack().unwrap(),
            [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F]
        );
        assert_eq!(
            MCVarLong(-1).pack().unwrap(),
            [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
        );
        assert_eq!(
            MCVarLong(i64::MIN).pack().unwrap(),
            [0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]
        );
    }

    #[test]
    fn roundtrips_boundary_values() {
        for value in [i64::MIN, -1, 0, 1, 127, 128, i64::MAX] {
            let encoded = MCVarLong(value).pack().unwrap();
            assert_eq!(
                MCVarLong::unpack(&mut Cursor::new(encoded)).unwrap(),
                value.into()
            );
        }
    }

    #[test]
    fn rejects_more_than_ten_bytes() {
        let error = MCVarLong::unpack(&mut Cursor::new([0x80; 10])).unwrap_err();

        assert!(matches!(error, ProtocolError::Overflow));
    }

    #[test]
    fn preserves_signed_bits_and_leaves_trailing_data() {
        let mut source = Cursor::new([
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f, 0x01,
        ]);

        assert_eq!(MCVarLong::unpack(&mut source).unwrap(), MCVarLong(-1));
        assert_eq!(source.position(), 10);
        assert_eq!(MCVarLong::unpack(&mut source).unwrap(), MCVarLong(1));
    }
}
