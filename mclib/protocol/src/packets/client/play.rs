use crate::packets::base::MCPacket;
use crate::types::base::MCType;
use crate::types::boolean::MCBoolean;
use crate::types::byte::MCByte;
use crate::types::int::MCInt;
use crate::types::long::MCLong;
use crate::types::position::MCPosition;
use crate::types::string::MCString;
use crate::types::ubyte::MCUByte;
use crate::types::varint::MCVarInt;
use mclib_macros::{MCPacket, MCType};
use std::io::Read;

#[derive(MCType, Debug, Clone)]
pub struct DeathInfo {
    pub death_dimension_name: MCString,
    pub death_location: MCPosition,
}

#[derive(MCPacket, Debug, Clone)]
#[packet(packet_id = 0x29)]
pub struct Play {
    pub entity_id: MCInt,
    pub is_hardcore: MCBoolean,
    pub dimensions: Vec<MCString>,
    pub max_players: MCVarInt,
    pub view_distance: MCVarInt,
    pub simulation_distance: MCVarInt,
    pub reduced_debug_info: MCBoolean,
    pub enable_respawn_screen: MCBoolean,
    pub do_limited_crafting: MCBoolean,
    pub dimension_type: MCString,
    pub dimension_name: MCString,
    pub hashed_seed: MCLong,
    pub game_mode: MCUByte,
    pub previous_game_mode: MCByte,
    pub is_debug: MCBoolean,
    pub is_flat: MCBoolean,
    pub death_info: Option<DeathInfo>,
    pub portal_cooldown: MCVarInt,
}
