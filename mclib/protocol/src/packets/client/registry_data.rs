use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::nbt::MCNBT;
use crate::types::varint::MCVarInt;
use mclib_macros::MCPacket;

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x05)]
pub struct RegistryData {
    registry_codec: MCNBT,
}
