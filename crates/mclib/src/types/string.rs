use std::io::Read;

use super::{MCType, ProtocolError, var_int};

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
        validate::<MAX_CHARACTERS>(&self.0)?;

        let mut bytes = var_int::pack(self.0.len() as i32);
        bytes.extend_from_slice(self.0.as_bytes());
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let length = var_int::unpack(source)?;
        let length = usize::try_from(length).map_err(|_| ProtocolError::NegativeLength(length))?;
        let max_bytes = MAX_CHARACTERS.saturating_mul(3);
        if length > max_bytes {
            return Err(ProtocolError::EncodedStringTooLong {
                bytes: length,
                max_bytes,
            });
        }

        let mut bytes = vec![0_u8; length];
        source.read_exact(&mut bytes)?;
        let value = String::from_utf8(bytes).map_err(|_| ProtocolError::InvalidUtf8)?;
        validate::<MAX_CHARACTERS>(&value)?;
        Ok(Self(value))
    }
}

fn validate<const MAX_CHARACTERS: usize>(value: &str) -> Result<(), ProtocolError> {
    let characters = value.chars().count();
    if characters > MAX_CHARACTERS {
        return Err(ProtocolError::StringTooLong {
            characters,
            max_characters: MAX_CHARACTERS,
        });
    }

    let max_bytes = MAX_CHARACTERS.saturating_mul(3);
    if value.len() > max_bytes {
        return Err(ProtocolError::EncodedStringTooLong {
            bytes: value.len(),
            max_bytes,
        });
    }

    Ok(())
}
