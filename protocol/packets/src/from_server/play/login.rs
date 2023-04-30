use crate::MinecraftPacket;
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;
use types::{
    MinecraftBoolean, MinecraftByte, MinecraftInt, MinecraftLong, MinecraftString, MinecraftType,
    MinecraftUnsignedByte, MinecraftVarInt,
};

pub struct Login {
    pub entity_id: MinecraftInt,
    pub is_hardcore: MinecraftBoolean,
    pub gamemode: MinecraftUnsignedByte,
    pub previous_gamemode: MinecraftByte,
    pub dimension_count: MinecraftVarInt,
    pub dimension_names: Vec<MinecraftString>,
    pub registry_codec: Vec<u8>, // TODO NBT
    pub dimension_type: MinecraftString,
    pub dimension_name: MinecraftString,
    pub hashed_seed: MinecraftLong,
    pub max_players: MinecraftVarInt,
    pub view_distance: MinecraftVarInt,
    pub simulation_distance: MinecraftVarInt,
    pub reduced_debug_info: MinecraftBoolean,
    pub enable_respawn_screen: MinecraftBoolean,
    pub is_debug: MinecraftBoolean,
    pub is_flat: MinecraftBoolean,
    pub has_death_location: MinecraftBoolean,
    // TODO Death dimension name - Identifier
    // TODO Death location - Position
}

#[async_trait]
impl MinecraftPacket for Login {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        todo!()
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.entity_id.parse_to().await?);
        result.extend_from_slice(&self.is_hardcore.parse_to().await?);
        result.extend_from_slice(&self.gamemode.parse_to().await?);
        result.extend_from_slice(&self.previous_gamemode.parse_to().await?);
        result.extend_from_slice(&self.dimension_count.parse_to().await?);
        for dimension_name in self.dimension_names.iter() {
            result.extend_from_slice(&dimension_name.parse_to().await?);
        }
        result.extend_from_slice(&self.registry_codec);
        result.extend_from_slice(&self.dimension_type.parse_to().await?);
        result.extend_from_slice(&self.dimension_name.parse_to().await?);
        result.extend_from_slice(&self.hashed_seed.parse_to().await?);
        result.extend_from_slice(&self.max_players.parse_to().await?);
        result.extend_from_slice(&self.view_distance.parse_to().await?);
        result.extend_from_slice(&self.simulation_distance.parse_to().await?);
        result.extend_from_slice(&self.reduced_debug_info.parse_to().await?);
        result.extend_from_slice(&self.enable_respawn_screen.parse_to().await?);
        result.extend_from_slice(&self.is_debug.parse_to().await?);
        result.extend_from_slice(&self.is_flat.parse_to().await?);
        result.extend_from_slice(&self.has_death_location.parse_to().await?);
        Ok(result)
    }
}
