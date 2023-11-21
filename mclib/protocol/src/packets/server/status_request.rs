use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::varint::MCVarInt;
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x00)]
pub struct StatusRequest {}
