use std::io::Read;

use crate::{MCDouble, MCFloat, MCInt, MCType, MCVarInt, ProtocolError};

/// Sets position and velocity; flags zero selects absolute values.
#[derive(Debug, Clone, PartialEq)]
pub struct SynchronizePlayerPosition {
    pub teleport_id: MCVarInt,
    pub x: MCDouble,
    pub y: MCDouble,
    pub z: MCDouble,
    pub velocity_x: MCDouble,
    pub velocity_y: MCDouble,
    pub velocity_z: MCDouble,
    pub yaw: MCFloat,
    pub pitch: MCFloat,
    pub flags: MCInt,
}

impl MCType for SynchronizePlayerPosition {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = Vec::new();
        bytes.extend(self.teleport_id.pack()?);
        bytes.extend(self.x.pack()?);
        bytes.extend(self.y.pack()?);
        bytes.extend(self.z.pack()?);
        bytes.extend(self.velocity_x.pack()?);
        bytes.extend(self.velocity_y.pack()?);
        bytes.extend(self.velocity_z.pack()?);
        bytes.extend(self.yaw.pack()?);
        bytes.extend(self.pitch.pack()?);
        bytes.extend(self.flags.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            teleport_id: MCVarInt::unpack(source)?,
            x: MCDouble::unpack(source)?,
            y: MCDouble::unpack(source)?,
            z: MCDouble::unpack(source)?,
            velocity_x: MCDouble::unpack(source)?,
            velocity_y: MCDouble::unpack(source)?,
            velocity_z: MCDouble::unpack(source)?,
            yaw: MCFloat::unpack(source)?,
            pitch: MCFloat::unpack(source)?,
            flags: MCInt::unpack(source)?,
        })
    }
}
