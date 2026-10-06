use std::io::Read;

use crate::{MCType, MCVarInt, ProtocolError};

/// Marks the number of chunks sent in a batch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkBatchFinished {
    pub size: MCVarInt,
}

impl MCType for ChunkBatchFinished {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = Vec::new();
        bytes.extend(self.size.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            size: MCVarInt::unpack(source)?,
        })
    }
}
