use std::io::Read;

use crate::{MCType, MCVarInt, ProtocolError};

/// A VarInt-prefixed UTF-8 Minecraft `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MCString<const MAX_CHARACTERS: usize = 32_767>(pub String);

impl<const MAX_CHARACTERS: usize> From<String> for MCString<MAX_CHARACTERS> {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl<const MAX_CHARACTERS: usize> From<&str> for MCString<MAX_CHARACTERS> {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl<const MAX_CHARACTERS: usize> From<MCString<MAX_CHARACTERS>> for String {
    fn from(value: MCString<MAX_CHARACTERS>) -> Self {
        value.0
    }
}

impl<const MAX_CHARACTERS: usize> AsRef<str> for MCString<MAX_CHARACTERS> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<const MAX_CHARACTERS: usize> MCType for MCString<MAX_CHARACTERS> {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let characters = self.0.encode_utf16().count();
        if characters > MAX_CHARACTERS {
            return Err(ProtocolError::Overflow);
        }

        let length = self.0.len();
        let max_bytes = MAX_CHARACTERS.saturating_mul(3);
        if length > max_bytes {
            return Err(ProtocolError::Overflow);
        }
        let length = i32::try_from(length).map_err(|_| ProtocolError::Overflow)?;
        let mut bytes = MCVarInt(length).pack()?;
        bytes.extend_from_slice(self.0.as_bytes());
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let length = MCVarInt::unpack(source)?.0;
        let length = usize::try_from(length).map_err(|_| ProtocolError::InvalidData)?;
        let max_bytes = MAX_CHARACTERS.saturating_mul(3);
        if length > max_bytes {
            return Err(ProtocolError::Overflow);
        }

        let mut bytes = vec![0_u8; length];
        source.read_exact(&mut bytes)?;
        let value = String::from_utf8(bytes).map_err(|_| ProtocolError::InvalidData)?;
        let characters = value.encode_utf16().count();
        if characters > MAX_CHARACTERS {
            return Err(ProtocolError::Overflow);
        }
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, ErrorKind};

    #[test]
    fn encodes_utf8_byte_length() {
        let value = MCString::<2>::from("éa");
        let encoded = value.pack().unwrap();

        assert_eq!(encoded, [0x03, 0xc3, 0xa9, 0x61]);
        assert_eq!(
            MCString::<2>::unpack(&mut Cursor::new(encoded)).unwrap(),
            value
        );
        assert_eq!(MCString::<0>::from("").pack().unwrap(), [0x00]);
    }

    #[test]
    fn rejects_character_and_byte_limits() {
        assert!(matches!(
            MCString::<1>::from("ab").pack(),
            Err(ProtocolError::Overflow)
        ));
        assert!(matches!(
            MCString::<1>::unpack(&mut Cursor::new([2, b'a', b'b'])),
            Err(ProtocolError::Overflow)
        ));
        assert!(matches!(
            MCString::<1>::from("😀").pack(),
            Err(ProtocolError::Overflow)
        ));
        // Reject the declared size before trying to read the missing payload.
        assert!(matches!(
            MCString::<1>::unpack(&mut Cursor::new([4])),
            Err(ProtocolError::Overflow)
        ));
    }

    #[test]
    fn counts_supplementary_characters_as_two_utf16_units() {
        let value = MCString::<2>::from("😀");
        let bytes = value.pack().unwrap();
        assert_eq!(bytes, [4, 0xf0, 0x9f, 0x98, 0x80]);
        assert_eq!(MCString::<2>::unpack(&mut bytes.as_slice()).unwrap(), value);

        let value = MCString::<2>::from("😀a");
        assert!(matches!(value.pack(), Err(ProtocolError::Overflow)));
        assert!(matches!(
            MCString::<2>::unpack(&mut [5, 0xf0, 0x9f, 0x98, 0x80, b'a'].as_slice()),
            Err(ProtocolError::Overflow)
        ));
    }

    #[test]
    fn rejects_negative_lengths_invalid_utf8_and_truncated_data() {
        assert!(matches!(
            MCString::<1>::unpack(&mut Cursor::new([0xff, 0xff, 0xff, 0xff, 0x0f])),
            Err(ProtocolError::InvalidData)
        ));
        assert!(matches!(
            MCString::<1>::unpack(&mut Cursor::new([1, 0xff])),
            Err(ProtocolError::InvalidData)
        ));
        assert!(matches!(
            MCString::<1>::unpack(&mut Cursor::new([1])),
            Err(ProtocolError::Io(error)) if error.kind() == ErrorKind::UnexpectedEof
        ));
    }
}
