use crate::MinecraftType;
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;

pub struct MinecraftInt(pub i32);

#[async_trait]
impl MinecraftType for MinecraftInt {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let value = i32::from_be_bytes([
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
        ]);
        Ok(Self(value))
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        Ok(self.0.to_be_bytes().to_vec())
    }
}
