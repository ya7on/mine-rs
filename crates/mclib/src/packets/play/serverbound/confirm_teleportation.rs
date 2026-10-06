use std::io::Read;

use crate::{MCDouble, MCFloat, MCType, MCVarInt, ProtocolError};

/// Protocol 777 confirms both teleport ID and resulting position/rotation.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfirmTeleportation {
    pub teleport_id: MCVarInt,
    pub x: MCDouble,
    pub y: MCDouble,
    pub z: MCDouble,
    pub yaw: MCFloat,
    pub pitch: MCFloat,
}

impl MCType for ConfirmTeleportation {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = Vec::new();
        bytes.extend(self.teleport_id.pack()?);
        bytes.extend(self.x.pack()?);
        bytes.extend(self.y.pack()?);
        bytes.extend(self.z.pack()?);
        bytes.extend(self.yaw.pack()?);
        bytes.extend(self.pitch.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            teleport_id: MCVarInt::unpack(source)?,
            x: MCDouble::unpack(source)?,
            y: MCDouble::unpack(source)?,
            z: MCDouble::unpack(source)?,
            yaw: MCFloat::unpack(source)?,
            pitch: MCFloat::unpack(source)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_protocol_777_position_echo() {
        // ID 1; doubles 8, 100, 8; floats 0, 0 (big endian).
        let bytes = [
            1, 0x40, 0x20, 0, 0, 0, 0, 0, 0, 0x40, 0x59, 0, 0, 0, 0, 0, 0, 0x40, 0x20, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let packet = ConfirmTeleportation::unpack(&mut bytes.as_slice()).unwrap();
        assert_eq!(packet.teleport_id.0, 1);
        assert_eq!((packet.x.0, packet.y.0, packet.z.0), (8.0, 100.0, 8.0));
        assert_eq!(packet.pack().unwrap(), bytes);
        for end in 0..bytes.len() {
            assert!(ConfirmTeleportation::unpack(&mut &bytes[..end]).is_err());
        }
    }
}
