use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::byte::MCByte;
use crate::types::float::MCFloat;
use crate::types::varint::MCVarInt;
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x36)]
pub struct PlayerAbilities {
    pub flags: MCByte,
    pub flying_speed: MCFloat,
    pub field_of_view_modifier: MCFloat,
}
