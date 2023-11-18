use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::string::MCString;
use crate::types::ushort::MCUShort;
use crate::types::varint::MCVarInt;
use mclib_macros::MCPacket;

#[derive(MCPacket)]
#[packet(packet_id = 0x00)]
pub struct Handshake {
    pub protocol_version: MCVarInt,
    pub server_address: MCString,
    pub server_port: MCUShort,
    pub next_state: MCVarInt,
}
