use std::io::Read;

use crate::{
    MCBoolean, MCInt, MCLong, MCPosition, MCPrefixedArray, MCString, MCType, MCVarInt,
    ProtocolError,
};

/// Initializes a world after Configuration, using negotiated dimension IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Login {
    pub entity_id: MCInt,
    pub is_hardcore: MCBoolean,
    pub dimensions: MCPrefixedArray<MCString, 16>,
    pub max_players: MCVarInt,
    pub view_distance: MCVarInt,
    pub simulation_distance: MCVarInt,
    pub reduced_debug_info: MCBoolean,
    pub enable_respawn_screen: MCBoolean,
    pub limited_crafting: MCBoolean,
    pub dimension_type: MCVarInt,
    pub dimension_name: MCString,
    pub hashed_seed: MCLong,
    pub game_mode: MCVarInt,
    pub previous_game_mode: MCVarInt,
    pub is_debug: MCBoolean,
    pub is_flat: MCBoolean,
    pub death_location: Option<(MCString, MCPosition)>,
    pub portal_cooldown: MCVarInt,
    pub sea_level: MCVarInt,
    pub online_mode: MCBoolean,
    pub enforces_secure_chat: MCBoolean,
}

impl MCType for Login {
    fn pack(&self) -> Result<Vec<u8>, ProtocolError> {
        let mut bytes = Vec::new();
        bytes.extend(self.entity_id.pack()?);
        bytes.extend(self.is_hardcore.pack()?);
        bytes.extend(self.dimensions.pack()?);
        bytes.extend(self.max_players.pack()?);
        bytes.extend(self.view_distance.pack()?);
        bytes.extend(self.simulation_distance.pack()?);
        bytes.extend(self.reduced_debug_info.pack()?);
        bytes.extend(self.enable_respawn_screen.pack()?);
        bytes.extend(self.limited_crafting.pack()?);
        bytes.extend(self.dimension_type.pack()?);
        bytes.extend(self.dimension_name.pack()?);
        bytes.extend(self.hashed_seed.pack()?);
        bytes.extend(self.game_mode.pack()?);
        bytes.extend(self.previous_game_mode.pack()?);
        bytes.extend(self.is_debug.pack()?);
        bytes.extend(self.is_flat.pack()?);
        bytes.extend(MCBoolean(self.death_location.is_some()).pack()?);
        if let Some((dimension, position)) = &self.death_location {
            bytes.extend(dimension.pack()?);
            bytes.extend(position.pack()?);
        }
        bytes.extend(self.portal_cooldown.pack()?);
        bytes.extend(self.sea_level.pack()?);
        bytes.extend(self.online_mode.pack()?);
        bytes.extend(self.enforces_secure_chat.pack()?);
        Ok(bytes)
    }

    fn unpack(source: &mut dyn Read) -> Result<Self, ProtocolError> {
        Ok(Self {
            entity_id: MCInt::unpack(source)?,
            is_hardcore: MCBoolean::unpack(source)?,
            dimensions: MCPrefixedArray::<MCString, 16>::unpack(source)?,
            max_players: MCVarInt::unpack(source)?,
            view_distance: MCVarInt::unpack(source)?,
            simulation_distance: MCVarInt::unpack(source)?,
            reduced_debug_info: MCBoolean::unpack(source)?,
            enable_respawn_screen: MCBoolean::unpack(source)?,
            limited_crafting: MCBoolean::unpack(source)?,
            dimension_type: MCVarInt::unpack(source)?,
            dimension_name: MCString::unpack(source)?,
            hashed_seed: MCLong::unpack(source)?,
            game_mode: MCVarInt::unpack(source)?,
            previous_game_mode: MCVarInt::unpack(source)?,
            is_debug: MCBoolean::unpack(source)?,
            is_flat: MCBoolean::unpack(source)?,
            death_location: if MCBoolean::unpack(source)?.0 {
                Some((MCString::unpack(source)?, MCPosition::unpack(source)?))
            } else {
                None
            },
            portal_cooldown: MCVarInt::unpack(source)?,
            sea_level: MCVarInt::unpack(source)?,
            online_mode: MCBoolean::unpack(source)?,
            enforces_secure_chat: MCBoolean::unpack(source)?,
        })
    }
}
