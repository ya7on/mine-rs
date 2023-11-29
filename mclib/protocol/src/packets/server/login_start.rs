use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::string::MCString;
use crate::types::uuid::MCUuid;
use crate::types::varint::MCVarInt;
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x00)]
pub struct LoginStart {
    pub name: MCString,
    pub player_uuid: MCUuid,
}
