use crate::MinecraftType;
use async_trait::async_trait;
use common::error::{MError, MResult};
use common::io::Buffer;
use uuid::Uuid;

/// UUID
pub struct MinecraftUUID(pub Uuid);

#[async_trait]
impl MinecraftType for MinecraftUUID {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let value = Uuid::from_slice(&[
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
            io.next_byte().await?,
        ])
        .map_err(|err| MError::TypeValidationError("Invalid UUID".to_string()))?;

        Ok(Self(value))
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        Ok(self.0.into_bytes().to_vec())
    }
}
