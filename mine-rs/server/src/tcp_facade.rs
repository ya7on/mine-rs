use crate::io::tcp::TcpInput;
use crate::io::tcp_facade::TcpFacadeOutput;
use common::error::MResult;
use common::tracing::{debug, warn};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub enum Messages {
    RegisterNewSession { uuid: Uuid, socket: TcpInput },
    UnregisterSession { uuid: Uuid },
    SendPacket { uuid: Uuid, bytes: Vec<u8> },
}

pub struct TcpFacade {
    input: TcpFacadeOutput,
    outputs: HashMap<Uuid, TcpInput>,
}

impl TcpFacade {
    pub fn new(input: TcpFacadeOutput) -> Self {
        Self {
            input,
            outputs: HashMap::new(),
        }
    }

    pub async fn run(&mut self) -> MResult<()> {
        while let Some(message) = self.input.next_message().await {
            debug!("Received new message: {:?}", message);
            match message {
                Messages::RegisterNewSession { uuid, socket } => {
                    self.outputs.insert(uuid, socket);
                    debug!("Registered new write socket: {}", uuid);
                }
                Messages::UnregisterSession { uuid } => {
                    self.outputs.remove(&uuid);
                }
                Messages::SendPacket { uuid, bytes } => {
                    if let Some(socket) = self.outputs.get_mut(&uuid) {
                        socket.send_packet(bytes).await?;
                    } else {
                        warn!("Received message for unknown socket: {}", uuid);
                    }
                }
            }
        }
        Ok(())
    }
}
