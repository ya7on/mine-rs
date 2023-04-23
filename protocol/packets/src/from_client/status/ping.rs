use crate::MinecraftPacket;
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;
use types::{MinecraftLong, MinecraftType};

pub struct PingRequest {
    pub payload: MinecraftLong,
}

#[async_trait]
impl MinecraftPacket for PingRequest {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let packet = Self {
            payload: MinecraftLong::parse_from(io).await?,
        };

        Ok(packet)
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.payload.parse_to().await?);
        Ok(result)
    }
}
