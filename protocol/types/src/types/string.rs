use crate::{MinecraftType, MinecraftUnsignedByte, MinecraftVarInt};
use async_trait::async_trait;
use common::error::{MError, MResult};
use common::io::Buffer;

/// Строковый тип. Представляет собой UTF-8 строку с префиксом длины строки в виде [VarInt](types::varint::MinecraftVarInt)
pub struct MinecraftString(pub String);

#[async_trait]
impl MinecraftType for MinecraftString {
    async fn parse_from(io: &mut (impl Buffer + Send)) -> MResult<Self> {
        let len = MinecraftVarInt::parse_from(io).await?.0;
        let mut string_buf = Vec::new();
        for _ in 0..len {
            string_buf.push(MinecraftUnsignedByte::parse_from(io).await?.0)
        }
        let value = String::from_utf8(string_buf).map_err(|err| {
            MError::TypeValidationError(format!("Failed to parse string: {:?}", err))
        })?;

        Ok(Self(value))
    }

    async fn parse_to(&self) -> MResult<Vec<u8>> {
        let bytes = self.0.as_bytes();
        let len = MinecraftVarInt(bytes.len() as i32);
        let mut result = Vec::new();
        result.extend_from_slice(&len.parse_to().await?);
        result.extend_from_slice(bytes);
        Ok(result)
    }
}
