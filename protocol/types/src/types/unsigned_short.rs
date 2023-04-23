use crate::{MinecraftType, MinecraftUnsignedByte};
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;

pub struct MinecraftUnsignedShort(u16);

#[async_trait]
impl MinecraftType for MinecraftUnsignedShort {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let value = u16::from_be_bytes([
            MinecraftUnsignedByte::parse_from(io).await?.0,
            MinecraftUnsignedByte::parse_from(io).await?.0,
        ]);

        Ok(Self(value))
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        Ok(self.0.to_be_bytes().to_vec())
    }
}
