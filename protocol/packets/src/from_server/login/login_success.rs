use crate::MinecraftPacket;
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;
use types::{MinecraftString, MinecraftType, MinecraftUUID, MinecraftVarInt};

pub struct LoginSuccessPacket {
    pub uuid: MinecraftUUID,
    pub username: MinecraftString,
    pub number_of_properties: MinecraftVarInt,
    pub properties: Vec<()>, // TODO add properties
}

#[async_trait]
impl MinecraftPacket for LoginSuccessPacket {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let packet = Self {
            uuid: MinecraftUUID::parse_from(io).await?,
            username: MinecraftString::parse_from(io).await?,
            number_of_properties: MinecraftVarInt::parse_from(io).await?,
            properties: vec![],
        };

        Ok(packet)
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.uuid.parse_to().await?);
        result.extend_from_slice(&self.username.parse_to().await?);
        result.extend_from_slice(&self.number_of_properties.parse_to().await?);
        Ok(result)
    }
}
