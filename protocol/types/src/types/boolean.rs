use crate::{MinecraftType, MinecraftUnsignedByte};
use async_trait::async_trait;
use common::error::{MError, MResult};
use common::io::Buffer;

pub struct MinecraftBoolean(pub bool);

#[async_trait]
impl MinecraftType for MinecraftBoolean {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let byte = MinecraftUnsignedByte::parse_from(io).await?;
        match byte.0 {
            0 => Ok(Self(false)),
            1 => Ok(Self(true)),
            _ => Err(MError::TypeValidationError(
                "Invalid boolean byte".to_string(),
            )),
        }
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        Ok(match self.0 {
            true => vec![0x01],
            false => vec![0x00],
        })
    }
}
