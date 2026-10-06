use std::io::Read;

use crate::{MCType, ProtocolError};

/// A Minecraft `Boolean`, encoded as a single byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCBoolean(pub bool);

impl From<bool> for MCBoolean {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<MCBoolean> for bool {
    fn from(value: MCBoolean) -> Self {
        value.0
    }
}

impl MCType for MCBoolean {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(vec![u8::from(self.0)])
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut byte = [0_u8];
        source.read_exact(&mut byte)?;
        match byte[0] {
            0 => Ok(Self(false)),
            1 => Ok(Self(true)),
            _ => Err(ProtocolError::InvalidData),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn packs_true_and_false_as_single_bytes() {
        assert_eq!(MCBoolean(true).pack().unwrap(), [0x01]);
        assert_eq!(MCBoolean(false).pack().unwrap(), [0x00]);
    }

    #[test]
    fn unpacks_known_bytes() {
        assert_eq!(
            MCBoolean::unpack(&mut Cursor::new([0x01])).unwrap(),
            true.into()
        );
        assert_eq!(
            MCBoolean::unpack(&mut Cursor::new([0x00])).unwrap(),
            false.into()
        );
    }

    #[test]
    fn rejects_other_values() {
        let error = MCBoolean::unpack(&mut Cursor::new([0x02])).unwrap_err();

        assert!(matches!(error, ProtocolError::InvalidData));
    }

    #[test]
    fn roundtrips() {
        for value in [true, false] {
            let encoded = MCBoolean(value).pack().unwrap();
            assert_eq!(
                MCBoolean::unpack(&mut Cursor::new(encoded)).unwrap(),
                value.into()
            );
        }
    }
}
