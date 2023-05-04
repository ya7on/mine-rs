use crate::{MinecraftType, MinecraftVarInt};
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;

pub struct MinecraftBitSet(pub Vec<u8>);

#[async_trait]
impl MinecraftType for MinecraftBitSet {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let bitset_length = MinecraftVarInt::parse_from(io).await?.0;
        let mut bitset = Vec::new();
        for _ in 0..bitset_length {
            bitset.push(io.next_byte().await?);
        }
        Ok(Self(bitset))
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        let mut result = Vec::new();
        result.extend_from_slice(&MinecraftVarInt(self.0.len() as i32).parse_to().await?);
        result.extend_from_slice(&self.0);
        Ok(result)
    }
}
