use crate::MinecraftPacket;
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;
use types::{MinecraftBoolean, MinecraftString, MinecraftType, MinecraftUUID};

pub struct LoginStartPacket {
    pub name: MinecraftString,
    pub has_uuid: MinecraftBoolean,
    pub uuid: Option<MinecraftUUID>,
}

#[async_trait]
impl MinecraftPacket for LoginStartPacket {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let name = MinecraftString::parse_from(io).await?;
        let has_uuid = MinecraftBoolean::parse_from(io).await?;
        let uuid = if has_uuid.0 {
            Some(MinecraftUUID::parse_from(io).await?)
        } else {
            None
        };

        Ok(Self {
            name,
            has_uuid,
            uuid,
        })
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.name.parse_to().await?);
        result.extend_from_slice(&self.has_uuid.parse_to().await?);
        if let Some(uuid) = &self.uuid {
            result.extend_from_slice(&uuid.parse_to().await?);
        }
        Ok(result)
    }
}
