use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::string::MCString;
use crate::types::ushort::MCUShort;
use crate::types::varint::MCVarInt;
use mclib_macros::MCPacket;

#[derive(Debug, Clone)]
pub enum HandshakeNextState {
    Status = 0x01,
    Login = 0x02,
    Unknown,
}

impl From<i32> for HandshakeNextState {
    fn from(value: i32) -> Self {
        match value {
            0x01 => Self::Status,
            0x02 => Self::Login,
            _ => Self::Unknown,
        }
    }
}

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x00)]
pub struct Handshake {
    pub protocol_version: MCVarInt,
    pub server_address: MCString,
    pub server_port: MCUShort,
    pub next_state: MCVarInt,
}
