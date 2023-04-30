use crate::io::reader::IOReader;
use crate::io::tcp::TcpInput;
use crate::io::writer::IOWriter;
use crate::tcp_facade;
use common::error::{MError, MResult};
use common::tracing::error;
use tokio::sync::mpsc::{Receiver, Sender};
use uuid::Uuid;

/// API для пишущего в TCP потока, для получения сообщений
pub type TcpFacadeOutput = IOReader<Receiver<tcp_facade::Messages>>;

impl TcpFacadeOutput {
    /// Получает следующее сообщение из очереди
    pub async fn next_message(&mut self) -> Option<tcp_facade::Messages> {
        self.output.recv().await
    }
}

/// API для общения с потоком, который занимается записью в TCP сокет
pub type TcpFacadeInput = IOWriter<Sender<tcp_facade::Messages>>;

impl TcpFacadeInput {
    /// Сообщение на регистрацию новой TCP сессии
    pub async fn new_session(&self, uuid: Uuid, socket: TcpInput) -> MResult<()> {
        self.input
            .send(tcp_facade::Messages::RegisterNewSession { uuid, socket })
            .await
            .map_err(|err| {
                MError::MPSCError(format!(
                    "Cannot send new session registration message: {}",
                    err
                ))
            })
    }

    /// Сообщение на удаление сессии
    pub async fn remove_session(&self, uuid: Uuid) -> MResult<()> {
        self.input
            .send(tcp_facade::Messages::UnregisterSession { uuid })
            .await
            .map_err(|err| MError::MPSCError(format!("Cannot unregister TCP session: {}", err)))
    }

    /// Сообщение на отправку сообщения сессии с указанным ID
    pub async fn send_to(&self, uuid: Uuid, bytes: Vec<u8>) -> MResult<()> {
        self.input
            .send(tcp_facade::Messages::SendPacket { uuid, bytes })
            .await
            .map_err(|err| MError::MPSCError(format!("Cannot send packet to tcp facade: {}", err)))
    }
}
