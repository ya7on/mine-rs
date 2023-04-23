use crate::MinecraftPacket;
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;
use types::{MinecraftString, MinecraftType, MinecraftUnsignedShort, MinecraftVarInt};

pub struct HandshakePacket {
    pub version: MinecraftVarInt,
    pub address: MinecraftString,
    pub port: MinecraftUnsignedShort,
    pub next_state: MinecraftVarInt,
}

#[async_trait]
impl MinecraftPacket for HandshakePacket {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let packet = Self {
            version: MinecraftVarInt::parse_from(io).await?,
            address: MinecraftString::parse_from(io).await?,
            port: MinecraftUnsignedShort::parse_from(io).await?,
            next_state: MinecraftVarInt::parse_from(io).await?,
        };
        Ok(packet)
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.version.parse_to().await?);
        result.extend_from_slice(&self.address.parse_to().await?);
        result.extend_from_slice(&self.port.parse_to().await?);
        result.extend_from_slice(&self.next_state.parse_to().await?);
        Ok(result)
    }
}
