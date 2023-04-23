use crate::error::MResult;
use async_trait::async_trait;

#[async_trait]
pub trait Buffer {
    async fn next_byte(&mut self) -> MResult<u8>;
}

#[async_trait]
impl Buffer for Vec<u8> {
    async fn next_byte(&mut self) -> MResult<u8> {
        Ok(self.remove(0))
    }
}
