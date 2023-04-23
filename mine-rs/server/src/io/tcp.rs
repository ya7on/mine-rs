use crate::io::reader::IOReader;
use crate::io::writer::IOWriter;
use async_trait::async_trait;
use common::error::{MError, MResult};
use common::io::Buffer;
use common::tracing::error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use types::{MinecraftType, MinecraftVarInt};

pub type TcpOutput = IOReader<OwnedReadHalf>;

impl TcpOutput {
    pub async fn next_packet(&mut self) -> MResult<(MinecraftVarInt, MinecraftVarInt, Vec<u8>)> {
        let len = MinecraftVarInt::parse_from(self).await?;
        let mut packet = self.pull_packet(len.0 as usize).await?;
        let packet_id = MinecraftVarInt::parse_from(&mut packet).await?;

        Ok((len, packet_id, packet))
    }

    pub async fn pull_packet(&mut self, length: usize) -> MResult<Vec<u8>> {
        let mut result = Vec::new();
        for _ in 0..length {
            result.push(self.next_byte().await?);
        }
        Ok(result)
    }

    pub async fn next_byte(&mut self) -> MResult<u8> {
        self.output.read_u8().await.map_err(|err| MError::from(err))
    }
}

#[async_trait]
impl Buffer for TcpOutput {
    async fn next_byte(&mut self) -> MResult<u8> {
        self.next_byte().await
    }
}

pub type TcpInput = IOWriter<OwnedWriteHalf>;

impl TcpInput {
    pub async fn send_packet(&mut self, data: Vec<u8>) -> MResult<()> {
        self.input
            .write_all(&data)
            .await
            .map_err(|err| MError::from(err))
    }
}
