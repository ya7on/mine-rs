use std::io::Read;

use crate::{MCFloat, MCType, MCUnsignedByte, ProtocolError};

/// Signals world events; event 13 starts waiting for chunks.
#[derive(Debug, Clone, PartialEq)]
pub struct GameEvent {
    pub event: MCUnsignedByte,
    pub value: MCFloat,
}

impl MCType for GameEvent {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = Vec::new();
        bytes.extend(self.event.pack()?);
        bytes.extend(self.value.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            event: MCUnsignedByte::unpack(source)?,
            value: MCFloat::unpack(source)?,
        })
    }
}
