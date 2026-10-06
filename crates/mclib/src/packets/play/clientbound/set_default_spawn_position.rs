use std::io::Read;

use crate::{MCFloat, MCPosition, MCString, MCType, ProtocolError};

/// Protocol 777 includes dimension and both rotation angles.
#[derive(Debug, Clone, PartialEq)]
pub struct SetDefaultSpawnPosition {
    pub dimension: MCString,
    pub location: MCPosition,
    pub yaw: MCFloat,
    pub pitch: MCFloat,
}

impl MCType for SetDefaultSpawnPosition {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = Vec::new();
        bytes.extend(self.dimension.pack()?);
        bytes.extend(self.location.pack()?);
        bytes.extend(self.yaw.pack()?);
        bytes.extend(self.pitch.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            dimension: MCString::unpack(source)?,
            location: MCPosition::unpack(source)?,
            yaw: MCFloat::unpack(source)?,
            pitch: MCFloat::unpack(source)?,
        })
    }
}
