use std::io::Read;

use crate::{MCType, MCVarInt, ProtocolError};

/// Sets the center of the client chunk cache.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCenterChunk {
    pub x: MCVarInt,
    pub z: MCVarInt,
}

impl MCType for SetCenterChunk {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = Vec::new();
        bytes.extend(self.x.pack()?);
        bytes.extend(self.z.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            x: MCVarInt::unpack(source)?,
            z: MCVarInt::unpack(source)?,
        })
    }
}
