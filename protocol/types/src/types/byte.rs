use crate::types::{Buffer, MinecraftType};
use async_trait::async_trait;
use common::error::MResult;

#[derive(Debug)]
pub struct MinecraftByte(pub i8);

#[async_trait]
impl MinecraftType for MinecraftByte {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let value = io.next_byte().await? as i8;

        Ok(Self(value))
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        Ok(self.0.to_be_bytes().to_vec())
    }
}
