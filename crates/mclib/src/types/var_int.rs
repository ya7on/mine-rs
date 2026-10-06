use std::io::Read;

use crate::{MCType, ProtocolError};

/// A signed Minecraft variable-length integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCVarInt(pub i32);

impl From<i32> for MCVarInt {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<MCVarInt> for i32 {
    fn from(value: MCVarInt) -> Self {
        value.0
    }
}

impl MCType for MCVarInt {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut value = self.0.cast_unsigned();
        let mut bytes = Vec::with_capacity(5);

        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            if value == 0 {
                bytes.push(byte);
                return Ok(bytes);
            }
            bytes.push(byte | 0x80);
        }
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut result = 0_i32;
        let mut byte = [0_u8];

        for index in 0..5 {
            source.read_exact(&mut byte)?;
            result |= i32::from(byte[0] & 0x7f) << (7 * index);
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
        assert_eq!(MCVarInt(0).pack().unwrap(), [0x00]);
        assert_eq!(MCVarInt(1).pack().unwrap(), [0x01]);
        assert_eq!(MCVarInt(127).pack().unwrap(), [0x7f]);
        assert_eq!(MCVarInt(128).pack().unwrap(), [0x80, 0x01]);
        assert_eq!(MCVarInt(16_383).pack().unwrap(), [0xff, 0x7f]);
        assert_eq!(MCVarInt(16_384).pack().unwrap(), [0x80, 0x80, 0x01]);
        assert_eq!(
            MCVarInt(2_147_483_647).pack().unwrap(),
            [0xFF, 0xFF, 0xFF, 0xFF, 0x07]
        );
        assert_eq!(MCVarInt(-1).pack().unwrap(), [0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
        assert_eq!(
            MCVarInt(i32::MIN).pack().unwrap(),
            [0x80, 0x80, 0x80, 0x80, 0x08]
        );
    }

    #[test]
    fn roundtrips_boundary_values() {
        for value in [i32::MIN, -1, 0, 1, 127, 128, i32::MAX] {
            let encoded = MCVarInt(value).pack().unwrap();
            assert_eq!(
                MCVarInt::unpack(&mut Cursor::new(encoded)).unwrap(),
                value.into()
            );
        }
    }

    #[test]
    fn rejects_more_than_five_bytes() {
        let error = MCVarInt::unpack(&mut Cursor::new([0x80; 5])).unwrap_err();

        assert!(matches!(error, ProtocolError::Overflow));
    }

    #[test]
    fn preserves_signed_bits_and_leaves_trailing_data() {
        let mut source = Cursor::new([0xff, 0xff, 0xff, 0xff, 0x7f, 0x01]);

        assert_eq!(MCVarInt::unpack(&mut source).unwrap(), MCVarInt(-1));
        assert_eq!(source.position(), 5);
        assert_eq!(MCVarInt::unpack(&mut source).unwrap(), MCVarInt(1));
    }
}
