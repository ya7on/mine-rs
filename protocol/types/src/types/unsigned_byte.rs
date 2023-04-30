use crate::MinecraftType;
use async_trait::async_trait;
use common::error::MResult;
use common::io::Buffer;

/// 8-битное целое число без знака
#[derive(Debug)]
pub struct MinecraftUnsignedByte(pub u8);

#[async_trait]
impl MinecraftType for MinecraftUnsignedByte {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let value = io.next_byte().await?;

        Ok(Self(value))
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        Ok(self.0.to_be_bytes().to_vec())
    }
}
