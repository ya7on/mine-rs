use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::string::MCString;
use crate::types::uuid::MCUuid;
use crate::types::varint::MCVarInt;
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug)]
#[packet(packet_id = 0x04)]
pub struct LoginStart {
    name: MCString,
    player_uuid: MCUuid,
}
