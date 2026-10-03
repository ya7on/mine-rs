use std::io::Read;

use super::{MCType, ProtocolError};

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
        Ok(pack(self.0))
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self(unpack(source)?))
    }
}

pub(super) fn pack(value: i32) -> Vec<u8> {
    let mut value = value as u32;
    let mut bytes = Vec::with_capacity(5);

    loop {
        if value & !0x7f == 0 {
            bytes.push(value as u8);
            return bytes;
        }

        bytes.push(((value & 0x7f) | 0x80) as u8);
        value >>= 7;
    }
}

pub(super) fn unpack(source: &mut dyn Read) -> Result<i32, ProtocolError> {
    let mut result = 0_u32;

    for index in 0..5 {
        let mut byte = [0_u8];
        source.read_exact(&mut byte)?;
        result |= u32::from(byte[0] & 0x7f) << (7 * index);
        if byte[0] & 0x80 == 0 {
            return Ok(result as i32);
        }
    }

    Err(ProtocolError::VarIntTooLong)
}
