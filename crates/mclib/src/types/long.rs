use std::io::Read;

use crate::{MCType, ProtocolError};

/// A signed, big-endian 64-bit Minecraft `Long`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCLong(pub i64);

impl From<i64> for MCLong {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<MCLong> for i64 {
    fn from(value: MCLong) -> Self {
        value.0
    }
}

impl MCType for MCLong {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        Ok(self.0.to_be_bytes().to_vec())
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        let mut bytes = [0_u8; 8];
        source.read_exact(&mut bytes)?;
        Ok(Self(i64::from_be_bytes(bytes)))
    }
}
