use crate::io::reader::IOReader;
use crate::io::writer::IOWriter;
use async_trait::async_trait;
use common::error::{MError, MResult};
use common::io::Buffer;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use types::{MinecraftType, MinecraftVarInt};

/// Интерфейс обертка для работы с читающим TCP сокетом
pub type TcpOutput = IOReader<OwnedReadHalf>;

impl TcpOutput {
    /// Получение следующего пакета из сокета
    /// Сначала побайтово получается длина пакета в виде [VarInt](types::MinecraftVarInt)
    /// потом исходя из длины пакета кешируется вся остальная часть пакета, сохраняется в `Vec<u8>`
    /// Из кешированного буфера читается ID пакета и функция возвращает результат в виде кортежа:
    /// (
    /// длина пакета в виде [VarInt](types::MinecraftVarInt),
    /// ID пакета в виде [VarInt](types::MinecraftVarInt),
    /// остальная часть пакета в виде `Vec<u8>`
    /// )
    pub async fn next_packet(&mut self) -> MResult<(MinecraftVarInt, MinecraftVarInt, Vec<u8>)> {
        let len = MinecraftVarInt::parse_from(self).await?;
        let mut packet = self.pull_packet(len.0 as usize).await?;
        let packet_id = MinecraftVarInt::parse_from(&mut packet).await?;

        Ok((len, packet_id, packet))
    }

    /// Вычитывает `length` байт из TCP сокета
    pub async fn pull_packet(&mut self, length: usize) -> MResult<Vec<u8>> {
        let mut result = Vec::new();
        for _ in 0..length {
            result.push(self.next_byte().await?);
        }
        Ok(result)
    }

    /// Читает следующий байт из сокета
    pub async fn next_byte(&mut self) -> MResult<u8> {
        self.output.read_u8().await.map_err(MError::from)
    }
}

#[async_trait]
impl Buffer for TcpOutput {
    async fn next_byte(&mut self) -> MResult<u8> {
        self.next_byte().await
    }
}

/// Интерфейс обертка для работы с пишущим TCP сокетом
pub type TcpInput = IOWriter<OwnedWriteHalf>;

impl TcpInput {
    /// Отправляет на TCP сокет пакет, переданный в виде вектора байт
    pub async fn send_packet(&mut self, data: Vec<u8>) -> MResult<()> {
        self.input.write_all(&data).await.map_err(MError::from)
    }
}
