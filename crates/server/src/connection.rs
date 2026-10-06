use std::io::{Error, ErrorKind};
use std::time::Duration;

use mclib::PacketFrame;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::ConnectionError;
use crate::config::StatusConfig;
mod handshaking;
mod status;

const READ_TIMEOUT: Duration = Duration::from_secs(30);

/// Serves one TCP connection: a handshake followed by the status exchange.
#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
    status: StatusConfig,
}

impl Connection {
    /// Prepares a connection handler for an already accepted stream.
    #[must_use]
    pub const fn new(stream: TcpStream, status: StatusConfig) -> Self {
        Self { stream, status }
    }

    /// Drives the connection from handshake to ping and closes it.
    ///
    /// # Errors
    ///
    /// Returns [`ConnectionError`] on any I/O or protocol violation; every
    /// error is fatal for the connection.
    pub async fn run(&mut self) -> Result<(), ConnectionError> {
        match handshaking::run(self).await? {
            handshaking::Outcome::Status => match status::run(self).await? {
                status::Outcome::StatusAndPing | status::Outcome::PingOnly => {}
            },
            // Login and Transfer remain unsupported until the Login phase exists.
            handshaking::Outcome::Login => {
                return Err(ConnectionError::UnsupportedIntent("login"));
            }
            handshaking::Outcome::Transfer => {
                return Err(ConnectionError::UnsupportedIntent("transfer"));
            }
        }
        self.stream.shutdown().await?;
        Ok(())
    }

    async fn read_frame(&mut self) -> Result<PacketFrame, ConnectionError> {
        // Timeout is fatal: never restart a cancelled partial frame read.
        Ok(timeout(READ_TIMEOUT, PacketFrame::read(&mut self.stream))
            .await
            .map_err(|_| Error::new(ErrorKind::TimedOut, "frame read timed out"))??)
    }

    async fn write_frame(&mut self, packet_id: i32, body: Vec<u8>) -> Result<(), ConnectionError> {
        PacketFrame::new(packet_id, body)
            .write(&mut self.stream)
            .await?;
        Ok(())
    }
}
